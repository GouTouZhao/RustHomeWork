// ==========================================
// 阶段一：构建脚本依赖导入
// ==========================================
use std::env;
use std::path::PathBuf;

// ==========================================
// 阶段二：gRPC 协议编译配置逻辑
// ==========================================
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
