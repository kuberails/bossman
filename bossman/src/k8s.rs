mod kube_job;

use crate::bossman::Job as BossmanJob;
use crate::consts::labels::{BOSSMAN_JOB_ID, BOSSMAN_JOB_NAME, MANAGED_BY_KEY, MANAGED_BY_VALUE};
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

pub struct K8s {
    client: Client,
}

impl K8s {
    pub async fn new() -> K8s {
        K8s {
            client: Client::try_default()
                .await
                .expect("Unable to configure kubernetes connection"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<KubeJob>, Error> {
        let jobs: Api<KubeJob> = Api::all(self.client.clone());

        let list_params =
            ListParams::default().labels(&format!("{}={}", MANAGED_BY_KEY, MANAGED_BY_VALUE));

        Ok(jobs.list(&list_params).await?.items)
    }

    pub async fn get_job(&self, id: &str) -> Result<KubeJob, Error> {
        let jobs: Api<KubeJob> = Api::all(self.client.clone());

        let list_params = ListParams::default().labels(&format!(
            "{}={},{}={}",
            BOSSMAN_JOB_ID, id, MANAGED_BY_KEY, MANAGED_BY_VALUE
        ));

        jobs.list(&list_params)
            .await?
            .items
            .get(0)
            .map(ToOwned::to_owned)
            .ok_or_else(|| Error::UnableToFindJob(id.to_string()))
    }

    pub async fn get_jobs_by_name(&self, name: &str) -> Result<Vec<KubeJob>, Error> {
        let jobs: Api<KubeJob> = Api::all(self.client.clone());

        let list_params = ListParams::default().labels(&format!(
            "{}={},{}={}",
            BOSSMAN_JOB_NAME, name, MANAGED_BY_KEY, MANAGED_BY_VALUE
        ));

        let job_list = jobs.list(&list_params).await?.items;

        match job_list.as_slice() {
            [] => Err(Error::UnableToFindJobList(name.to_string())),
            _jobs => Ok(job_list),
        }
    }

    pub async fn create_job(&self, job: &BossmanJob) -> Result<KubeJob, kube::Error> {
        let kube_job = KubeJob::from(job);
        let namespace = &kube_job.metadata.namespace.as_ref().unwrap();

        let jobs: Api<KubeJob> = Api::namespaced(self.client.clone(), namespace);

        let pp = PostParams::default();
        jobs.create(&pp, &kube_job).await
    }
}
