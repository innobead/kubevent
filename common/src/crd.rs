use kube_derive::CustomResource;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
#[kube(group = "kubevent.io", version = "v1alpha1")]
pub struct RuleSpec {
    // ref: kubectl explain events, extensible middlewares implementing other filters
    name: String,
    types: Vec<String>,
    count: i32,
    involved_objects: Vec<String>,
    related_objects: Vec<String>,
    message_regex: String,
    reason_regex: String,
    sources: Vec<String>,
}

#[derive(CustomResource, Serialize, Deserialize, Default, Clone, Debug)]
#[kube(group = "kubevent.io", version = "v1alpha1")]
pub struct BrokerSpec {
    name: String,
    uri: String,
    user: String,
    password: String,
    token: String,
}

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "kubevent.io", version = "v1alpha1")]
pub struct RuleBindingSpec {
    name: String,
    rules: Vec<String>,
    broker: Broker,
}
