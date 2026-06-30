// ==========================================
// 阶段一：模块依赖与协议引入
// ==========================================
use common::config::load_config;
use protocol::static_svc::static_server::{Static, StaticServer};
use protocol::static_svc::*;
use tonic::{transport::Server, Request, Response, Status};

// ==========================================
// 阶段二：Static 服务实现与接口逻辑
// ==========================================
#[derive(Default)]
pub struct StaticService {}

#[tonic::async_trait]
impl Static for StaticService {
    async fn get_article_list(&self, _request: Request<GetArticleListReq>) -> Result<Response<GetArticleListResp>, Status> {
        todo!()
    }

    async fn get_article_details(&self, _request: Request<GetArticleDetailsReq>) -> Result<Response<GetArticleDetailsResp>, Status> {
        todo!()
    }

    async fn add_view_count(&self, _request: Request<AddViewCountReq>) -> Result<Response<AddViewCountResp>, Status> {
        todo!()
    }

    async fn get_comment_list(&self, _request: Request<GetCommentListReq>) -> Result<Response<GetCommentListResp>, Status> {
        todo!()
    }

    async fn get_comment_son_list(&self, _request: Request<GetCommentSonListReq>) -> Result<Response<GetCommentSonListResp>, Status> {
        todo!()
    }

    async fn get_last_blog_list(&self, _request: Request<GetLastBlogListReq>) -> Result<Response<GetLastBlogListResp>, Status> {
        todo!()
    }
}

// ==========================================
// 阶段三：Static 服务主程序配置与启动
// ==========================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = load_config()?;
    let addr = "[::]:8084".parse()?;
    let service = StaticService::default();

    println!("Static RPC server listening on {}", addr);

    Server::builder()
        .add_service(StaticServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
