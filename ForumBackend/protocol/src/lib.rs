// ==========================================
// 阶段一：核心业务服务协议导出 (Admin, BManager, Image)
// ==========================================
pub mod admin {
    tonic::include_proto!("newblog.admin");
}

pub mod bmanager {
    tonic::include_proto!("newblog.bmanager");
}

pub mod image {
    tonic::include_proto!("newblog.image");
}

// ==========================================
// 阶段二：支撑业务服务协议导出 (SDK, Static, User)
// ==========================================
pub mod sdkws {
    tonic::include_proto!("newblog.sdkws");
}

// `static` is a keyword in Rust, so we use `static_svc`
pub mod static_svc {
    tonic::include_proto!("newblog.r#static");
}

pub mod user {
    tonic::include_proto!("newblog.user");
}
