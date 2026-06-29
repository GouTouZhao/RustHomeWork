use common::config::load_config;
use protocol::bmanager::b_manager_server::{BManager, BManagerServer};
use protocol::bmanager::*;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct BManagerService {}

#[tonic::async_trait]
impl BManager for BManagerService {
    async fn push_forum_comment_new(&self, _request: Request<PushForumCommentNewReq>) -> Result<Response<PushForumCommentNewResp>, Status> {
        Ok(Response::new(PushForumCommentNewResp {
            comment_id: "1".to_string(),
        }))
    }

    async fn delete_comment(&self, _request: Request<DeleteCommentReq>) -> Result<Response<DeleteCommentResp>, Status> {
        Ok(Response::new(DeleteCommentResp {}))
    }

    async fn push_forum_new(&self, _request: Request<PushForumNewReq>) -> Result<Response<PushForumNewResp>, Status> {
        Ok(Response::new(PushForumNewResp {
            article_id: "1".to_string(),
        }))
    }

    async fn delete_forum(&self, _request: Request<DeleteForumReq>) -> Result<Response<DeleteForumResp>, Status> {
        Ok(Response::new(DeleteForumResp {}))
    }

    async fn update_forum(&self, _request: Request<UpdateForumReq>) -> Result<Response<UpdateForumResp>, Status> {
        Ok(Response::new(UpdateForumResp {}))
    }

    async fn get_oss_key(&self, _request: Request<GetOssKeyReq>) -> Result<Response<GetOssKeyResp>, Status> {
        Ok(Response::new(GetOssKeyResp {
            host: String::new(),
            key: String::new(),
            policy: String::new(),
            oss_access_key_id: String::new(),
            signature: String::new(),
            expire: 0,
            security_token: String::new(),
        }))
    }

    async fn get_image_url(&self, _request: Request<GetImageUrlReq>) -> Result<Response<GetImageUrlResp>, Status> {
        Ok(Response::new(GetImageUrlResp {
            url: String::new(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let addr = "[::]:8082".parse()?;
    let service = BManagerService::default();

    println!("BManager RPC server listening on {}", addr);

    Server::builder()
        .add_service(BManagerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
