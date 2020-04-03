# rust-grpc-server

Grpc server made with Rust.

## Usage

To run the server in your local machine, while you develop, open a terminal in the root of the project and execute:

```shell
cargo watch +x run
```

If you want then to test the server, you can curl it usign **grpcurl** like:

```shell
grpcurl \
 -plaintext \
 -import-path ./proto \
 -proto grpc_server.proto \
 -d '{"hello":"Rob"}' \
 -H 'authorization: Bearer myjwttoken' \
 localhost:50051 \
 grpc_server.Server01/SayHi
```
