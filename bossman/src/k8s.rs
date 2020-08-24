mod kube_job;

use crate::bossman::Job as BossmanJob;
use crate::consts::{BOSSMAN_JOB_ID, BOSSMAN_JOB_NAME};
use k8s_openapi::api::batch::v1::Job as KubeJob;
use kube::{
    api::{Api, ListParams, PostParams},
    Client,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to find job for id: {0}")]
    UnableToFindJob(String),

    #[error("unable to find jobs for name: {0}")]
    UnableToFindJobList(String),

    #[error("unable to create job in the kubernetes cluster: {0}")]
    KubeCreateError(kube::Error),

    #[error("unable to get job from the kubernetes cluster: {0}")]
    KubeGetError(kube::Error),

    #[error("unknown kube error: {0}")]
    UnknownKubeError(#[from] kube::Error),
}

impl From<Error> for tonic::Status {
    fn from(error: Error) -> Self {
        match error {
            e @ Error::UnableToFindJob(_) => tonic::Status::not_found(e.to_string()),
            e @ Error::UnableToFindJobList(_) => tonic::Status::not_found(e.to_string()),
            error => tonic::Status::unknown(error.to_string()),
        }
    }
}

pub async fn get_job(id: &str) -> Result<KubeJob, Error> {
    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::all(client);

    let list_params = ListParams::default().labels(&format!("{}={}", BOSSMAN_JOB_ID, id));

    jobs.list(&list_params)
        .await?
        .items
        .get(0)
        .map(ToOwned::to_owned)
        .ok_or_else(|| Error::UnableToFindJob(id.to_string()))
}

pub async fn get_jobs_by_name(name: &str) -> Result<Vec<KubeJob>, Error> {
    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::all(client);

    let list_params = ListParams::default().labels(&format!("{}={}", BOSSMAN_JOB_NAME, name));

    let job_list = jobs.list(&list_params).await?.items;

    match job_list.as_slice() {
        [] => Err(Error::UnableToFindJobList(name.to_string())),
        _jobs => Ok(job_list),
    }
}

pub async fn create_job(job: &BossmanJob) -> Result<KubeJob, kube::Error> {
    let kube_job = KubeJob::from(job);
    let namespace = &kube_job.metadata.namespace.as_ref().unwrap();

    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::namespaced(client, namespace);

    let pp = PostParams::default();
    jobs.create(&pp, &kube_job).await
}
