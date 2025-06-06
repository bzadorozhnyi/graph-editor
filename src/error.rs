pub enum GraphEditorError{ 
    FailedOpenFile,
    FailedSaveFile,
    FailedTakeScreenshot
}

impl GraphEditorError {
    pub fn message(&self) -> &str {
        match self {
            GraphEditorError::FailedOpenFile => "Failed to open the file",
            GraphEditorError::FailedSaveFile => "Failed to save the file",
            GraphEditorError::FailedTakeScreenshot => "Failed to take the screenshot"
        }
    }
}