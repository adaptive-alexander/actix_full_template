# Stage 1 - cache skeleton
FROM rust AS chef
WORKDIR /app
RUN apt update && apt install -yq cmake git && \
cargo install cargo-chef

# Stage 2 - use cached deps
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3 - build
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy app
COPY . .
# Build app
RUN cargo install --path .

# Stage 4 - deploy
# use google distroless as runtime image (non-root user)
FROM gcr.io/distroless/cc-debian11:nonroot
# set work dir in second image
WORKDIR /app

# CHANGE
# set app name
ARG APPNAME=template
ARG PORT=8080

# copy app from builder
COPY --from=builder --chown=nonroot:nonroot /usr/local/cargo/bin/back_end /app/template

# expose port
EXPOSE 8080

# start app
ENTRYPOINT ["/app/template"]
