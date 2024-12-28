use tonic_build;

fn main() {
    tonic_build::configure()
        .build_server(true) // Generates server code
        .build_client(true) // Generates client code
        .out_dir("src/generated") // Where to place the generated code
        .compile_protos(&["proto/job_management.proto"], &["proto"])
        .unwrap();
}
