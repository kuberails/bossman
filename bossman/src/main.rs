use bincode;
use bossman::job::{self, GetRequest, GetResponse, PerformRequest, PerformResponse};
use bossman::job_service_server::{JobService, JobServiceServer};
use bossman::Job;
use redis;
use redis::Commands;
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
}

impl From<Error> for Status {
    fn from(error: Error) -> Self {
        match error {
            e @ Error::RequiredRequestFieldMissing(_) => Status::invalid_argument(e.to_string()),
        }
    }
}

#[tonic::async_trait]
impl JobService for JobServer {
    async fn perform(&self, request: Request<PerformRequest>) -> TonicResponse<PerformResponse> {
        let request = request.into_inner();

        // connect to redis
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

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

        let encoded_job: Vec<u8> = bincode::serialize(&job).unwrap();

        let _: () = con.set(&job.id, encoded_job).unwrap();

        let reply = PerformResponse { job: Some(job) };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> TonicResponse<GetResponse> {
        let request = request.into_inner();

        // connect to redis
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        let encoded_job: Vec<u8> = con.get(&request.id).unwrap();

        let job = bincode::deserialize(&encoded_job).unwrap();

        let reply = GetResponse { job: Some(job) };

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
