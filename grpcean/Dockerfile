# Use alpine-based rust image for smaller size
FROM rust:1-alpine AS builder

RUN apk add --no-cache protobuf-dev musl-dev gcc build-base protoc

WORKDIR /app

# Copy the source code
COPY build.rs Cargo.toml Cargo.lock ./
COPY proto ./proto
COPY src ./src

# Build the application
RUN cargo build --release

# Final stage
FROM alpine:latest

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the built binary
COPY --from=builder /app/target/release/server .

# Change ownership of the binary
RUN chown appuser:appgroup .

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./server"]
