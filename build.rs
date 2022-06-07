fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile(
            &[
                "proto/fdl/users/v1/users.proto",
                "proto/fdl/economy/v1/economy.proto",
            ],
            &["proto"],
        )
        .unwrap();

    Ok(())
}
