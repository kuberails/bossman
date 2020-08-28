use crate::bossman::options::{env, env_from, Env, EnvFrom};
use crate::bossman::Job;
use crate::consts::labels::{BOSSMAN_JOB_ID, BOSSMAN_JOB_NAME, MANAGED_BY_KEY, MANAGED_BY_VALUE};
use k8s_openapi::api::batch::v1::{Job as KubeJob, JobSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;
use std::convert::{From, TryFrom};

use k8s_openapi::api::core::v1::{
    ConfigMapEnvSource, ConfigMapKeySelector, Container, EnvFromSource, EnvVar, EnvVarSource,
    LocalObjectReference, PodSpec, PodTemplateSpec, SecretEnvSource, SecretKeySelector,
};

impl From<&Job> for KubeJob {
    fn from(job: &Job) -> Self {
        let job_options = job.options.clone().unwrap_or_default();
        let namespace = job_options
            .namespace
            .unwrap_or_else(|| "default".to_string());

        let image_pull_secrets = match job_options.image_pull_secrets {
            Some(string) => Some(vec![LocalObjectReference { name: Some(string) }]),
            None => None,
        };

        let labels: BTreeMap<String, String> = vec![
            (BOSSMAN_JOB_ID, job.id.as_str()),
            (BOSSMAN_JOB_NAME, job.name.as_str()),
            (MANAGED_BY_KEY, MANAGED_BY_VALUE),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();

        let job_spec = JobSpec {
            backoff_limit: job_options.retries,
            parallelism: job_options.parallelism,
            completions: job_options.completions,
            active_deadline_seconds: job_options.timeout,
            template: PodTemplateSpec {
                metadata: None,
                spec: Some(PodSpec {
                    image_pull_secrets,
                    restart_policy: Some("OnFailure".to_string()),
                    containers: vec![Container {
                        image: Some(job.docker_image_name.clone()),
                        name: job.name.to_string(),
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

        KubeJob {
            metadata: ObjectMeta {
                name: Some(job.kube_job_name()),
                annotations: Some(job_options.annotations.into_iter().collect()),
                labels: Some(labels),
                namespace: Some(namespace),
                ..ObjectMeta::default()
            },
            spec: Some(job_spec),
            ..KubeJob::default()
        }
    }
}

fn convert_to_kube_envs(envs: Vec<Env>) -> Vec<EnvVar> {
    envs.into_iter()
        .map(TryFrom::try_from)
        .filter_map(Result::ok)
        .collect()
}

fn convert_to_kube_env_froms(envs: Vec<EnvFrom>) -> Vec<EnvFromSource> {
    envs.into_iter()
        .map(TryFrom::try_from)
        .filter_map(Result::ok)
        .collect()
}

impl TryFrom<Env> for EnvVar {
    type Error = &'static str;

    fn try_from(env: Env) -> Result<EnvVar, Self::Error> {
        match env.env {
            Some(env::Env::Value(value)) => Ok(EnvVar {
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

                Ok(EnvVar {
                    name: value.name,
                    value_from,
                    value: None,
                })
            }
            None => Err("env.env not present"),
        }
    }
}

impl TryFrom<EnvFrom> for EnvFromSource {
    type Error = &'static str;

    fn try_from(env: EnvFrom) -> Result<EnvFromSource, Self::Error> {
        match env.env_from {
            Some(env_from::EnvFrom::ConfigMapKeyRef(config_map)) => Ok(EnvFromSource {
                config_map_ref: Some(ConfigMapEnvSource {
                    name: Some(config_map.name.clone()),
                    optional: None,
                }),
                ..EnvFromSource::default()
            }),
            Some(env_from::EnvFrom::SecretKeyRef(secret_key)) => Ok(EnvFromSource {
                secret_ref: Some(SecretEnvSource {
                    name: Some(secret_key.name.clone()),
                    optional: None,
                }),
                ..EnvFromSource::default()
            }),
            None => Err("env.env_from not present"),
        }
    }
}
