// Throwaway cold-start test for the LDH engine in a container, data baked into the image. GET /api/query?q=<name>
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const bootStart = performance.now();
const instanceId = Math.random().toString(36).slice(2, 8);
let served = 0;
let conPromise;

const DATA = process.env.DATA_URL ?? join(process.cwd(), 'data', 'acs.parquet');
const QUERIES = JSON.parse(readFileSync(join(process.cwd(), 'api', '_queries.json'), 'utf8'));

async function connection() {
  if (!conPromise) {
    conPromise = (async () => {
      const t = performance.now();
      const { DuckDBInstance } = await import('@duckdb/node-api');
      const inst = await DuckDBInstance.create(':memory:', { threads: '2' });
      const con = await inst.connect();
      return { con, initMs: performance.now() - t };
    })();
  }
  return conPromise;
}

export async function GET(request) {
  const reqStart = performance.now();
  const name = new URL(request.url).searchParams.get('q') ?? 'enum, 1 group, national';
  const sql = QUERIES[name];
  if (!sql) return Response.json({ error: 'unknown query', names: Object.keys(QUERIES) }, { status: 400 });
  const cold = served === 0;
  served += 1;
  const { con, initMs } = await connection();
  const q0 = performance.now();
  const reader = await con.runAndReadAll(sql.replaceAll('__DATA__', DATA));
  const rows = reader.getRowObjectsJson();
  const queryMs = performance.now() - q0;
  let explain = null;
  if (new URL(request.url).searchParams.get('explain')) {
    const ex = await con.runAndReadAll('EXPLAIN ANALYZE ' + sql.replaceAll('__DATA__', DATA));
    explain = ex.getRows().map(r => r.map(String).join('\n')).join('\n'); // includes HTTPFS in/out bytes
  }
  return Response.json(
    {
      name, cold, instanceId, served,
      sinceModuleLoadMs: Math.round(reqStart - bootStart),
      duckdbInitMs: cold ? Math.round(initMs) : 0,
      queryMs: Math.round(queryMs),
      handlerMs: Math.round(performance.now() - reqStart),
      rows: rows.length,
      rssMB: Math.round(process.memoryUsage().rss / 1e6),
      region: process.env.VERCEL_REGION ?? null,
      explain,
    },
    { headers: { 'Cache-Control': 'no-store' } },
  );
}
