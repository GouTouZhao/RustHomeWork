// ==========================================
// 阶段一：模块依赖与协议引入
// ==========================================
use common::config::load_config;
use protocol::admin::admin_server::{Admin, AdminServer};
use protocol::admin::*;
use tonic::{transport::Server, Request, Response, Status};

// ==========================================
// 阶段二：Admin 服务实现与接口逻辑
// ==========================================
#[derive(Default)]
pub struct AdminService {}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn push_blog_new(&self, _request: Request<PushBlogNewReq>) -> Result<Response<PushBlogNewResp>, Status> {
        Ok(Response::new(PushBlogNewResp {
            article_id: "1".to_string(),
        }))
    }

    async fn delete_blog(&self, _request: Request<DeleteBlogReq>) -> Result<Response<DeleteBlogResp>, Status> {
        Ok(Response::new(DeleteBlogResp {}))
    }

    async fn post_blog_change(&self, _request: Request<PostBlogChangeReq>) -> Result<Response<PostBlogChangeResp>, Status> {
        Ok(Response::new(PostBlogChangeResp {}))
    }
}

// ==========================================
// 阶段三：Admin 服务主程序配置与启动
// ==========================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let addr = "[::]:8085".parse()?;
    let service = AdminService::default();

    println!("Admin RPC server listening on {}", addr);

    Server::builder()
        .add_service(AdminServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
