# gRPC Calculator Apps

Sample GRPC Calculator apps in multiple languages. This is practice project for [gRPC](https://grpc.io/) and [Protocol Buffers](https://developers.google.com/protocol-buffers).

## Languages

- [Go](https://go.dev/)

## Getting Started

### Prerequisites

- Docker is necessary to build and run the apps.
- gRPC client like [grpcurl](https://github.com/fullstorydev/grpcurl) or [grpcui](https://github.com/fullstorydev/grpcui) is necessary to test the apps.

### Build

```bash
docker build -t grpc/calculator .
```

### Run

```bash
docker run -p 8080:8080 grpc/calculator
```

### Test

```bash
grpcurl -plaintext -d '{"numbers": [5, 10, 15, -5]}' localhost:7777 calculator.Calculator.Sum
```
