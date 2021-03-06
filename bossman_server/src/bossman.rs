tonic::include_proto!("bossman.protobuf.v1alpha1");

use crate::consts::labels::{BOSSMAN_JOB_ID, BOSSMAN_JOB_NAME};
use crate::error::{CollectionExt, OptionExt};
use chrono::offset::Utc;
use k8s_openapi::api::batch::v1::{Job as KubeJob, JobStatus};
use k8s_openapi::api::core::v1::{
    ConfigMapEnvSource, ConfigMapKeySelector, EnvFromSource, EnvVar, EnvVarSource, SecretEnvSource,
    SecretKeySelector,
};
use std::collections::{BTreeMap, HashMap};
use std::convert::{From, TryFrom};
use thiserror::Error;

impl Job {
    pub fn get_namespace(&self) -> &str {
        self.options
            .as_ref()
            .map(|options| options.namespace.as_ref())
            .flatten()
            .map(String::as_str)
            .unwrap_or("default")
    }

    pub fn kube_job_name(&self) -> String {
        format!("{}-{}", self.name, &self.id[..13])
    }
}

impl TryFrom<&KubeJob> for Job {
    type Error = FromError;

    fn try_from(kube_job: &KubeJob) -> Result<Self, Self::Error> {
        let labels = kube_job.metadata.labels.as_ref().ctx("labels")?;
        let spec = kube_job.spec.as_ref().ctx("spec")?;

        let pod_spec = spec.template.spec.as_ref().ctx("spec.template.spec")?;
        let container = &pod_spec.containers[0];

        Ok(Self {
            id: labels.get_or_err(BOSSMAN_JOB_ID, "labels.id")?,
            name: labels
                .get_or_err(BOSSMAN_JOB_NAME, "labels.name")?
                .to_string(),
            docker_image_name: container
                .image
                .as_ref()
                .ctx("spec.template.spec.containers[0].image")?
                .to_string(),
            options: Some(Options {
                namespace: kube_job.metadata.namespace.as_ref().map(String::from),
                annotations: kube_job
                    .metadata
                    .annotations
                    .as_ref()
                    .map(|btree| btree.clone().into_iter().collect())
                    .unwrap_or_else(|| HashMap::new()),
                args: container
                    .args
                    .as_ref()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| Vec::new()),
                command: container
                    .command
                    .as_ref()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| Vec::new()),
                image_pull_secrets: pod_spec
                    .image_pull_secrets
                    .as_ref()
                    .map(|v| v[0].name.clone())
                    .flatten(),
                parallelism: spec.parallelism,
                completions: spec.completions,
                retries: spec.backoff_limit,
                timeout: spec.active_deadline_seconds,
                env: Vec::new(),
                env_from: Vec::new(),
            }),
            status: Some(get_status(
                &kube_job,
                spec.completions.unwrap_or(1),
                spec.backoff_limit.unwrap_or(6),
            )),
        })
    }
}

impl<T> OptionExt<T, FromError> for Option<T> {
    fn ctx(self, field: &'static str) -> Result<T, FromError> {
        match self {
            Some(v) => Ok(v),
            None => Err(FromError::FieldNotPresent(field)),
        }
    }
}

impl CollectionExt<String, FromError> for BTreeMap<String, String> {
    fn get_or_err(&self, field: &'static str, ctx: &'static str) -> Result<String, FromError> {
        self.get(field).ctx(ctx).map(String::from)
    }
}

impl TryFrom<EnvVar> for options::Env {
    type Error = FromError;

