extern crate protoc_grpcio;

fn main() {
  protoc_grpcio::compile_grpc_protos(
      &["./audros-schema/audros.proto"],
      &["./audros-schema"],
      "./src/grpc"
  ).expect("failed to compile gRPC definitions");
}
