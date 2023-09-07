extern crate fs_extra;
extern crate regex;
extern crate tempfile;

use std::{fs, vec};
use std::path::{Path, PathBuf};

pub mod errors;
pub mod git;
pub mod git_url_parser;

pub struct DownloadOptions {
    pub target_files: Option<Vec<String>>,
    pub dest_path: String
}

pub fn download(url: &str, opts: DownloadOptions) -> Result<(), errors::GitDownError> {
    let git_url = git_url_parser::parse_url(&url);
    let git_dir = git::sparse_checkout(git_url.unwrap().clone(), opts.target_files.unwrap_or(vec!["./".to_string()]));

    let dest_path = Path::new(&opts.dest_path);

    move_files(&git_dir.unwrap().contents(), &dest_path);
    Ok(())
}


fn move_files(source_paths: &Vec<PathBuf>, dest_path: &Path) {
  let options = fs_extra::dir::CopyOptions::new();

  if !dest_path.exists() {
      fs::create_dir(dest_path).expect("Cannot create destination directory");
  }

  let dest = dest_path.to_str().unwrap().to_string();
  let mut sources: Vec<String> = Vec::new();

  for path in source_paths {
      sources.push(path.to_str().unwrap().to_string());
  }

  fs_extra::move_items(&sources, &dest, &options)
      .expect(&format!("Failed to move files to {}", dest));
}
