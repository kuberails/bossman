fn main() {
    tonic_build::compile_protos("src/protos/bossman.proto").unwrap();
}
