# syntax=docker/dockerfile:1

# build stage

ARG BUN_VERSION=1
FROM oven/bun:${BUN_VERSION}-slim AS build
WORKDIR /app

COPY package.json bun.lockb ./
RUN bun install

COPY . .
RUN bun nuxt build

# runtime stage

FROM oven/bun:${BUN_VERSION}-slim AS final
LABEL org.opencontainers.image.source=https://github.com/shirok1/palboard
LABEL org.opencontainers.image.description="PalBoard frontend"
LABEL org.opencontainers.image.licenses=SSPL-1.0

COPY --from=build /app/.output /app

EXPOSE 3000

CMD [ "bun", "/app/server/index.mjs" ]