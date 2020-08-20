tonic::include_proto!("bossman.protobuf.v1alpha1");

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
