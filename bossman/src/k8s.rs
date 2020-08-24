mod kube_job;

use crate::bossman::Job as BossmanJob;
use crate::consts::{BOSSMAN_JOB_ID, BOSSMAN_JOB_NAME};
use k8s_openapi::api::batch::v1::Job as KubeJob;
use kube::{
    api::{Api, ListParams, PostParams},
    Client,
};

pub async fn get_job(id: &str) -> Result<KubeJob, kube::Error> {
    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::all(client);

    let list_params = ListParams::default().labels(&format!("{}={}", BOSSMAN_JOB_ID, id));

    Ok(jobs.list(&list_params).await?.items[0].clone())
}

pub async fn get_jobs_by_name(name: &str) -> Result<Vec<KubeJob>, kube::Error> {
    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::all(client);

    let list_params = ListParams::default().labels(&format!("{}={}", BOSSMAN_JOB_NAME, name));

    Ok(jobs.list(&list_params).await?.items)
}

pub async fn create_job(job: &BossmanJob) -> Result<KubeJob, kube::Error> {
    let kube_job = KubeJob::from(job);
    let namespace = &kube_job.metadata.namespace.as_ref().unwrap();

    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::namespaced(client, namespace);

    let pp = PostParams::default();
    jobs.create(&pp, &kube_job).await
}
