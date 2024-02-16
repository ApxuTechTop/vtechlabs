FROM rust:buster AS base
WORKDIR /code
# RUN cargo init
# COPY Cargo.toml /code/Cargo.toml
# RUN cargo fetch
COPY . /code
EXPOSE 7878
RUN cargo build --release
FROM dokken/ubuntu-22.04
WORKDIR /app
# RUN apk update && apk add --no-cache musl-dev
COPY --from=base /code/target/release/vtechlabs /app/vtechlabs
COPY files/hello.html /app/files/hello.html
CMD /app/vtechlabs
