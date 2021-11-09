use tonic::{Request, Response, Status};

mod pb {
    tonic::include_proto!("app");
}

pub struct NewType<T>(pub T);


#[tonic::async_trait]
impl pb::user_service_server::UserService for NewType<UserServiceImpl> {
    async fn sync_contacts(
        &self,
        request: Request<pb::SyncContactsInput>,
    ) -> Result<Response<pb::SyncContactsOutput>, Status> {
        unimplemented!()
    }

    async fn onboarding(
        &self,
        request: Request<pb::OnboardingInput>,
    ) -> Result<Response<pb::MeUser>, Status> {
        unimplemented!()
    }

    async fn me(
        &self,
        request: Request<pb::Empty>,
    ) -> Result<Response<pb::MeUser>, Status> {
        unimplemented!()
    }

    async fn get(
        &self,
        request: Request<pb::UserGetInput>,
    ) -> Result<Response<pb::User>, Status> {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct UserServiceImpl {}


#[tonic::async_trait]
impl pb::user_service_server::UserService for UserServiceImpl {
    async fn sync_contacts(
        &self,
        request: Request<pb::SyncContactsInput>,
    ) -> Result<Response<pb::SyncContactsOutput>, Status> {
        unimplemented!()
    }

    async fn onboarding(
        &self,
        request: Request<pb::OnboardingInput>,
    ) -> Result<Response<pb::MeUser>, Status> {
        unimplemented!()
    }

    async fn me(
        &self,
        request: Request<pb::Empty>,
    ) -> Result<Response<pb::MeUser>, Status> {
        unimplemented!()
    }

    async fn get(
        &self,
        request: Request<pb::UserGetInput>,
    ) -> Result<Response<pb::User>, Status> {
        unimplemented!()
    }
}