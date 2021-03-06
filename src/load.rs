use yaml_rust::{YamlLoader, Yaml, YamlEmitter};

// use std::path;
use std::path::PathBuf;
use std::fs;

pub fn check_if_config_exists(path_directory: &std::path::PathBuf) -> bool {
  let p = create_config_path(path_directory);
  if p.exists() {
    return true;
  }
  println!("'.file-sort-config.yaml' doesn't exist in {}", path_directory.to_str().unwrap());
  return false;
}

pub fn load_config(path_directory: &std::path::PathBuf) -> Vec<Yaml> {
  let config_file = fs::read_to_string(&create_config_path(path_directory)).unwrap();
  YamlLoader::load_from_str(&config_file).unwrap()
}

fn create_config_path(path_directory: &std::path::PathBuf) -> PathBuf {
//  let p = path_directory.join(".file-sort-config.yaml");
 path_directory.join(".file-sort-config.yaml")
//  *p.as_path()
}

pub fn save_config(path_directory: &std::path::PathBuf, config: &Yaml) {
  let path = create_config_path(path_directory);
  let mut out_str = String::new();
  {
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(config).unwrap(); // dump the YAML object to a String
  }
  let result = fs::write(path, out_str);
  match result {
      Err(err) => {
        println!("An error on occurred while saving: {}", err);
      },
      Ok(_) => {

      }
  }
}
