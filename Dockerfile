# flexfetch — static musl build into a scratch image (~1 MB).
#
# The minimal build (--no-default-features) drops the TUI/live dashboard (not
# useful in a container) and image logos, keeping the binary pure-Rust and fully
# static. Usage:
#   docker build -t flexfetch .
#   docker run --rm -it flexfetch
#   docker run --rm -it --pid=host --network=host flexfetch --minimal

# ---- builder stage: compile a static musl binary ----
FROM rust:1-alpine AS builder
# musl-dev provides the static linker pieces; build the release config.
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY . .
RUN cargo build --release --locked --no-default-features --package flexfetch-cli

# ---- runtime stage: scratch (the binary is fully static) ----
FROM scratch
COPY --from=builder /build/target/release/flexfetch /flexfetch
ENTRYPOINT ["/flexfetch"]
