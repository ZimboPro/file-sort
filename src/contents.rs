use std::fs;
use std::env;

enum ContentType {
  File,
  Folder,
  Both
}

fn get_current_dir() -> std::io::Result<std::path::PathBuf> {
  let path = env::current_dir()?;
  Ok(path)
}

pub fn list_files_in_dir(path_directory: &std::path::PathBuf) -> Vec<std::path::PathBuf>  {
  return dir_contents(path_directory, ContentType::File);
}

pub fn list_files_in_curr_dir() -> Vec<std::path::PathBuf>  {
  match get_current_dir() {
    std::result::Result::Err(error) => { 
      println!("Error occurred: {}", error);
      return Vec::new();
    },
    std::result::Result::Ok(path) => {
        return dir_contents(&path, ContentType::File);
    }
  }
}

pub fn list_folders_in_dir(path_directory: &std::path::PathBuf) -> Vec<std::path::PathBuf>  {
  return dir_contents(path_directory, ContentType::Folder);
}

pub fn list_folders_in_curr_dir() -> Vec<std::path::PathBuf>  {
  match get_current_dir() {
    std::result::Result::Err(error) => { 
      println!("Error occurred: {}", error);
      return Vec::new();
    },
    std::result::Result::Ok(path) => {
        return dir_contents(&path, ContentType::Folder);
    }
  }
}

pub fn list_contents_in_dir(path_directory: &std::path::PathBuf) -> Vec<std::path::PathBuf>  {
  return dir_contents(path_directory, ContentType::Both);
}

pub fn list_contents_in_curr_dir() -> Vec<std::path::PathBuf>  {
  match get_current_dir() {
    std::result::Result::Err(error) => {
      println!("Error occurred: {}", error);
      return Vec::new();
    },
    std::result::Result::Ok(path) => {
        return dir_contents(&path, ContentType::Both);
    }
  }
}

fn dir_contents(dir: &std::path::PathBuf, content_type: ContentType) -> Vec<std::path::PathBuf> {
  let paths = fs::read_dir(dir).unwrap();
  let mut dir_contents: Vec<std::path::PathBuf> = Vec::new();

  for path in paths {
    // path.as_ref().unwrap().path().get_dir();
    match content_type {
      ContentType::File => {
        if path.as_ref().unwrap().path().is_file() {
          dir_contents.push(path.as_ref().unwrap().path())
        }
      },
      ContentType::Folder => {
        if path.as_ref().unwrap().path().is_dir() {
          dir_contents.push(path.as_ref().unwrap().path())
        }
      },
      ContentType::Both => dir_contents.push(path.as_ref().unwrap().path())
    }
  }
  return dir_contents;
}
