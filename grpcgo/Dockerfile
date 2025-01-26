# Use alpine-based golang image for smaller size
FROM golang:1-alpine AS builder

WORKDIR /app

# Copy go mod and sum files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the application
RUN go build -o bin/server server/main.go

# Final stage
FROM alpine:latest

# Create a non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the built binary
COPY --from=builder /app/bin/server .

# Change ownership of the binary
RUN chown appuser:appgroup ./server

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./server"]
