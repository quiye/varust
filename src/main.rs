extern crate yaml_rust;
use std::collections::HashMap;
use structopt::StructOpt;
use yaml_rust::{Yaml, YamlLoader};

#[derive(StructOpt)]
struct Cli {
    root: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let docs = YamlLoader::load_from_str(&content).unwrap();
    let doc = &docs[0];
    let map = search_nodes(doc, &args.root);
    for (k, v) in map {
        println!("{}={}", k, v);
    }
}

fn search_nodes<'a>(yaml: &'a yaml_rust::Yaml, path: &str) -> HashMap<&'a str, String> {
    let nodes = path.split('.').collect::<Vec<&str>>();
    fn inner_search_node<'a>(
        yaml: &'a yaml_rust::Yaml,
        pathv: Vec<&str>,
    ) -> HashMap<&'a str, String> {
        if pathv.len() == 1 {
            let &node_name = pathv.first().unwrap();
            // let Yaml::Hash(h) = yaml[node_name];
            match &yaml[node_name] {
                Yaml::Hash(h) => {
                    let mut map: HashMap<&str, String> = HashMap::new();
                    for (k, v) in h {
                        if let Yaml::String(ks) = k {
                            match v {
                                Yaml::String(vs) => map.insert(ks, vs.to_string()),
                                Yaml::Integer(vs) => map.insert(ks, vs.to_string()),
                                _ => None,
                            };
                        }
                    }
                    map
                }
                _ => HashMap::new(),
            }
        } else {
            let mut pathc = pathv.clone();
            let rest_path = pathc.drain(1..).collect();
            let &node_name = pathc.first().unwrap();
            let rest_yaml = &yaml[node_name];
            inner_search_node(&rest_yaml, rest_path)
        }
    }
    inner_search_node(yaml, nodes)
}

#[test]
fn yaml_load_test() {
    let yaml = "
    foo:
      bar:
        HUGA: huge
        PIYO: puyo
        SAZAE: 3
    ";
    let ans: HashMap<&str, String> = [
        ("HUGA", "huge".to_string()),
        ("PIYO", "puyo".to_string()),
        ("SAZAE", "3".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let converted = &YamlLoader::load_from_str(yaml).unwrap()[0];
    assert_eq!(ans, search_nodes(converted, "foo.bar"));
}
