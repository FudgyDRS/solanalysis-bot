fn main() {
  tonic_build::configure()
    .build_server(true)
    .out_dir("src/")
    .compile(
      &[
        "../../proto/api_gateway.proto",
        "../../proto/solana_service.proto",
      ],
      &["../../proto"],
    )
    .unwrap();
}
