FROM rust:1.24.0

WORKDIR /usr/src/try_gluon
COPY . .

RUN yarn install
RUN webpack
RUN cargo update -p https://github.com/gluon-lang/gluon
RUN cargo install

CMD ["try_gluon"]
