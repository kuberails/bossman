use bossman::job::{GetRequest, GetResponse};
use bossman::job_service_server::{JobService, JobServiceServer};
use bossman::Job;

use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct JobServer {}

pub mod bossman {
    tonic::include_proto!("bossman.protobuf.v1alpha1");
}

#[tonic::async_trait]
impl JobService for JobServer {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        println!("Request: {:#?}", request);

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
