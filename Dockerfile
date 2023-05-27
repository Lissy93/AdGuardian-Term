# syntax=docker/dockerfile:1.2

FROM --platform=$BUILDPLATFORM rust:1.69.0-alpine AS builder

# Adding necessary packages
RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev

# Set working directory in container; make directory if not exists
RUN mkdir -p /usr/src/adguardian
WORKDIR /usr/src/adguardian

# Copy all files from local computer to container
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

# Build release application
RUN cargo build --release


FROM scratch
COPY --from=builder /usr/src/adguardian/target/release/adguardian /
ENTRYPOINT ["/adguardian"]
