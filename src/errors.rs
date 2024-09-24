use thiserror::Error;

#[derive(Error, Debug)]
pub enum HHEError {
    #[error("Couldn't find folder with path `{0}`")]
    ProjectFolderDoesntExist(String, std::io::Error)
}