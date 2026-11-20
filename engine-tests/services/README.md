Throwaway test: one Vercel project, two services (stub web + engine container built FROM a prebuilt image).
1. Build and push the engine image once: from engine-tests/container (with data/acs.parquet), `vercel vcr build docker . ldh-engine:test --push` (in the test project). Record the image ref.
2. Set ENGINE_IMAGE in engine/Dockerfile's FROM line to that ref.
3. vercel.json defines services `web` (Node) and `engine` (container); rewrites: /api/q/* -> engine, /* -> web. Add the binding that injects the engine URL into web (see docs/services/bindings; name it ENGINE_URL).
4. Deploy with `--target preview` (do a throwaway first deploy; first deploys of a new project went to production).
5. Measure: engine build time (FROM pull) on deploy 1; deploy 2 changing only web/ (does engine rebuild? time); web->engine internal call latency via /timing (cold and warm) vs. direct /api/q.
6. Delete the project and VCR images.
