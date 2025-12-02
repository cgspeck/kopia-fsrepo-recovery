use anyhow::{Result, bail};
use indicatif::ProgressBar;
use log::{error, info};
use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::BlobIdExpectedPath;

pub fn run_check(source_repo: &PathBuf, missing_blobs_fp: &PathBuf) -> Result<()> {
    info!(
        "Checking source repository from '{}'",
        source_repo.to_string_lossy(),
    );

    let rdr = File::open(missing_blobs_fp)?;
    let items: Vec<BlobIdExpectedPath> = serde_json::from_reader(rdr)?;
    let source_repo_base = PathBuf::from(&source_repo);
    let length: u64 = items.len().try_into()?;
    let pb = ProgressBar::new(length);

    let mut missing: Vec<BlobIdExpectedPath> = vec![];
    for item in items {
        let src_path = source_repo_base.join(&item.expected_path);
        match fs::metadata(&src_path) {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    error!("{} is 0 bytes", &src_path.to_string_lossy());
                    missing.push(item);
                }
            }
            Err(_e) => {
                error!("{} is missing", src_path.to_string_lossy());
                missing.push(item);
            }
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");

    if missing.len() > 0 {
        bail!("One or more required blobs were missing or zero bytes.");
    } else {
        info!("All missing blobs present and accounted for in source repo")
    }

    Ok(())
}
