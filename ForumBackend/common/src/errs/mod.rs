// ==========================================
// 阶段一：自定义错误结构体定义
// ==========================================
use std::fmt;

#[derive(Debug, Clone)]
pub struct CodeError {
    pub code: i32,
    pub msg: String,
    pub detail: String,
}

impl CodeError {
    pub fn new(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            detail: String::new(),
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        let d = detail.into();
        if self.detail.is_empty() {
            self.detail = d;
        } else {
            self.detail = format!("{}; {}", self.detail, d);
        }
        self
    }
}

// ==========================================
// 阶段二：错误特质实现与转换
// ==========================================
impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.detail.is_empty() {
            write!(f, "{} {}", self.code, self.msg)
        } else {
            write!(f, "{} {} {}", self.code, self.msg, self.detail)
        }
    }
}

impl std::error::Error for CodeError {}

// Predefined errors
// ==========================================
// 阶段三：预定义错误常量与工厂函数
// ==========================================
pub const SERVER_INTERNAL_ERROR_CODE: i32 = 500;
pub const ARGS_ERROR_CODE: i32 = 1001;

pub fn err_internal_server() -> CodeError {
    CodeError::new(SERVER_INTERNAL_ERROR_CODE, "ServerInternalError")
}

pub fn err_args() -> CodeError {
    CodeError::new(ARGS_ERROR_CODE, "ArgError")
}

// Convert from anyhow to CodeError for grpc status
impl From<CodeError> for tonic::Status {
    fn from(e: CodeError) -> Self {
        let code = match e.code {
            1001 => tonic::Code::InvalidArgument,
            _ => tonic::Code::Internal,
        };
        tonic::Status::new(code, format!("{}", e))
    }
}
