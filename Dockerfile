FROM rust:slim-buster as builder
WORKDIR ./backend
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bullseye-slim
RUN rm -rf /var/lib/apt/lists/* && apt-get update
COPY --from=builder /backend/target/release/backend ./backend
CMD ["./backend"]
EXPOSE 8080