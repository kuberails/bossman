mod db;

use bossman::job::{
    self, GetListRequest, GetListResponse, GetRequest, GetResponse, GetStatusResponse,
    PerformRequest, PerformResponse,
};
use bossman::job_service_server::{JobService, JobServiceServer};
use bossman::Job;
use thiserror::Error;
use uuid::Uuid;

use tonic::{transport::Server, Request, Response, Status};

type TonicResponse<T> = Result<Response<T>, Status>;

#[derive(Debug, Default)]
pub struct JobServer {}

pub mod bossman {
    tonic::include_proto!("bossman.protobuf.v1alpha1");
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing the required field in your request: {0})")]
    RequiredRequestFieldMissing(&'static str),
    #[error(transparent)]
    DbError(#[from] db::Error),
}

impl From<Error> for Status {
    fn from(error: Error) -> Self {
        match error {
            e @ Error::RequiredRequestFieldMissing(_) => Status::invalid_argument(e.to_string()),
            e @ Error::DbError(db::Error::UnableToFindJob(_)) => Status::not_found(e.to_string()),
            e @ Error::DbError(db::Error::UnableToFindJobList(_)) => {
                Status::not_found(e.to_string())
            }
            e => Status::unknown(e.to_string()),
        }
    }
}

#[tonic::async_trait]
impl JobService for JobServer {
    async fn perform(&self, request: Request<PerformRequest>) -> TonicResponse<PerformResponse> {
        let request = request.into_inner();

        let job = Job {
            id: Uuid::new_v4().to_string(),
            docker_image_name: request
                .docker_image_name
                .ok_or(Error::RequiredRequestFieldMissing("docker_image_name"))?,
            name: request
                .name
                .ok_or(Error::RequiredRequestFieldMissing("name"))?,
            status: job::Status::Waiting.into(),
            options: request.options,
        };

        db::save_job(&job).await.map_err(Error::DbError)?;

        let reply = PerformResponse { job: Some(job) };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> TonicResponse<GetResponse> {
        let job = db::get_job(&request.into_inner().id)
            .await
            .map_err(Error::DbError)?;

        let reply = GetResponse { job: Some(job) };

        Ok(Response::new(reply))
    }

    async fn get_list(&self, request: Request<GetListRequest>) -> TonicResponse<GetListResponse> {
        let jobs = db::get_jobs_by_name(&request.into_inner().name)
            .await
            .map_err(Error::DbError)?;

        let reply = GetListResponse { jobs };

        Ok(Response::new(reply))
    }

    async fn get_status(&self, request: Request<GetRequest>) -> TonicResponse<GetStatusResponse> {
        let status = db::get_job(&request.into_inner().id)
            .await
            .map_err(Error::DbError)?
            .status;

        let reply = GetStatusResponse { status };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = "127.0.0.1:50051".parse()?;

    let job = JobServer::default();

    println!("Running bossman job server on: {:?}", socket);

    Server::builder()
        .add_service(JobServiceServer::new(job))
        .serve(socket)
        .await?;

    Ok(())
}
