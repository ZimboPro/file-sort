use regex::Regex;

pub fn compare_regex<'a>(file_name: &'a str, regex_str: &str, output: &str) -> String {
  let re = Regex::new(regex_str).unwrap();
  if re.is_match(file_name) {
    return re.replace_all(file_name, output).as_ref().to_owned();
    // let c = re.replace_all(file_name, output).as_ref().to_owned();
    // println!("====== {} =======", c);
    // return c;
  }
  return String::from("");
}

pub fn matches_regex(file_name: &str, regex_str: &str) -> bool {
  let re = Regex::new(regex_str).unwrap();
  re.is_match(file_name)
}

pub fn update_name(file_name: & str, regex_str: &str, output: &str, directory: &str) -> std::path::PathBuf {
  let re = Regex::new(regex_str).unwrap();
  let d = std::path::Path::new(directory);
  d.join(re.replace_all(file_name, output).as_ref().to_owned())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn matches() {
      assert_ne!(compare_regex("file.txt", "file", "new_name"), "");
      assert_eq!(compare_regex("file.txt", "file", "new_name"), "new_name.txt");
  }

  #[test]
  fn matches_replace() {
      assert_ne!(compare_regex("file 1.txt", r"file (?P<ep>\d+)", "new_name $ep"), "");
      assert_eq!(compare_regex("file 1.txt", r"file (?P<ep>\d+)", "new_name $ep"), "new_name 1.txt");
      assert_eq!(compare_regex("file 10.txt", r"file (?P<ep>\d+)", "new_name $ep"), "new_name 10.txt");
  }
}
