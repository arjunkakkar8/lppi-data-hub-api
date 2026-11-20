// Stub web service: /timing calls the engine over the internal binding and reports latency.
import { createServer } from 'node:http';
const ENGINE = process.env.ENGINE_URL;
createServer(async (req, res) => {
  const u = new URL(req.url, 'http://x');
  if (u.pathname === '/timing') {
    const q = u.searchParams.get('q') ?? 'enum, 1 group, national';
    const t = performance.now();
    const r = await fetch(`${ENGINE}/api/query?q=${encodeURIComponent(q)}`);
    const body = await r.json();
    res.writeHead(200, { 'content-type': 'application/json', 'cache-control': 'no-store' });
    return res.end(JSON.stringify({ internalCallMs: Math.round(performance.now() - t), engine: body, engineUrlSet: !!ENGINE }));
  }
  res.writeHead(200, { 'content-type': 'text/plain' }); res.end('stub web ok');
}).listen(process.env.PORT ?? 3000);
