extern crate git_download;

const TEST_FOLDER: &'static str = "./tests/bootstrap";

#[test]
fn test_download() {
  git_download::download("https://github.com/twbs/bootstrap.git:main", git_download::DownloadOptions {
    target_files: Some(vec!["dist".to_string(), "README.md".to_string()]),
    dest_path: String::from(TEST_FOLDER)
  }).unwrap();

  assert!(std::path::Path::new(TEST_FOLDER).exists());
  assert!(std::path::Path::new(TEST_FOLDER).join("dist").exists());
  assert!(std::path::Path::new(TEST_FOLDER).join("README.md").exists());
}
