use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VCSInfo {
    pub vcs: String,
    pub requested_revision: Option<String>,
    pub commit_id: String,
    pub resolved_revision: Option<String>,
    pub resolved_revision_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ArchiveInfo {
    pub hash: Option<String>,
    pub hashes: Option<HashMap<String, String>>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DirInfo {
    pub editable: Option<bool>,
}

impl DirInfo {
    pub fn is_editable(&self) -> bool {
        self.editable.unwrap_or(false)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Info {
    #[serde(rename = "vcs_info")]
    VCS(VCSInfo),

    #[serde(rename = "archive_info")]
    Archive(ArchiveInfo),

    #[serde(rename = "dir_info")]
    Dir(DirInfo),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DirectURL {
    pub url: Url,

    #[serde(flatten)]
    pub info: Info,
}

#[cfg(test)]
mod test {
    use serde_json;

    use super::*;

    #[test]
    fn test_vcs_info() {
        let raw: &str = r#"
        {
            "url": "git+https://github.com/meltano/meltano.git@main",
            "vcs_info": {
                "vcs": "git",
                "commit_id": "1234567890abcdef1234567890abcdef12345678",
                "requested_revision": "main"
            }
        }
        "#;
        let actual: DirectURL = serde_json::from_str(raw).unwrap();
        let expected = DirectURL {
            url: Url::parse("git+https://github.com/meltano/meltano.git@main").unwrap(),
            info: Info::VCS(VCSInfo {
                vcs: "git".to_string(),
                requested_revision: Some("main".to_string()),
                commit_id: "1234567890abcdef1234567890abcdef12345678".to_string(),
                resolved_revision: None,
                resolved_revision_type: None,
            }),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_archive_info_legacy_hash() {
        let raw: &str = r#"
        {
            "url": "https://path/to/archive.tar.gz",
            "archive_info": {
                "hash": "sha256=1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            }
        }
        "#;
        let actual: DirectURL = serde_json::from_str(raw).unwrap();
        let expected = DirectURL {
            url: Url::parse("https://path/to/archive.tar.gz").unwrap(),
            info: Info::Archive(ArchiveInfo {
                hash: Some(
                    "sha256=1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                        .to_string(),
                ),
                hashes: None,
            }),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_archive_info() {
        let raw: &str = r#"
        {
            "url": "https://path/to/archive.tar.gz",
            "archive_info": {
                "hashes": {
                    "sha256": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                    "md5": "1234567890abcdef1234567890abcdef"
                }
            }
        }
        "#;
        let actual: DirectURL = serde_json::from_str(raw).unwrap();
        let expected = DirectURL {
            url: Url::parse("https://path/to/archive.tar.gz").unwrap(),
            info: Info::Archive(ArchiveInfo {
                hash: None,
                hashes: Some(
                    vec![
                        ("sha256".to_string(), "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()),
                        ("md5".to_string(), "1234567890abcdef1234567890abcdef".to_string()),
                    ].into_iter().collect()
                ),
            }),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dir_info() {
        let raw: &str = r#"
        {
            "url": "file:///Users/edgarramirez/Code/meltano/meltano/.venv/lib/python3.11/site-packages/meltano-3.0.0.dist-info/direct_url.json",
            "dir_info": {
                "editable": true
            }
        }
        "#;
        let actual: DirectURL = serde_json::from_str(raw).unwrap();
        let expected = DirectURL {
            url: Url::parse("file:///Users/edgarramirez/Code/meltano/meltano/.venv/lib/python3.11/site-packages/meltano-3.0.0.dist-info/direct_url.json").unwrap(),
            info: Info::Dir(DirInfo {
                editable: Some(true),
            }),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dir_info_not_editable() {
        let raw: &str = r#"
        {
            "url": "file:///Users/edgarramirez/Code/meltano/meltano/.venv/lib/python3.11/site-packages/meltano-3.0.0.dist-info/direct_url.json",
            "dir_info": {
                "editable": false
            }
        }
        "#;
        let actual: DirectURL = serde_json::from_str(raw).unwrap();
        let expected = DirectURL {
            url: Url::parse("file:///Users/edgarramirez/Code/meltano/meltano/.venv/lib/python3.11/site-packages/meltano-3.0.0.dist-info/direct_url.json").unwrap(),
            info: Info::Dir(DirInfo {
                editable: Some(false),
            }),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dir_info_editable_missing() {
        let raw: &str = r#"
        {
            "url": "file:///Users/edgarramirez/Code/meltano/meltano/.venv/lib/python3.11/site-packages/meltano-3.0.0.dist-info/direct_url.json",
            "dir_info": {}
        }
        "#;
        let actual: DirectURL = serde_json::from_str(raw).unwrap();
        let expected = DirectURL {
            url: Url::parse("file:///Users/edgarramirez/Code/meltano/meltano/.venv/lib/python3.11/site-packages/meltano-3.0.0.dist-info/direct_url.json").unwrap(),
            info: Info::Dir(DirInfo {
                editable: None,
            }),
        };
        assert_eq!(actual, expected);
    }
}
