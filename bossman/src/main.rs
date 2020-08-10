use bossman::job::{self, GetRequest, GetResponse, PerformRequest, PerformResponse};
use bossman::job_service_server::{JobService, JobServiceServer};
use bossman::Job;
use uuid::Uuid;

use tonic::{transport::Server, Request, Response, Status};

type TonicResponse<T> = Result<Response<T>, Status>;

#[derive(Debug, Default)]
pub struct JobServer {}

pub mod bossman {
    tonic::include_proto!("bossman.protobuf.v1alpha1");
}

#[tonic::async_trait]
impl JobService for JobServer {
    async fn perform(&self, request: Request<PerformRequest>) -> TonicResponse<PerformResponse> {
        let reply = PerformResponse {
            id: Uuid::new_v4().to_string(),
            status: job::Status::Waiting.into(),
        };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> TonicResponse<GetResponse> {
        let reply = GetResponse {
            job: Some(Job {
                name: "test".to_string(),
                id: "uuid".to_string(),
                ..Job::default()
            }),
        };

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
