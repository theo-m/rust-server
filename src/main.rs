use tonic::transport::Server;

use pb::user_service_server::UserService;
use pb::user_service_server::UserServiceServer;
use user::UserServiceImpl;

mod user;

mod pb { tonic::include_proto!("app"); }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = user::NewType(user::UserServiceImpl::default());

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}


// use tonic::{Request,Response,Status};
// #[tonic::async_trait]
// impl pb::user_service_server::UserService for UserServiceImpl {
//
//     async fn sync_contacts(
//         &self,
//         request: Request<pb::SyncContactsInput>,
//     ) -> Result<Response<pb::SyncContactsOutput>, Status> {
//         unimplemented!()
//     }
//
//     async fn onboarding(
//         &self,
//         request: Request<pb::OnboardingInput>,
//     ) -> Result<Response<pb::MeUser>, Status> {
//         unimplemented!()
//     }
//
//     async fn me(
//         &self,
//         request: Request<pb::Empty>,
//     ) -> Result<Response<pb::MeUser>, Status> {
//         unimplemented!()
//     }
//
//     async fn get(
//         &self,
//         request: Request<pb::UserGetInput>,
//     ) -> Result<Response<pb::User>, Status> {
//         unimplemented!()
//     }
// }