use common::config::load_config;
use protocol::image::image_server::{Image, ImageServer};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct ImageService {}

#[tonic::async_trait]
impl Image for ImageService {
    async fn get_image(&self, _request: Request<protocol::image::GetImageReq>) -> Result<Response<protocol::image::GetImageResp>, Status> {
        todo!()
    }
    
    async fn get_avatar_thumbnail(&self, _request: Request<protocol::image::GetAvatarThumbnailReq>) -> Result<Response<protocol::image::GetAvatarThumbnailResp>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = load_config()?;
    let addr = "[::]:8083".parse()?;
    let service = ImageService::default();

    println!("Image RPC server listening on {}", addr);

    Server::builder()
        .add_service(ImageServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
