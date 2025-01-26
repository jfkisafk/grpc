# gRPC Calculator Apps

Sample GRPC Calculator apps in multiple languages. This is practice project for [gRPC](https://grpc.io/) and [Protocol Buffers](https://developers.google.com/protocol-buffers).

## Languages

- [Go](https://go.dev/)
- [Rust](https://www.rust-lang.org/)

## Getting Started

### Prerequisites

- Docker is necessary to build and run the apps.
- gRPC client like [grpcurl](https://github.com/fullstorydev/grpcurl) or [grpcui](https://github.com/fullstorydev/grpcui) is necessary to test the apps.

### Run

```bash
docker compose up -d --build
```

### Test (Go)

```bash
grpcurl -plaintext -d '{"numbers": [5, 10, 15, -5]}' localhost:7777 calculator.Calculator.Sum
```

### Test (Rust)

```bash
grpcurl -plaintext -d '{"numbers": [5, 10, 15, -5]}' localhost:7878 calculator.Calculator.Sum
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
