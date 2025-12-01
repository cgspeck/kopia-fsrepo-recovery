use anyhow::{Context, Result, bail};
use log::{error, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
struct ObjectAndBlobId {
    object_id: String,
    blob_id: String,
    expected_path: String,
}


fn to_expected_path(blob_id: &str) -> String {
    return format!("{}/{}/{}.f", &blob_id[0..3], &blob_id[3..6], &blob_id[6..]);
}

pub fn extract_from_log(
    input_logfile: &PathBuf,
    out_file_path: &PathBuf,
    continue_on_unknown_errors: &bool,
) -> Result<()> {
    info!(
        "Extracting missing blobs from '{}'",
        input_logfile.to_string_lossy()
    );
    let re_error_count = Regex::new(r"^encountered (?P<error_count>\d+)").unwrap(); // Named groups
    // ^error processing .* object (.*) is backed by missing blob (.*)$
    let re_objects_and_blobs = Regex::new(
        r"^error processing .* object (?P<object_id>.*) is backed by missing blob (?P<blob_id>.*)$",
    )
    .unwrap();

    let file_in = File::open(input_logfile)?;
    let reader = BufReader::new(file_in);
    let mut maybe_error_count: Option<u32> = None;
    let mut object_and_blob_ids: Vec<ObjectAndBlobId> = vec![];
    let mut unprocessable_errors: Vec<String> = vec![];

    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading

        if line.starts_with("error processing") {
            if let Some(captures) = re_objects_and_blobs.captures(&line) {
                let object_id = captures
                    .name("object_id")
                    .map(|m| m.as_str())
                    .context("Unable to parse object_id")?;
                let blob_id = captures
                    .name("blob_id")
                    .map(|m| m.as_str())
                    .context("Unable to parse blob_id")?;

                object_and_blob_ids.push(ObjectAndBlobId {
                    object_id: object_id.into(),
                    blob_id: blob_id.into(),
                    expected_path: to_expected_path(blob_id),
                });
            } else {
                unprocessable_errors.push(line.clone());
            }
        }

        if let Some(captures) = re_error_count.captures(&line) {
            let captured_error_count = captures.name("error_count").map(|m| m.as_str());

            if let Some(error_count_str) = captured_error_count {
                maybe_error_count = Some(error_count_str.parse::<u32>().unwrap());
            }
        }
    }

    let error_count = maybe_error_count.context("Unable to read total error count!")?;
    info!("Error_count: {:?}", error_count);
    if object_and_blob_ids.len() != error_count as usize {
        error!(
            "{} errors reported but only {} relate to missing blobs",
            error_count,
            object_and_blob_ids.len()
        );
    }
    if !unprocessable_errors.is_empty() {
        let joined: String = unprocessable_errors.join("\n");
        if !continue_on_unknown_errors {
            let completed = format!(
                include_str!("../resources/extract_unknown_error_help.md"),
                joined
            );
            bail!("{}", termimad::inline(&completed))
        } else {
            let completed = format!(
                include_str!("../resources/extract_unknown_error_list.md"),
                joined
            );
            warn!("{}", termimad::inline(&completed))
        }
    }
    // object_and_blob_ids.sort();
    // object_and_blob_ids.dedup();
    info!("{} unique blobs identified", object_and_blob_ids.len());

    info!("Writing '{:#?}'", out_file_path);
    serde_json::to_writer_pretty(&File::create(out_file_path)?, &object_and_blob_ids)?;
    Ok(())
}
