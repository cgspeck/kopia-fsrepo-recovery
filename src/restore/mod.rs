use anyhow::{Context, Result, bail};
use log::{error, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::BlobIdExpectedPath;

pub fn restore(
    source_repo: &PathBuf,
    dest_repo: &PathBuf,
    missing_blobs_fp: &PathBuf,
    skip_source_check: &bool,
    dry_run: &bool,
) -> Result<()> {
    info!(
        "Restoring from '{}' to '{}'",
        source_repo.to_string_lossy(),
        dest_repo.to_string_lossy()
    );

    let rdr = File::open(missing_blobs_fp)?;
    let items: Vec<BlobIdExpectedPath> = serde_json::from_reader(rdr)?;
    let source_repo_base = PathBuf::from(&source_repo);

    if !skip_source_check {
        let mut missing: Vec<BlobIdExpectedPath> = vec![];
        for item in items {
            let src_path = source_repo_base.join(&item.expected_path);
            match fs::metadata(&src_path) {
                Ok(metadata) => {
                    if metadata.len() > 0 {
                        error!("{} is 0 bytes", &src_path.to_string_lossy());
                        missing.push(item);
                    }
                }
                Err(e) => {
                    error!("{} is missing", src_path.to_string_lossy());
                    missing.push(item);
                }
            }
        }
    }

    Ok(())
}
