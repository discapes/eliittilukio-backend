# needs to be alpine if its run on alpine
FROM rust:alpine AS builder
WORKDIR /app

RUN cargo init
COPY Cargo* ./
# update crates.io index
RUN cargo update --dry-run
RUN apk add musl-dev sqlite
# ^^^important
# compile dependencies
RUN cargo build --release


COPY src src
RUN sqlite3 database.db "$(cat src/schema.sql)"
# update mtime so cargo rebuilds
RUN touch src/main.rs 
RUN DATABASE_URL="sqlite://database.db" cargo build --release

FROM alpine
WORKDIR /app
EXPOSE 3000
COPY --from=builder /app/target/release/eliittilukio-backend ./
CMD ./eliittilukio-backend