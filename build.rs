use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_file = manifest_dir.join("cockatiel_protobuf.proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    prost_build::compile_protos(
        &[proto_file.to_str().unwrap().to_string()],
        &[manifest_dir.to_str().unwrap().to_string()],
    )
    .unwrap();
}