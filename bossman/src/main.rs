mod bossman;
mod consts;
mod error;
mod k8s;

use bossman::job::{
    self, status, GetListRequest, GetListResponse, GetRequest, GetResponse, GetStatusResponse,
    PerformRequest, PerformResponse,
};
use bossman::job_service_server::{JobService, JobServiceServer};
use bossman::Job;
use k8s_openapi::api::batch::v1::Job as KubeJob;
use std::convert::TryFrom;
use thiserror::Error;
use uuid::Uuid;

use tonic::{transport::Server, Request, Response, Status};

type TonicResponse<T> = Result<Response<T>, Status>;

#[derive(Debug, Default)]
pub struct JobServer {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing the required field in your request: {0})")]
    RequiredRequestFieldMissing(&'static str),
    #[error("unable to create job in the kubernetes cluster: {0}")]
    KubeCreateError(kube::Error),
    #[error("unable to get job from the kubernetes cluster: {0}")]
    KubeGetError(kube::Error),
    #[error(transparent)]
    KubeJobConversionError(bossman::FromError),
}

impl From<Error> for Status {
    fn from(error: Error) -> Self {
        match error {
            e @ Error::RequiredRequestFieldMissing(_) => Status::invalid_argument(e.to_string()),
            e @ Error::KubeCreateError(_) => Status::unknown(e.to_string()),
            e @ Error::KubeGetError(_) => Status::unknown(e.to_string()),
            e @ Error::KubeJobConversionError(_) => Status::not_found(e.to_string()),
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
            status: Some(job::Status {
                status: Some(status::Status::Waiting(status::Waiting {})),
            }),
            options: request.options,
        };

        k8s::create_job(&job)
            .await
            .map_err(Error::KubeCreateError)?;

        let reply = PerformResponse { job: Some(job) };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> TonicResponse<GetResponse> {
        let kube_job: KubeJob = k8s::get_job(&request.into_inner().id)
            .await
            .map_err(Error::KubeGetError)?;

        let job = bossman::Job::try_from(&kube_job).map_err(Error::KubeJobConversionError)?;
        let reply = GetResponse { job: Some(job) };

        Ok(Response::new(reply))
    }

    async fn get_list(&self, request: Request<GetListRequest>) -> TonicResponse<GetListResponse> {
        let kube_jobs = k8s::get_jobs_by_name(&request.into_inner().name)
            .await
            .map_err(Error::KubeGetError)?;

        let jobs = kube_jobs
            .iter()
            .map(bossman::Job::try_from)
            .filter_map(Result::ok)
            .collect();

        let reply = GetListResponse { jobs };

        Ok(Response::new(reply))
    }

    async fn get_status(&self, request: Request<GetRequest>) -> TonicResponse<GetStatusResponse> {
        let kube_job: KubeJob = k8s::get_job(&request.into_inner().id)
            .await
            .map_err(Error::KubeGetError)?;

        let job = bossman::Job::try_from(&kube_job).map_err(Error::KubeJobConversionError)?;

        let reply = GetStatusResponse { status: job.status };

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
