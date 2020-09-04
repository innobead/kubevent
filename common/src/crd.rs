use kube_derive::CustomResource;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
#[kube(apiextensions = "v1beta1", group = "kubevent.io", version = "v1alpha1")]
pub struct RuleSpec {
    // ref: kubectl explain events, extensible middlewares implementing other filters
    pub kind: String,
    pub types: Option<Vec<String>>,
    // pub count: Option<i32>,
    // pub involved_objects: Option<Vec<String>>,
    // pub related_objects: Option<Vec<String>>,
    // pub message_regex: Option<String>,
    // pub reason_regex: Option<String>,
    // pub sources: Option<Vec<String>>,
}

#[derive(CustomResource, Serialize, Deserialize, Default, Clone, Debug)]
#[kube(apiextensions = "v1beta1", group = "kubevent.io", version = "v1alpha1")]
pub struct BrokerSpec {
    pub kind: String,
    pub uri: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub ca: Option<String>,
}

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(apiextensions = "v1beta1", group = "kubevent.io", version = "v1alpha1")]
pub struct RuleBindingSpec {
    pub rule: String,
    pub brokers: Vec<String>,
}
