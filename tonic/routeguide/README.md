
# routeguide (tonic + gRPC learning exercise)

A small Rust learning project implementing the classic gRPC RouteGuide demo using **tonic**.

It includes two binaries:

- `routeguide-server`: a gRPC server that serves route guide data (features, route recording, bidirectional chat)
- `routeguide-client`: a client that exercises all RPCs (unary, server-streaming, client-streaming, bidirectional-streaming)

## Run

### Start the server

```sh
cargo run --bin routeguide-server
```

### Run the client (in another terminal)

```sh
cargo run --bin routeguide-client
```

## Data

The server loads initial features from `route_guide_db.json` via `data.rs`.

## Notes

- Uses `tonic`, `tokio`, and `async-stream`
- Uses `prost` / `tonic-build` via build script to generate protobuf code from route_guide.proto
