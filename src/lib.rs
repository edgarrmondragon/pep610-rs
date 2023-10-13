use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VCSInfo {
    pub vcs: String,
    pub requested_revision: Option<String>,
    pub commit_id: String,
    pub resolved_revision: Option<String>,
    pub resolved_revision_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchiveInfo {
    pub hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirInfo {
    pub editable: Option<bool>,
}

impl DirInfo {
    pub fn is_editable(&self) -> bool {
        self.editable.unwrap_or(false)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Info {
    #[serde(rename = "vcs_info")]
    VCS(VCSInfo),

    #[serde(rename = "archive_info")]
    Archive(ArchiveInfo),

    #[serde(rename = "dir_info")]
    Dir(DirInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectURL {
    pub url: String,

    #[serde(flatten)]
    pub info: Info,
}
