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
    // match doc {
    //     Yaml::Hash(h) => {
    //         println!("{:?}", h);
    //         for y in h {
    //             println!("{:?}", y);
    //         }
    //     }
    //     Yaml::Boolean(b) => println!("{}", b),
    //     _ => println!("none"),
    // }
    // println!("{:?}", doc.);
    // println!("{:?}", &docs[0]);
    // for x in &docs[0] {
    //     println!("{:?}", x);
    // }
    let map = search_nodes(doc, &args.root);
    for (k,v) in map {
        println!("{}={}",k,v);
    }
}

fn search_nodes<'a>(yaml: &'a yaml_rust::Yaml, path: &str) -> HashMap<&'a str, &'a str> {
    let nodes = path.split('.').collect::<Vec<&str>>();
    fn inner_search_node<'a>(yaml: &'a yaml_rust::Yaml, pathv: Vec<&str>) -> HashMap<&'a str, &'a str> {
        if pathv.len() == 1 {
            let &node_name = pathv.first().unwrap();
            // let Yaml::Hash(h) = yaml[node_name];
            match &yaml[node_name] {
                Yaml::Hash(h) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    for (k, v) in h {
                        if let Yaml::String(ks) = k {
                            if let Yaml::String(vs) = v {
                                map.insert(&ks, &vs);
                            }
                        }
                    }
                    map
                },
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
    hoge:
      HUGA: huge
      PIYO: puyo
    ";
    let ans: HashMap<&str, &str> = [("HUGA", "huge"), ("PIYO", "puyo")]
        .iter()
        .cloned()
        .collect();
    let converted = &YamlLoader::load_from_str(yaml).unwrap()[0];
    assert_eq!(ans, search_nodes(converted, "hoge"));
}
