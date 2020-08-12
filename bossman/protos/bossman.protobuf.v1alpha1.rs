#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Options {
    /// optional
    #[prost(message, optional, tag="1")]
    pub timeout: ::std::option::Option<i32>,
    /// optional
    #[prost(message, optional, tag="3")]
    pub image_pull_secrets: ::std::option::Option<::std::string::String>,
    /// optional
    #[prost(map="string, string", tag="4")]
    pub annotations: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// default=default
    #[prost(message, optional, tag="5")]
    pub namespace: ::std::option::Option<::std::string::String>,
    /// optional
    #[prost(message, optional, tag="6")]
    pub retries: ::std::option::Option<i32>,
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
    /// optional
    #[prost(message, repeated, tag="11")]
    pub env: ::std::vec::Vec<options::Env>,
    /// optional
    #[prost(message, repeated, tag="12")]
    pub env_from: ::std::vec::Vec<options::EnvFrom>,
}
pub mod options {
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct Env {
        #[prost(oneof="env::Env", tags="2, 3")]
        pub env: ::std::option::Option<env::Env>,
    }
    pub mod env {
        #[derive(Clone, PartialEq, ::prost::Message)]
        #[derive(Serialize, Deserialize)]
        pub struct EnvFrom {
            #[prost(string, tag="1")]
            pub name: std::string::String,
            #[prost(oneof="env_from::ValueFrom", tags="2, 3")]
            pub value_from: ::std::option::Option<env_from::ValueFrom>,
        }
        pub mod env_from {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            #[derive(Serialize, Deserialize)]
            pub enum ValueFrom {
                #[prost(message, tag="2")]
                SecretKeyRef(super::SecretKeyRef),
                #[prost(message, tag="3")]
                ConfigMapKeyRef(super::ConfigMapKeyRef),
            }
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        #[derive(Serialize, Deserialize)]
        pub struct EnvValue {
            #[prost(string, tag="1")]
            pub name: std::string::String,
            #[prost(string, tag="2")]
            pub value: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        #[derive(Serialize, Deserialize)]
        pub struct ConfigMapKeyRef {
            #[prost(string, tag="2")]
            pub name: std::string::String,
            #[prost(string, tag="1")]
            pub key: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        #[derive(Serialize, Deserialize)]
        pub struct SecretKeyRef {
            #[prost(string, tag="2")]
            pub name: std::string::String,
            #[prost(string, tag="1")]
            pub key: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        #[derive(Serialize, Deserialize)]
        pub enum Env {
            #[prost(message, tag="2")]
            Value(EnvValue),
            #[prost(message, tag="3")]
            ValueFrom(EnvFrom),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct EnvFrom {
        #[prost(oneof="env_from::EnvFrom", tags="1, 2")]
        pub env_from: ::std::option::Option<env_from::EnvFrom>,
    }
    pub mod env_from {
        #[derive(Clone, PartialEq, ::prost::Message)]
        #[derive(Serialize, Deserialize)]
        pub struct ConfigMapKeyRef {
            #[prost(string, tag="1")]
            pub name: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        #[derive(Serialize, Deserialize)]
        pub struct SecretKeyRef {
            #[prost(string, tag="1")]
            pub name: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        #[derive(Serialize, Deserialize)]
        pub enum EnvFrom {
            #[prost(message, tag="1")]
            SecretKeyRef(SecretKeyRef),
            #[prost(message, tag="2")]
            ConfigMapKeyRef(ConfigMapKeyRef),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Job {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(string, tag="3")]
    pub docker_image_name: std::string::String,
    #[prost(message, optional, tag="4")]
    pub options: ::std::option::Option<Options>,
    #[prost(enumeration="job::Status", tag="5")]
    pub status: i32,
}
pub mod job {
    /// perform()
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct PerformRequest {
        /// required
        #[prost(message, optional, tag="1")]
        pub name: ::std::option::Option<::std::string::String>,
        /// required
        #[prost(message, optional, tag="2")]
        pub docker_image_name: ::std::option::Option<::std::string::String>,
        /// required
        #[prost(message, optional, tag="3")]
        pub options: ::std::option::Option<super::Options>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct PerformResponse {
        #[prost(message, optional, tag="1")]
        pub job: ::std::option::Option<super::Job>,
    }
    /// get_status()
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct GetStatusRequest {
        #[prost(string, tag="1")]
        pub job_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct GetStatusResponse {
        #[prost(enumeration="Status", tag="1")]
        pub status: i32,
    }
    /// get()
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct GetRequest {
        #[prost(string, tag="1")]
        pub id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct GetResponse {
        #[prost(message, optional, tag="1")]
        pub job: ::std::option::Option<super::Job>,
    }
    /// get_list()
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct GetListRequest {
        #[prost(string, tag="1")]
        pub name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[derive(Serialize, Deserialize)]
    pub struct GetListResponse {
        #[prost(message, repeated, tag="1")]
        pub jobs: ::std::vec::Vec<super::Job>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(Serialize, Deserialize)]
    pub enum Status {
        Waiting = 0,
        Processing = 1,
        Complete = 2,
        Error = 3,
    }
}
