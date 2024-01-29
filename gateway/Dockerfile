# syntax=docker/dockerfile:1

# build stage

ARG RUST_VERSION=1
ARG APP_NAME=palboard-gateway
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=.cargo,target=.cargo \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/gateway
EOF

# runtime stage

FROM cm2network/steamcmd:latest AS final
LABEL org.opencontainers.image.source=https://github.com/shirok1/palboard
LABEL org.opencontainers.image.description="PalBoard gateway"
LABEL org.opencontainers.image.licenses=SSPL-1.0

COPY --from=build /bin/gateway /bin/

EXPOSE 8080

ENV RUST_LOG debug
ENV GATEWAY_ADDR 0.0.0.0:8080

CMD ["/bin/gateway"]