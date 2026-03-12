# grpc-mouse

A minimal gRPC demo in Rust using [Tonic](https://github.com/hyperium/tonic)

## Requirements

- Rust (v1.85+)
- Protobuf compiler (`protoc`)

## Running
```bash
# Terminal 1 - start the server
cargo run --bin server

# Terminal 2 - run the client
cargo run --bin client
```

## Structure
```
grpc-mouse/
├── build.rs            # Compiles mouse.proto into Rust types at build time
├── proto/
│   └── mouse.proto     # The service contract
└── src/
    ├── server.rs       # gRPC server implementation
    └── client.rs       # gRPC client implementation
```
