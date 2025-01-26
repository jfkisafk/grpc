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

# Copy the built binary
COPY --from=builder /app/bin/server .

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./server"]
