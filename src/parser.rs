use crate::{
    errors::HHEError,
    project::{Content, Project, Room},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

pub struct ProjectParser {
    root: PathBuf,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Metadata {
    pub settings: MetadataSettings,
    pub meta: MetadataInfo,
}
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MetadataSettings {
    pub first_room: String,
    pub rooms_folder: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MetadataInfo {
    pub author: Option<String>,
    pub name: String,
}

impl ProjectParser {
    pub fn new(folder_path: &str) -> ProjectParser {
        ProjectParser {
            root: PathBuf::from(folder_path),
            metadata: Metadata::default(),
        }
    }

    pub fn parse(&mut self) -> Result<Project> {
        self.root = match fs::canonicalize(self.root.clone()) {
            Ok(v) => v,
            Err(e) => {
                return Err(HHEError::ProjectFolderDoesntExist(
                    self.root.clone().into_os_string().into_string().unwrap(),
                    e,
                )
                .into())
            }
        };

        let mut meta_toml = self.root.clone();
        meta_toml.push("meta.toml");
        let meta_content = fs::read_to_string(meta_toml)?;
        let meta: Metadata = toml::from_str(&meta_content)?;
        self.metadata = meta;
        let mut proj = Project {
            rooms: HashMap::new(),
            name: self.metadata.meta.name.clone(),
            meta: self.metadata.clone(),
            author: self
                .metadata
                .meta
                .author
                .clone()
                .unwrap_or("No author provided".into()),
        };

        self.parse_rooms(&mut proj);

        Ok(proj)
    }

    pub fn parse_rooms(&mut self, proj: &mut Project) {
        for fold in self.metadata.settings.rooms_folder.clone() {
            self.parse_rooms_folder(proj, &self.root.clone().join(fold));
        }
    }
    pub fn parse_rooms_folder(&mut self, proj: &mut Project, folder_path: &PathBuf) {
        let dir = fs::read_dir(folder_path).unwrap();
        for fold in dir {
            let fold = fold.unwrap();
            let meta = fs::metadata(fold.path()).unwrap();
            if meta.is_dir() {
                self.parse_rooms_folder(proj, &fold.path());
            } else {
                let room = self.parse_room(fold.path());
                proj.rooms.insert(
                    fold.path()
                        .file_stem()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap(),
                    room,
                );
            }
        }
    }
    pub fn parse_room(&mut self, folder_path: PathBuf) -> Room {
        let file = fs::read_to_string(folder_path).expect("Failed to open file");
        let mut pre = String::new();
        let mut post = String::new();

        let mut output: Vec<Content> = vec![];
        let mut pre_or_post = 0;

        let lines = file.lines();
        let mut buffer = "".to_string();

        for ln in lines {
            if ln.trim_end() == "---" {
                match pre_or_post {
                    0 => {
                        pre = buffer.trim().to_string();
                    }
                    1 => {
                        output = parse_content(buffer.trim());
                    }
                    2 => {
                        post = buffer.trim().to_string();
                    }
                    _ => {}
                };
                buffer = "".into();
                pre_or_post += 1;
            } else {
                buffer.push_str(&(ln.to_string() + "\n"));
            }
        }
        match pre_or_post {
            0 => {
                pre = buffer;
            }
            1 => {
                output = parse_content(&buffer.trim());
            }
            2 => {
                post = buffer;
            }
            _ => {}
        };

        Room {
            pre,
            post,
            content: output,
        }
    }
}

fn parse_content(buf: &str) -> Vec<Content> {
    let mut in_lisp = false;
    let mut content = vec![];

    let mut buffer = String::new();

    for ch in buf.chars() {
        if ch == '`' {
            if in_lisp {
                content.push(Content::Lisp(buffer));
            } else {
                buffer.chars().for_each(|ch| {
                    content.push(Content::Char(ch));
                });
            }
            buffer = String::new();
            in_lisp = !in_lisp;
        } else {
            buffer.push(ch);
        }
    }
    if in_lisp {
        content.push(Content::Lisp(buffer));
    } else {
        buffer.chars().for_each(|ch| {
            content.push(Content::Char(ch));
        });
    }

    content
}
