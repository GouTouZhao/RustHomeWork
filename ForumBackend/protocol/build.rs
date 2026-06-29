use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &[
        "proto/admin.proto",
        "proto/bmanager.proto",
        "proto/image.proto",
        "proto/sdkws.proto",
        "proto/static.proto",
        "proto/user.proto",
    ];

    let dirs = &["proto"];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(proto_files, dirs)?;

    Ok(())
}
