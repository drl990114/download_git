Modify according to [git-down](https://github.com/zikani03/git-down), Supplemented test cases, and more convenient to be called.

lets you download one or multiple directories from a Git repository.

## Usage

```rs
use git_download;

git_download::download("https://github.com/twbs/bootstrap.git:main", git_download::DownloadOptions {
  target_files: Some(vec!["dist".to_string(), "README.md".to_string()]),
  dest_path: String::from(TEST_FOLDER)
}).unwrap();

```

## License

MIT
