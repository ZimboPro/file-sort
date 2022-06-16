use std::path::PathBuf;

pub struct CopyAction {
  pub directory: String,
  pub path: PathBuf,
  pub new_path: PathBuf
}

impl CopyAction {
    pub fn new(directory: &str, path: &PathBuf, new_path: &PathBuf) -> Self {
      CopyAction {
        directory: directory.to_owned(),
        path: path.to_path_buf(),
        new_path: new_path.to_path_buf()
      }
    }
}
