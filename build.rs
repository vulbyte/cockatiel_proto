use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_file = manifest_dir.join("cockatiel_protobuf.proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    // Use the vendored protoc binary so consumers do NOT need a system
    // `protoc` install (the stock prost_build::compile_protos shells out to a
    // `protoc` on PATH and fails the build when it's missing).
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc");
    let mut cfg = prost_build::Config::new();
    cfg.protoc_executable(protoc);
    cfg.compile_protos(
        &[proto_file.to_str().unwrap().to_string()],
        &[manifest_dir.to_str().unwrap().to_string()],
    )
    .unwrap();
}
