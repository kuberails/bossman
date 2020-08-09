#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    #[prost(map="string, string", tag="4")]
    pub annotations: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// optional
    #[prost(string, repeated, tag="9")]
    pub args: ::std::vec::Vec<std::string::String>,
    /// optional
    #[prost(string, repeated, tag="10")]
    pub command: ::std::vec::Vec<std::string::String>,
    #[prost(message, repeated, tag="11")]
    pub env: ::std::vec::Vec<options::Env>,
    #[prost(message, repeated, tag="12")]
    pub env_from: ::std::vec::Vec<options::EnvFrom>,
    #[prost(oneof="options::Timeout", tags="1")]
    pub timeout: ::std::option::Option<options::Timeout>,
    #[prost(oneof="options::DockerImage", tags="2")]
    pub docker_image: ::std::option::Option<options::DockerImage>,
    #[prost(oneof="options::ImagePullSecrets", tags="3")]
    pub image_pull_secrets: ::std::option::Option<options::ImagePullSecrets>,
    #[prost(oneof="options::Namespace", tags="5")]
    pub namespace: ::std::option::Option<options::Namespace>,
    #[prost(oneof="options::BackoffLimit", tags="6")]
    pub backoff_limit: ::std::option::Option<options::BackoffLimit>,
    #[prost(oneof="options::Completions", tags="7")]
    pub completions: ::std::option::Option<options::Completions>,
    #[prost(oneof="options::Parallelism", tags="8")]
    pub parallelism: ::std::option::Option<options::Parallelism>,
}
pub mod options {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Env {
        #[prost(oneof="env::Env", tags="2, 3")]
        pub env: ::std::option::Option<env::Env>,
    }
    pub mod env {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Env {
            #[prost(message, tag="2")]
            Value(super::EnvValue),
            #[prost(message, tag="3")]
            ValueFrom(super::EnvFrom),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnvFrom {
        #[prost(string, tag="1")]
        pub name: std::string::String,
        #[prost(oneof="env_from::ValueFrom", tags="2, 3")]
        pub value_from: ::std::option::Option<env_from::ValueFrom>,
    }
    pub mod env_from {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ValueFrom {
            #[prost(message, tag="2")]
            SecretKeyRef(super::SecretKeyRef),
            #[prost(message, tag="3")]
            ConfigMapKeyRef(super::ConfigMapKeyRef),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnvValue {
        #[prost(string, tag="1")]
        pub name: std::string::String,
        #[prost(string, tag="2")]
        pub value: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfigMapKeyRef {
        #[prost(string, tag="1")]
        pub key: std::string::String,
        #[prost(string, tag="2")]
        pub name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecretKeyRef {
        #[prost(string, tag="1")]
        pub key: std::string::String,
        #[prost(string, tag="2")]
        pub name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Timeout {
        #[prost(int64, tag="1")]
        Timeout(i64),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DockerImage {
        #[prost(string, tag="2")]
        DockerImage(std::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ImagePullSecrets {
        #[prost(string, tag="3")]
        ImagePullSecrets(std::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Namespace {
        #[prost(string, tag="5")]
        Namespace(std::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackoffLimit {
        #[prost(int32, tag="6")]
        BackoffLimit(i32),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Completions {
        #[prost(int32, tag="7")]
        Completions(i32),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parallelism {
        #[prost(int32, tag="8")]
        Parallelism(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    #[prost(string, tag="1")]
    pub name: std::string::String,
    #[prost(string, tag="2")]
    pub id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub options: ::std::option::Option<Options>,
    #[prost(enumeration="Status", tag="4")]
    pub status: i32,
}
/// perform()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformRequest {
    /// required
    #[prost(message, optional, tag="2")]
    pub options: ::std::option::Option<Options>,
    #[prost(oneof="perform_request::Name", tags="1")]
    pub name: ::std::option::Option<perform_request::Name>,
}
pub mod perform_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Name {
        #[prost(string, tag="1")]
        Name(std::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformResponse {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(enumeration="Status", tag="2")]
    pub status: i32,
}
/// get_status()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusRequest {
    #[prost(string, tag="1")]
    pub name: std::string::String,
}
// get_status()

/// get()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag="1")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag="1")]
    pub job: ::std::option::Option<Job>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusResponse {
    #[prost(string, tag="1")]
    pub job_id: std::string::String,
}
/// get_list()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetListRequest {
    #[prost(string, tag="1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetListResponse {
    #[prost(message, repeated, tag="1")]
    pub jobs: ::std::vec::Vec<Job>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Waiting = 0,
    Processing = 1,
    Complete = 2,
    Error = 3,
}
