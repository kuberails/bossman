fn main() {
    std::env::set_var("OUT_DIR", "protos");
    prost_build::compile_protos(&["protos/bossman.proto"], &["protos"]).unwrap();
}
