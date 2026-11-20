FROM node:22-slim
WORKDIR /app
COPY package.json ./
RUN npm install --omit=dev
COPY api ./api
COPY server.mjs ./
COPY data/acs.parquet ./data/acs.parquet
ENV PORT=8080
CMD ["node", "server.mjs"]
