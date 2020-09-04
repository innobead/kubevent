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
        crd::RuleBinding::crd(),
    ];

    let mut crd_strings = String::new();
    for resource in resources {
        let crd_str = match serde_yaml::to_string(&resource) {
            Ok(s) => s,
            _ => panic!(),
        };

        crd_strings.push_str(format!("{}\n", crd_str.as_str()).as_str())
    }

    let path = &Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .parent()
        .unwrap()
        .join("manifests")
        .join("generated")
        .join("crd.yaml");

    #[allow(unused_must_use)]
    {
        if let Some(p) = path.parent().unwrap().to_str() {
            println!("Saving the generated CRD into {}", p);

            remove_dir_all(p);
            create_dir_all(p);
        }

        match File::create(path) {
            Ok(mut f) => f.write_all(crd_strings.as_bytes()),
            _ => panic!(),
        };
    }
}
