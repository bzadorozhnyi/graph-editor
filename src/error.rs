pub enum GraphEditorError{ 
    FailedOpenFile,
    FailedSaveFile
}

impl GraphEditorError {
    pub fn message(&self) -> &str {
        match self {
            GraphEditorError::FailedOpenFile => "Failed to open the file",
            GraphEditorError::FailedSaveFile => "Failed to save the file",
        }
    }
}