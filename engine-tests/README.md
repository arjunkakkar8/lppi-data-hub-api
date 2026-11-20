Throwaway engine hosting tests (delete after use).
1. Build data: `npm i` in either dir, then `node reencode.mjs ../../data/processed_2022.parquet acs.parquet` (needs git lfs pull of data/processed_2022.parquet).
2. container/: copy acs.parquet to container/data/acs.parquet; Vercel Container Runtime detects the root Dockerfile. Deploy with `vercel deploy --target preview`.
3. vercel-blob/: upload acs.parquet to a temporary public Blob store, set DATA_URL on the preview, deploy with `--target preview`. Add `&explain=1` to a query to get httpfs bytes read.
4. Run `./bench.sh <url>` (cold, 3 warm rounds), then a second cold start after idle.
