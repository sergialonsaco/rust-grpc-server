# rust-grpc-server

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/fee507753bbf46b58c408ebdcb3dfb70)](https://app.codacy.com/manual/sergi.alonsobadia/rust-grpc-server?utm_source=github.com&utm_medium=referral&utm_content=sergialonsaco/rust-grpc-server&utm_campaign=Badge_Grade_Dashboard)

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
