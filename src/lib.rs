use serde::{Deserialize, Serialize};

pub mod extract_from_log;
pub mod restore;

#[derive(Debug, Deserialize, Serialize)]
pub struct BlobIdExpectedPath {
    blob_id: String,
    expected_path: String,
}
