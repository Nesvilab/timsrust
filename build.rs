use std::fs;
use std::path::Path;

fn main() {
    prost_build::Config::new()
        .out_dir("generated") // Specifies where to put the generated Rust files
        .compile_protos(&["proto/spectra.proto"], &["proto/"]) // Adjust paths accordingly
        .unwrap();

    let from_path = Path::new("generated/com.rogerli.rs");
    let to_path = Path::new("src/spectra_proto.rs");
    if from_path.exists() {
        fs::rename(from_path, to_path).expect("Failed to rename generated Rust file");
    }
}
