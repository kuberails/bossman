fn main() {
    tonic_build::compile_protos("protos/bossman.proto").unwrap();
}
