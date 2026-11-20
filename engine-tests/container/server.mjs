// Container wrapper (Vercel Container Runtime or Cloud Run): same handler, data baked into the image at /app/data/acs.parquet
import { createServer } from 'node:http';
import { GET } from './api/query.mjs';
createServer(async (req, res) => {
  const r = await GET(new Request(`http://x${req.url}`));
  res.writeHead(r.status, Object.fromEntries(r.headers)); res.end(await r.text());
}).listen(process.env.PORT ?? 8080);
