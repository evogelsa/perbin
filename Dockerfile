FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev

COPY . /sources
WORKDIR /sources
RUN cargo build --release
RUN chown nobody:nogroup /sources/target/release/bin
RUN mkdir /data

FROM scratch
COPY --from=builder /sources/target/release/bin /pastebin
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /data /data

EXPOSE 8000
ENTRYPOINT ["/pastebin", "0.0.0.0:8000" "--max-paste-size 131072"]
