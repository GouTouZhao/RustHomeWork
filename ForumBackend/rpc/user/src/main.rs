// ==========================================
// 阶段一：模块依赖与协议引入
// ==========================================
use common::config::load_config;
use protocol::user::user_server::{User, UserServer};
use tonic::{transport::Server, Request, Response, Status};

// ==========================================
// 阶段二：User 服务实现与接口逻辑
// ==========================================
#[derive(Default)]
pub struct UserService {}

#[tonic::async_trait]
impl User for UserService {
    async fn user_register(&self, _request: Request<protocol::user::UserRegisterReq>) -> Result<Response<protocol::user::UserRegisterResp>, Status> {
        todo!()
    }

    async fn user_login(&self, _request: Request<protocol::user::UserLoginReq>) -> Result<Response<protocol::user::UserLoginResp>, Status> {
        todo!()
    }

    async fn get_user_info(&self, _request: Request<protocol::user::GetUserInfoReq>) -> Result<Response<protocol::user::GetUserInfoResp>, Status> {
        todo!()
    }

    async fn update_user_info(&self, _request: Request<protocol::user::UpdateUserInfoReq>) -> Result<Response<protocol::user::UpdateUserInfoResp>, Status> {
        todo!()
    }

    async fn get_image_url(&self, _request: Request<protocol::user::GetImageUrlReq>) -> Result<Response<protocol::user::GetImageUrlResp>, Status> {
        todo!()
    }

    async fn get_user_photo_compre(&self, _request: Request<protocol::user::GetUserPhotoCompreReq>) -> Result<Response<protocol::user::GetUserPhotoCompreResp>, Status> {
        todo!()
    }

    async fn post_update_profile_photo(&self, _request: Request<protocol::user::PostUpdateProfilePhotoReq>) -> Result<Response<protocol::user::PostUpdateProfilePhotoResp>, Status> {
        todo!()
    }

    async fn get_captcha(&self, _request: Request<protocol::user::GetCaptchaReq>) -> Result<Response<protocol::user::GetCaptchaResp>, Status> {
        todo!()
    }

    async fn post_update_password(&self, _request: Request<protocol::user::PostUpdatePasswordReq>) -> Result<Response<protocol::user::PostUpdatePasswordResp>, Status> {
        todo!()
    }

    async fn send_email_code(&self, _request: Request<protocol::user::SendEmailCodeReq>) -> Result<Response<protocol::user::SendEmailCodeResp>, Status> {
        todo!()
    }
}

// ==========================================
// 阶段三：User 服务主程序配置与启动
// ==========================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = load_config()?;
    let addr = "[::]:8081".parse()?;
    let service = UserService::default();

    println!("User RPC server listening on {}", addr);

    Server::builder()
        .add_service(UserServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
