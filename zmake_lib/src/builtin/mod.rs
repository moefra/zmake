pub mod c;
pub mod cpp;

use crate::id::{ArtifactId, GroupId, QualifiedArtifactId};
use crate::make_builtin;
use ahash::AHashMap;
use semver::Version;
use static_init::dynamic;
use std::str::FromStr;

#[dynamic(lazy)]
pub static KAWAYI_GROUP_ID: GroupId = GroupId::from_str("moe.kawayi").unwrap();

#[dynamic(lazy)]
pub static ZMAKE_ARTIFACT_ID: ArtifactId = ArtifactId::from_str("moe.kawayi:zmake").unwrap();

#[dynamic(lazy)]
pub static ZMAKE_QUALIFIED_ARTIFACT_ID: QualifiedArtifactId = QualifiedArtifactId::from(
    (&*ZMAKE_ARTIFACT_ID).clone(),
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
);

#[dynamic(lazy)]
pub static ZMAKE_V1V0V0: QualifiedArtifactId = QualifiedArtifactId::from(
    (&*ZMAKE_ARTIFACT_ID).clone(),
    Version::parse("1.0.0").unwrap(),
);

make_builtin! {
    self => {
        "moe.kawayi:zmake@1.0.0" => {
            crate::id::IdType::TargetType =>
            {
                "initialize" => "initialize",
                "clean" => "clean",
                "build" => "build",
                "test" => "test",
                "package" => "package",
                "install" => "install",
                "deploy" => "deploy"
            },
            crate::id::IdType::Architecture =>{
                "x64" => "x64",
                "arm64" => "arm64"
            },
            crate::id::IdType::Os =>{
                "windows" => "windows",
                "linux" => "linux",
                "macos" => "macos"
            },
            crate::id::IdType::ToolType =>{
                "archiver" => "archiver",
                "downloader" => "downloader",
                "linker" => "linker"
            },
            crate::id::IdType::ToolName =>{
                "linkExe" => "link_exe",
                "ld" => "ld",
                "badTar" => "bad_tar",
                "curl" => "curl",
                "git" => "git"
            }
        }
    },
    c,
    cpp
}
