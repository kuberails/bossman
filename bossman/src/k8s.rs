use crate::bossman::options::{env, env_from};
use crate::bossman::options::{Env, EnvFrom};
use crate::Job as BossmanJob;
use k8s_openapi::api::batch::v1::{Job as KubeJob, JobSpec};
use k8s_openapi::api::core::v1::{
    ConfigMapEnvSource, ConfigMapKeySelector, Container, EnvFromSource, EnvVar, EnvVarSource,
    LocalObjectReference, PodSpec, PodTemplateSpec, SecretEnvSource, SecretKeySelector,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{
    api::{Api, PostParams},
    Client,
};

pub async fn get_job(job: &BossmanJob) -> Result<KubeJob, kube::Error> {
    let client = Client::try_default().await?;

    let namespace = job
        .options
        .clone()
        .unwrap_or_default()
        .namespace
        .unwrap_or_else(|| "default".to_string());

    let jobs: Api<KubeJob> = Api::namespaced(client, &namespace);

    jobs.get(&job_name(&job)).await
}

pub async fn create_job(job: &BossmanJob) -> Result<KubeJob, kube::Error> {
    let client = Client::try_default().await?;

    let job_options = job.options.clone().unwrap_or_default();
    let namespace = job_options
        .namespace
        .unwrap_or_else(|| "default".to_string());

    let image_pull_secrets = match job_options.image_pull_secrets {
        Some(string) => Some(vec![LocalObjectReference { name: Some(string) }]),
        None => None,
    };

    let jobs: Api<KubeJob> = Api::namespaced(client, &namespace);

    let job_spec = JobSpec {
        backoff_limit: job_options.retries,
        parallelism: job_options.parallelism,
        active_deadline_seconds: job_options.timeout,
        template: PodTemplateSpec {
            metadata: None,
            spec: Some(PodSpec {
                image_pull_secrets,
                restart_policy: Some("OnFailure".to_string()),
                containers: vec![Container {
                    image: Some(job.docker_image_name.clone()),
                    name: job.name.clone(),
                    args: job.options.as_ref().map(|options| options.args.clone()),
                    command: job.options.as_ref().map(|options| options.command.clone()),
                    env: job
                        .options
                        .as_ref()
                        .map(|options| convert_to_kube_envs(options.env.clone())),
                    env_from: job
                        .options
                        .as_ref()
                        .map(|options| convert_to_kube_env_froms(options.env_from.clone())),
                    ..Container::default()
                }],
                ..PodSpec::default()
            }),
        },
        ..JobSpec::default()
    };

    let kube_job = KubeJob {
        metadata: ObjectMeta {
            name: Some(job_name(&job)),
            annotations: Some(job_options.annotations.into_iter().collect()),
            namespace: Some(namespace),
            ..ObjectMeta::default()
        },
        spec: Some(job_spec),
        ..KubeJob::default()
    };

    let pp = PostParams::default();
    jobs.create(&pp, &kube_job).await
}

fn convert_to_kube_envs(envs: Vec<Env>) -> Vec<EnvVar> {
    envs.into_iter()
        .filter_map(|env| match env.env {
            Some(env::Env::Value(value)) => Some(EnvVar {
                name: value.name,
                value: Some(value.value),
                value_from: None,
            }),
            Some(env::Env::ValueFrom(value)) => {
                let value_from = match value.value_from {
                    Some(env::env_from::ValueFrom::SecretKeyRef(secret_ref)) => {
                        Some(EnvVarSource {
                            secret_key_ref: Some(SecretKeySelector {
                                name: Some(secret_ref.name),
                                key: secret_ref.key.clone(),
                                optional: None,
                            }),
                            ..EnvVarSource::default()
                        })
                    }
                    Some(env::env_from::ValueFrom::ConfigMapKeyRef(config_map_ref)) => {
                        Some(EnvVarSource {
                            config_map_key_ref: Some(ConfigMapKeySelector {
                                name: Some(config_map_ref.name),
                                key: config_map_ref.key.clone(),
                                optional: None,
                            }),
                            ..EnvVarSource::default()
                        })
                    }
                    None => None,
                };

                Some(EnvVar {
                    name: value.name,
                    value_from,
                    value: None,
                })
            }
            None => None,
        })
        .collect()
}

fn convert_to_kube_env_froms(envs: Vec<EnvFrom>) -> Vec<EnvFromSource> {
    envs.into_iter()
        .filter_map(|env| match env.env_from {
            Some(env_from::EnvFrom::ConfigMapKeyRef(config_map)) => Some(EnvFromSource {
                config_map_ref: Some(ConfigMapEnvSource {
                    name: Some(config_map.name.clone()),
                    optional: None,
                }),
                ..EnvFromSource::default()
            }),
            Some(env_from::EnvFrom::SecretKeyRef(secret_key)) => Some(EnvFromSource {
                secret_ref: Some(SecretEnvSource {
                    name: Some(secret_key.name.clone()),
                    optional: None,
                }),
                ..EnvFromSource::default()
            }),
            None => None,
        })
        .collect()
}

fn job_name(job: &BossmanJob) -> String {
    format!("{}-{}", job.name, &job.id[..8])
}
