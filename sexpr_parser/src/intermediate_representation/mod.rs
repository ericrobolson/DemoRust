/// The core representation of the artifact that will be built.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Artifact {
    pub artifact_type: ArtifactType,
}

/// What type of
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ArtifactType {
    Executable(Executable),
    Library(Library),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Library {}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Executable {
    pub file_name: String,
    pub main_module: Module,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Module {
    pub file_name: String,
}
