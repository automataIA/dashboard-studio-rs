/// Upload state tracking for CSV files
#[derive(Clone, Debug, PartialEq)]
pub enum UploadState {
    /// No upload in progress
    Idle,
    /// User is selecting a file
    SelectingFile,
    /// File is being uploaded/read
    Uploading {
        /// Progress percentage (0-100)
        progress: f64,
        /// Bytes read so far
        bytes_read: u64,
        /// Total file size in bytes
        total_bytes: u64,
    },
    /// File is being parsed
    Parsing {
        /// Progress percentage (0-100)
        progress: f64,
        /// Number of rows processed
        rows_processed: u64,
    },
    /// Upload completed successfully
    Completed,
    /// Upload failed with error message
    Failed(String),
}

/// Upload progress data for UI display
#[derive(Clone, Debug)]
pub struct UploadProgress {
    /// Current upload state
    pub state: UploadState,
    /// Name of the file being uploaded
    pub filename: Option<String>,
    /// Size of the file in bytes
    pub file_size: Option<u64>,
}

impl Default for UploadProgress {
    fn default() -> Self {
        Self {
            state: UploadState::Idle,
            filename: None,
            file_size: None,
        }
    }
}
