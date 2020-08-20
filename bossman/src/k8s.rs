mod kube_job;

use crate::bossman::Job as BossmanJob;
use k8s_openapi::api::batch::v1::Job as KubeJob;
use kube::{
    api::{Api, PostParams},
    Client,
};

pub async fn get_job(job: &BossmanJob) -> Result<KubeJob, kube::Error> {
    let namespace = job.get_namespace();

    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::namespaced(client, namespace);

    jobs.get(&job.kube_job_name()).await
}

pub async fn create_job(job: &BossmanJob) -> Result<KubeJob, kube::Error> {
    let kube_job: KubeJob = job.into();
    let namespace = &kube_job.metadata.namespace.as_ref().unwrap();

    let client = Client::try_default().await?;
    let jobs: Api<KubeJob> = Api::namespaced(client, namespace);

    let pp = PostParams::default();
    jobs.create(&pp, &kube_job).await
}
