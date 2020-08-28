use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path: &Path = "protos/bossman.proto".as_ref();

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
