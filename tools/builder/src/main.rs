fn main() {
    std::env::set_var("OUT_DIR", "protos");
    prost_build::Config::new()
        .type_attribute(".", "#[derive(Serialize, Deserialize)]")
        .compile_protos(&["protos/bossman.proto"], &["protos"])
        .unwrap();
}
