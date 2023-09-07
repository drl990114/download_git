extern crate download_git;

#[test]
fn test_parse_shortcut_url() {
    let url = "gh:twbs/bootstrap.git:master";
    let git_url = download_git::git_url_parser::parse_shortcut_url(url).unwrap();
    assert_eq!(git_url.url, "https://github.com/twbs/bootstrap");
    assert_eq!(git_url.name, "bootstrap");
    assert_eq!(git_url.branch, "master");
}

#[test]
fn test_parse_https_url() {
    let url = "https://github.com/twbs/bootstrap.git:master";
    let git_url = download_git::git_url_parser::parse_http_url(url).unwrap();
    assert_eq!(git_url.url, "https://github.com/twbs/bootstrap.git");
    assert_eq!(git_url.name, "bootstrap");
    assert_eq!(git_url.branch, "master");
}

#[test]
fn test_parse_url() {
    let url = "gh:twbs/bootstrap.git:master";
    let git_url = download_git::git_url_parser::parse_url(url).unwrap();
    assert_eq!(git_url.url, "https://github.com/twbs/bootstrap");
    assert_eq!(git_url.name, "bootstrap");
    assert_eq!(git_url.branch, "master");
}