    fn try_from(env_var: EnvVar) -> Result<options::Env, Self::Error> {
        match env_var {
            EnvVar {
                name,
                value: Some(value),
                value_from: None,
            } => Ok(options::Env {
                env: Some(options::env::Env::Value({
                    options::env::EnvValue { name, value: value }
                })),
            }),

            EnvVar {
                name,
                value: None,
                value_from:
                    Some(EnvVarSource {
                        secret_key_ref: Some(secret_key @ SecretKeySelector { .. }),
                        ..
                    }),
            } => Ok(options::Env {
                env: Some(options::env::Env::ValueFrom(options::env::EnvFrom {
                    name,
                    value_from: Some(options::env::env_from::ValueFrom::SecretKeyRef(
                        options::env::SecretKeyRef {
                            key: secret_key.key.clone(),
                            name: secret_key.name.ctx("secret_key_ref.name")?,
                        },
                    )),
                })),
            }),

            EnvVar {
                name,
                value: None,
                value_from:
                    Some(EnvVarSource {
                        config_map_key_ref: Some(config_map @ ConfigMapKeySelector { .. }),
                        ..
                    }),
            } => Ok(options::Env {
                env: Some(options::env::Env::ValueFrom(options::env::EnvFrom {
                    name,
                    value_from: Some(options::env::env_from::ValueFrom::ConfigMapKeyRef(
                        options::env::ConfigMapKeyRef {
                            key: config_map.key.clone(),
                            name: config_map.name.ctx("secret_key_ref.name")?,
                        },
                    )),
                })),
            }),

            _ => Err(FromError::FieldNotPresent("env")),
        }
    }
}

impl TryFrom<EnvFromSource> for options::EnvFrom {
    type Error = FromError;

    fn try_from(env_from_source: EnvFromSource) -> Result<options::EnvFrom, Self::Error> {
        match env_from_source {
            EnvFromSource {
                config_map_ref: Some(config_map_ref @ ConfigMapEnvSource { .. }),
                ..
            } => Ok(options::EnvFrom {
                env_from: Some(options::env_from::EnvFrom::ConfigMapKeyRef(
                    options::env_from::ConfigMapKeyRef {
                        name: config_map_ref.name.ctx("config_map_key_ref.name")?,
                    },
                )),
            }),
            EnvFromSource {
                secret_ref: Some(secret_ref @ SecretEnvSource { .. }),
                ..
            } => Ok(options::EnvFrom {
                env_from: Some(options::env_from::EnvFrom::SecretKeyRef(
                    options::env_from::SecretKeyRef {
                        name: secret_ref.name.ctx("config_map_key_ref.name")?,
                    },
                )),
            }),
            _ => Err(FromError::FieldNotPresent(
                "secret_key_ref and config_map_key_ref not present",
            )),
        }
    }
}

fn get_status(kube_job: &KubeJob, completions: i32, retries: i32) -> job::Status {
    match kube_job.status.as_ref() {
        Some(JobStatus {
            active: Some(n),
            start_time,
            ..
        }) if n >= &1 => job::Status {
            status: Some(job::status::Status::Active(job::status::Active {
                started_at: start_time
                    .as_ref()
                    .map(|inner| inner.0)
                    .unwrap_or_else(|| Utc::now())
                    .to_string(),
            })),
        },

        Some(JobStatus {
            succeeded: Some(n),
            start_time,
            completion_time,
            ..
        }) if n >= &completions => job::Status {
            status: Some(job::status::Status::Completed(job::status::Completed {
                started_at: start_time
                    .as_ref()
                    .map(|inner| inner.0)
                    .unwrap_or_else(|| Utc::now())
                    .to_string(),
                completed_at: completion_time
                    .as_ref()
                    .map(|inner| inner.0)
                    .unwrap_or_else(|| Utc::now())
                    .to_string(),
            })),
        },

        Some(JobStatus {
            failed: Some(n),
            completion_time,
            start_time,
            ..
        }) if n >= &retries => job::Status {
            status: Some(job::status::Status::Failed(job::status::Failed {
                started_at: start_time
                    .as_ref()
                    .map(|inner| inner.0)
                    .unwrap_or_else(|| Utc::now())
                    .to_string(),
                failed_at: completion_time
                    .as_ref()
                    .map(|inner| inner.0)
                    .unwrap_or_else(|| Utc::now())
                    .to_string(),
            })),
        },

        // if its not failed or succeeded its still active
        Some(JobStatus { start_time, .. }) => job::Status {
            status: Some(job::status::Status::Active(job::status::Active {
                started_at: start_time
                    .as_ref()
                    .map(|inner| inner.0)
                    .unwrap_or_else(|| Utc::now())
                    .to_string(),
            })),
        },

        // should not be possible
        None => job::Status {
            status: Some(job::status::Status::Waiting(job::status::Waiting {})),
        },
    }
}

#[derive(Debug, Error)]
pub enum FromError {
    #[error("field {0} not present in kubernetes job")]
    FieldNotPresent(&'static str),
}
