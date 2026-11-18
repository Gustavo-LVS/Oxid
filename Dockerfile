# Imagem da API (oxid-server-api). Usada por deploy/prod/docker-compose.yml.
FROM rust:1-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p oxid-server-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /src/target/release/oxid-server-api /usr/local/bin/oxid-server-api
EXPOSE 8080
CMD ["oxid-server-api"]
