// node reencode.mjs <processed_2022.parquet> <out acs.parquet>  — narrows int types, zstd, sorted (same as build.py)
import { DuckDBInstance } from '@duckdb/node-api';
const [src, out] = process.argv.slice(2);
const con = await (await DuckDBInstance.create(':memory:')).connect();
const cols = (await con.runAndReadAll(`SELECT * FROM '${src}' LIMIT 0`)).columnNames();
const st = (await con.runAndReadAll(`SELECT ${cols.map(x => `min("${x}")::BIGINT, max("${x}")::BIGINT`).join(',')} FROM '${src}'`)).getRows()[0].map(Number);
const ex = cols.map((x, i) => { const lo = st[2*i], hi = st[2*i+1]; const t = lo >= -128 && hi <= 127 ? 'TINYINT' : lo >= -32768 && hi <= 32767 ? 'SMALLINT' : 'INTEGER'; return `CAST("${x}" AS ${t}) AS ${x.toLowerCase()}`; });
await con.run(`COPY (SELECT ${ex.join(',')} FROM '${src}' ORDER BY statefip, county_id, race_recoded) TO '${out}' (FORMAT parquet, COMPRESSION zstd, ROW_GROUP_SIZE 122880)`);
console.log('wrote', out);
