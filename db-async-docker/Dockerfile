# Build stage
FROM rust:1.70-buster as builder

WORKDIR /app

# accept the build argument
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

# copy everything in dir into the docker image
COPY . . 

# build from inside the docker image
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

# output from build stage goes into /release/
# REMEMBER to change to name of proj/package in .toml
COPY --from=builder /app/target/release/db-async-docker .

# the command to run the rust application
CMD ["./db-async-docker"]