# syntax=docker/dockerfile:1.2

# Build application - Copy assets, install deps and compile binary
FROM --platform=$BUILDPLATFORM rust:1.69.0-alpine AS builder
RUN apk add --no-cache pkgconfig openssl openssl-dev musl-dev
WORKDIR /usr/src/adguardian
COPY . .
RUN cargo build --release

# Run application - Using lightweight base, execute the binary
FROM scratch
COPY --from=builder /usr/src/adguardian/target/release/adguardian /
ENTRYPOINT ["/adguardian"]
