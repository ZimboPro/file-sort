use std::env;

mod contents;

fn main() {
    match get_current_dir() {
        std::result::Result::Err(error) => println!("Error occured: {}", error),
        std::result::Result::Ok(path) => {
            let dir_contents = contents::list_files_in_dir(&path);
            for elem in dir_contents {
                println!("{}", elem.as_path().to_str().unwrap());
            }
            let dir_contents = contents::list_folders_in_dir(&path);
            for elem in dir_contents {
                println!("{}", elem.as_path().to_str().unwrap());
            }
        }
    }
}


fn get_current_dir() -> std::io::Result<std::path::PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}