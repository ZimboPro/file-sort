use std::env;
use std::fs;

mod contents;
mod load;
mod compare;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut test = false;
    if args.len() == 2 && args[1] == "-t" {
      test = true;
    }
    match get_current_dir() {
        std::result::Result::Err(error) => println!("Error occured: {}", error),
        std::result::Result::Ok(path) => {
            if load::check_if_config_exists(&path) {
              let configs = load::load_config(&path);
              let config = &configs[0];

              let dir_contents = &contents::list_files_in_dir(&path);
              if config["configurations"].is_array() {
                for c in config["configurations"].as_vec().unwrap() {
                  if !c["regex"].is_null() {
                    for elem in dir_contents {
                      if compare::matches_regex(elem.as_path().to_str().unwrap(), c["regex"]["search"].as_str().unwrap()) {
                        let mut new_path = compare::update_name(std::path::Path::new(elem.file_name().unwrap()).to_str().unwrap(), c["regex"]["search"].as_str().unwrap(),
                        c["regex"]["output"].as_str().unwrap(), c["directory"].as_str().unwrap());
                        if new_path.extension() != elem.extension() {
                          // new_path.push(path: P);
                          new_path.set_extension(elem.extension().unwrap());
                        }
                        let new_path = &new_path;
                        if test {
                          println!("{} => {}", elem.as_path().to_str().unwrap(), new_path.to_str().unwrap());
                        } else if new_path.exists() {
                          println!("{} already exists", new_path.to_str().unwrap());
                        } else {
                          let dir = std::path::Path::new(c["directory"].as_str().unwrap());
                          if !dir.exists() {
                            let _ = fs::create_dir_all(dir);
                          }
                          let result = fs::rename(elem.as_path(), new_path);
                          if result.is_err() {
                            let copy_result = fs::copy(elem.as_path(), new_path);
                            if copy_result.is_ok() {
                              let _ = fs::remove_file(elem.as_path());
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
        }
    }
}


fn get_current_dir() -> std::io::Result<std::path::PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}
