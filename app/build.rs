use kubevent_common::crd;
use std::env;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::Path;

fn main() {
    println!("Generating CRD yaml");

    let resources = vec![
        crd::Broker::crd(),
        crd::Rule::crd(),
        crd::RuleBrokersBinding::crd(),
    ];

    let mut crd_strings = String::new();
    for resource in resources {
        let crd_str = match serde_yaml::to_string(&resource) {
            Ok(s) => s,
            _ => panic!(format!("failed to serialize CRD {:#?} to string", resource)),
        };

        crd_strings.push_str(&format!("{}\n", crd_str))
    }

    let generated_dir_path = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .parent()
        .unwrap()
        .join("manifests")
        .join("generated");

    let crd_path = generated_dir_path.join("crd.yaml");

    #[allow(unused_must_use)]
    {
        println!("Saving the generated CRD into {:?}", generated_dir_path);

        remove_dir_all(&generated_dir_path);
        create_dir_all(&generated_dir_path);

        match File::create(crd_path) {
            Ok(mut f) => f.write_all(crd_strings.as_bytes()),
            _ => panic!("failed to create CRD manifests"),
        };
    }
}
