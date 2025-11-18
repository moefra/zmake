use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[serde(untagged)]
pub enum Visibility {
    VisibleToArtifact { visible_to_artifact: Vec<String> },
    VisibleToFile { visible_to_file: Vec<String> },
    VisibleToDir { visible_to_dir: Vec<String> },
    Private,
    Public,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
#[serde(untagged)]
pub enum TransitiveLevel {
    Interface,
    Public,
    Private,
}
