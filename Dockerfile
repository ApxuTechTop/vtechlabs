FROM rust:buster AS base
WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code
EXPOSE 7878
CMD ["cargo", "run"]

