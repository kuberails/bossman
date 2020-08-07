use prost_build;

fn main() {
    prost_build::compile_protos(&["src/protos/job.proto"], &["src/"]).unwrap();
}
