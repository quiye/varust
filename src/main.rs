extern crate yaml_rust;
use std::collections::HashMap;
use structopt::StructOpt;
use yaml_rust::{Yaml, YamlLoader};

#[derive(StructOpt)]
struct Cli {
    node: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(short = "o", long = "on")]
    base_node: Option<String>,
    #[structopt(short = "s", long = "show")]
    /// Show all paths
    show: bool,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let docs = YamlLoader::load_from_str(&content).unwrap();
    let doc = &docs[0];
    if args.show {
        show_nodes(doc)
            .into_iter()
            .for_each(move |x| println!("{}", x));
        return;
    }
    let map = search_nodes(doc, &args.node);
    let mut base_map = match args.base_node {
        Some(node) => search_nodes(doc, &node),
        None => HashMap::new(),
    };

    for (k, v) in map {
        base_map.insert(k, v);
    }
    for (k, v) in base_map {
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
            match &yaml[node_name] {
                Yaml::Hash(h) => {
                    let mut map: HashMap<&str, String> = HashMap::new();
                    for (k, v) in h {
                        if let Yaml::String(ks) = k {
                            match v {
                                Yaml::String(vs) => map.insert(ks, vs.to_string()),
                                Yaml::Integer(vs) => map.insert(ks, vs.to_string()),
                                Yaml::Real(vs) => map.insert(ks, vs.to_string()),
                                Yaml::Boolean(vs) => map.insert(ks, vs.to_string()),
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
        PI: 3.141592
        BOOLEAN: true
    ";
    let ans: HashMap<&str, String> = [
        ("HUGA", "huge".to_string()),
        ("PIYO", "puyo".to_string()),
        ("SAZAE", "3".to_string()),
        ("PI", "3.141592".to_string()),
        ("BOOLEAN", "true".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let converted = &YamlLoader::load_from_str(yaml).unwrap()[0];
    assert_eq!(ans, search_nodes(converted, "foo.bar"));
}

fn show_nodes<'a>(yaml: &'a yaml_rust::Yaml) -> Vec<String> {
    fn inner_search_node<'a>(yaml: &'a yaml_rust::Yaml, nodes: &Vec<String>) -> Vec<String> {
        match &yaml {
            Yaml::Hash(h) => {
                let mut vv: Vec<String> = Vec::new();
                for (k, v) in h {
                    if let Yaml::String(k) = k {
                        vv.extend(
                            inner_search_node(v, &nodes)
                                .into_iter()
                                .map(|x| k.to_string() + "." + &x),
                        );
                    }
                }
                // TODO: use tailrec if enable
                vv
            }
            Yaml::String(s) => vec![s.to_string()],
            Yaml::Integer(s) => vec![s.to_string()],
            Yaml::Real(s) => vec![s.to_string()],
            Yaml::Boolean(s) => vec![s.to_string()],
            _ => Vec::new(),
        }
    }
    inner_search_node(yaml, &Vec::new())
}

#[test]
fn show_path_test() {
    let yaml = "
    foo:
      bar:
        HUGA: huge
        PIYO: puyo
        SAZAE: 3
        PI: 3.141592
        BOOLEAN: true
    ";
    let ans = vec![
        "foo.bar.HUGA.huge".to_string(),
        "foo.bar.PIYO.puyo".to_string(),
        "foo.bar.SAZAE.3".to_string(),
        "foo.bar.PI.3.141592".to_string(),
        "foo.bar.BOOLEAN.true".to_string(),
    ];
    let converted = &YamlLoader::load_from_str(yaml).unwrap()[0];
    assert_eq!(ans, show_nodes(converted));
}
