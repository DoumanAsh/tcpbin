FROM quay.io/doumanash/rust-musl:latest AS build

WORKDIR /src

COPY . /src

RUN cargo build --release --features tokio,cli

FROM scratch
COPY --from=build /src/target/release/tcpbin /tcpbin
CMD ["/tcpbin"]
