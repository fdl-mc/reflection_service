FROM rust:latest
WORKDIR /app
COPY . .
RUN apt update && apt install -y cmake
RUN cargo install --path .
CMD ["reflection_service"]
