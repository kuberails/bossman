#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    #[prost(message, optional, tag="1")]
    pub timeout: ::std::option::Option<i32>,
    /// required
    #[prost(message, optional, tag="2")]
    pub docker_image: ::std::option::Option<::std::string::String>,
    /// optional
    #[prost(message, optional, tag="3")]
    pub image_pull_secrets: ::std::option::Option<::std::string::String>,
    #[prost(map="string, string", tag="4")]
    pub annotations: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// default=default
    #[prost(message, optional, tag="5")]
    pub namespace: ::std::option::Option<::std::string::String>,
    /// optional
    #[prost(message, optional, tag="6")]
    pub backoff_limit: ::std::option::Option<i32>,
    /// optional
    #[prost(message, optional, tag="7")]
    pub completions: ::std::option::Option<i32>,
    /// optional
    #[prost(message, optional, tag="8")]
    pub parallelism: ::std::option::Option<i32>,
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
    #[prost(message, optional, tag="1")]
    pub name: ::std::option::Option<::std::string::String>,
    /// required
    #[prost(message, optional, tag="2")]
    pub options: ::std::option::Option<Options>,
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
