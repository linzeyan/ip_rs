FROM --platform=$TARGETPLATFORM rust:1.81 AS builder

WORKDIR /app

COPY . ./

RUN make build



FROM --platform=$TARGETPLATFORM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ip_rs /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/ip_rs" ]