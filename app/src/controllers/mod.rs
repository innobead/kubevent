use core::fmt::Debug;

use kube::api::Meta;
use snafu::Snafu;

pub mod resource;

#[derive(Debug, Snafu)]
enum Error {}

trait ResourceExtTrait {
    fn description(&self) -> String
    where
        Self: kube::api::Meta,
    {
        Meta::meta(self).self_link.as_ref().unwrap().clone()
    }
}

impl<T> ResourceExtTrait for T where T: kube::api::Meta {}
