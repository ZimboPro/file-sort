use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::stdin;

use copy_action::CopyAction;

mod contents;
mod load;
mod compare;
mod copy_action;
const VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "-t" {
      file_sort(true);
    } else if args.len() == 2 && args[1] == "-v" {
      println!("Version: {}", VERSION);
    } else {
      file_sort(false);
    }
}

fn file_sort(test: bool) {
    let mut actions: Vec<CopyAction> = Vec::new();
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
                      actions.push(CopyAction::new(c["directory"].as_str().unwrap(), elem, new_path));
                    } else if new_path.exists() {
                      println!("{} already exists", new_path.to_str().unwrap());
                    } else {
                      // let dir = std::path::Path::new(c["directory"].as_str().unwrap());
                      // if !dir.exists() {
                      //   let _ = fs::create_dir_all(dir);
                      // }
                      // let result = fs::rename(elem.as_path(), new_path);
                      // if result.is_err() {
                      //   let copy_result = fs::copy(elem.as_path(), new_path);
                      //   if copy_result.is_ok() {
                      //     let _ = fs::remove_file(elem.as_path());
                      //   }
                      // }
                      copy_file(c["directory"].as_str().unwrap(), elem, new_path);
                    }
                  }
                }
              }
            }
          }
        }
    }
  }
  if test && actions.len() > 0 {
    println!("Do you want to copy the files over? (Y/n)");
    let mut ans :u8 = check_input();
    while ans == 0 {
      println!("Enter a correct option: (Y/n)");
      ans = check_input();
    }
    if ans == 1 {
      for action in actions {
          copy_file(action.directory.as_str(), &action.path, &action.new_path);
      }
    }
  }
}

fn check_input() -> u8 {
  let mut line = String::new();
  let _ = stdin().read_line(&mut line).unwrap();
  let temp = line.to_lowercase();
  let input = temp.as_str();
  match input {
    "y" | "yes" => 1,
    "n" | "no" => 2,
    _ => 0
  }
}

fn copy_file(directory: &str, elem: &PathBuf, new_path: &PathBuf) {
  let dir = std::path::Path::new(directory);
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


fn get_current_dir() -> std::io::Result<std::path::PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}
