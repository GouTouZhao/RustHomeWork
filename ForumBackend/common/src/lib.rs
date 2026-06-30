// ==========================================
// 阶段一：通用配置与错误处理模块导出
// ==========================================
pub mod config;
pub mod errs;
// ==========================================
// 阶段二：服务发现与存储模块框架
// ==========================================
pub mod discovery {
    // will implement later
}
pub mod storage {
    pub mod database {
        pub mod mysql {
            // will implement sea-orm models
        }
    }
    pub mod redis {
        // will implement redis connection
    }
}
pub mod utils {
    // will implement utilities
}
