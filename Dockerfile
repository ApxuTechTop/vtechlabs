FROM rust:buster AS base
WORKDIR /code
COPY . /code
RUN cargo build --release
FROM dokken/ubuntu-22.04
WORKDIR /app
RUN useradd -ms /bin/bash test_user
USER test_user
COPY --from=base /code/target/release/vtechlabs /app/vtechlabs
COPY files/hello.html /app/files/hello.html
EXPOSE 80
CMD /app/vtechlabs
