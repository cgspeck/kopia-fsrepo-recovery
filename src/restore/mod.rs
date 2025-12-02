use anyhow::{Context, Result, bail};
use log::{error, info};
use std::{
    fs::{self, File, copy, create_dir_all},
    path::PathBuf,
};

use super::{BlobIdExpectedPath, check};

pub fn run_restore(
    source_repo: &PathBuf,
    dest_repo: &PathBuf,
    missing_blobs_fp: &PathBuf,
    skip_source_check: &bool,
    for_real: &bool,
) -> Result<()> {
    if !skip_source_check {
        check::run_check(source_repo, missing_blobs_fp)?;
    }
    info!(
        "Restoring from '{}' to '{}'",
        source_repo.to_string_lossy(),
        dest_repo.to_string_lossy()
    );

    let rdr = File::open(missing_blobs_fp)?;
    let items: Vec<BlobIdExpectedPath> = serde_json::from_reader(rdr)?;
    let source_repo_base = PathBuf::from(&source_repo);
    let dest_repo_base = PathBuf::from(&dest_repo);
    let total = items.len();
    let mut i = 1;
    let mut missing_or_empty: Vec<BlobIdExpectedPath> = vec![];

    for item in items {
        let src_path = source_repo_base.join(&item.expected_path);
        let dest_path = dest_repo_base.join(&item.expected_path);
        match fs::metadata(&src_path) {
            Ok(metadata) => {
                if metadata.len() > 0 {
                    let dest_parent = dest_path.parent().context(format!(
                        "Unable to find parent path for {}",
                        dest_path.to_string_lossy()
                    ))?;
                    if *for_real {
                        if !dest_parent.exists() {
                            info!("{}/{}: mkdir {}", i, total, dest_parent.to_string_lossy());
                            create_dir_all(dest_parent)?
                        }
                        info!(
                            "{}/{}: copy {} to {}",
                            i,
                            total,
                            src_path.to_string_lossy(),
                            dest_path.to_string_lossy()
                        );
                        copy(src_path, dest_path)?;
                    } else {
                        if !dest_parent.exists() {
                            info!(
                                "{}/{}: dry-run: mkdir {}",
                                i,
                                total,
                                dest_parent.to_string_lossy()
                            );
                        }
                        info!(
                            "{}/{}: dry-run: copy {} to {}",
                            i,
                            total,
                            src_path.to_string_lossy(),
                            dest_path.to_string_lossy()
                        );
                    }
                } else {
                    error!("{} is 0 bytes", &src_path.to_string_lossy());
                    missing_or_empty.push(item);
                }
            }
            Err(_e) => {
                error!("{} is missing", src_path.to_string_lossy());
                missing_or_empty.push(item);
            }
        }
        i += 1;
    }

    if missing_or_empty.len() > 0 {
        let joined: String = missing_or_empty
            .iter()
            .map(|m| m.expected_path.clone())
            .collect::<Vec<String>>()
            .join("\n");
        let completed = format!(include_str!("../resources/restore_error_list.md"), joined);
        bail!("{}", termimad::inline(&completed));
    }

    let maybe_would_be = if *for_real { "were" } else { "would be" };
    info!(
        "{} blobs {} restored from source repo",
        total, maybe_would_be
    );
    Ok(())
}
