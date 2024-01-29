# syntax=docker/dockerfile:1

# build stage

ARG BUN_VERSION=1.0
FROM oven/bun:${BUN_VERSION}-slim AS build
WORKDIR /app

COPY package.json bun.lockb ./
RUN bun install

COPY . .
RUN bun nuxt build

# runtime stage

FROM oven/bun:${BUN_VERSION}-slim AS final

COPY --from=build /app/.output /app

EXPOSE 3000

CMD [ "bun", "/app/server/index.mjs" ]