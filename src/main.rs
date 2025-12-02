use std::path::PathBuf;

use clap::{Parser, Subcommand};
use env_logger::Env;
use kopia_fsrepo_recovery::{
    check::run_check, extract_from_log::run_extract_from_log, restore::run_restore,
};

const LONG_ABOUT: &str = include_str!("resources/long_about.md");

#[derive(Parser)]
#[command(version, about, long_about = LONG_ABOUT, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Verifies that missing blobs are present in source repo
    Check {
        source_repo: PathBuf,
        #[arg(short, long, default_value = "./missing-blobs.json")]
        missing_blobs_fp: PathBuf,
    },
    /// Extracts list of missing blogs from given log
    ExtractFromLog {
        input_logfile: PathBuf,
        #[arg(short, long, default_value = "./missing-blobs.json")]
        out_file_path: PathBuf,
        #[arg(short, long, default_value_t = false)]
        continue_on_unknown_errors: bool,
    },
    // Copies identified blobs from source repo to destination repo
    Restore {
        source_repo: PathBuf,
        dest_repo: PathBuf,
        #[arg(short, long, default_value = "./missing-blobs.json")]
        missing_blobs_fp: PathBuf,
        #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        skip_source_check: bool,
        // Disables dry-run mode
        #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        for_real: bool,
    },
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check {
            source_repo,
            missing_blobs_fp,
        }) => run_check(source_repo, missing_blobs_fp),
        Some(Commands::ExtractFromLog {
            input_logfile,
            out_file_path,
            continue_on_unknown_errors,
        }) => run_extract_from_log(input_logfile, out_file_path, continue_on_unknown_errors),
        Some(Commands::Restore {
            source_repo,
            dest_repo,
            missing_blobs_fp,
            skip_source_check,
            for_real,
        }) => run_restore(
            source_repo,
            dest_repo,
            missing_blobs_fp,
            skip_source_check,
            for_real,
        ),
        None => Ok(()),
    }
    .unwrap()
}
