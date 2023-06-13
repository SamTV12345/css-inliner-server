FROM alpine AS cache
RUN apk add -U --no-cache ca-certificates

FROM scratch as base
WORKDIR /app/

FROM base as amd64
COPY --from=cache /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY ./target/x86_64-unknown-linux-musl/release/css-inliner-server /app/css-inliner-server

FROM base as armv7
COPY --from=cache /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY ./target/armv7-unknown-linux-musleabihf/release/css-inliner-server /app/css-inliner-server

FROM base as arm64
COPY --from=cache /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY ./target/aarch64-unknown-linux-musl/release/css-inliner-server /app/css-inliner-server

FROM ${TARGETARCH}${TARGETVARIANT} as final


EXPOSE 8000
CMD ["./css-inliner-server"]