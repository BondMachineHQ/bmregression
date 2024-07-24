extern crate tempdir;
use clap::Parser;

use std::fs::File;
use std::io::{self, Write};
use tempdir::TempDir;
/// The arguments for the program 
#[derive(Parser)]
struct Args {
    /// The regression command to run
    command: String,
    /// The example to run
    example: String,
}

fn main() {
    let args = Args::parse();

    let test_regression = "zedboard_bellstate";    
    if let Err(_) = execute_regression(test_regression) {
        ::std::process::exit(1);
    }
}

// override the clone of bmexample
// override the clone of vmregressiondata
// override the tools installation (uses the tools in the system)
// location of the regression data if resetting

fn execute_regression( regression_name: &str) -> Result<(), io::Error> {
    println!("Running regression: {}", regression_name);

    let tmp_dir = TempDir::new("bmregression")?;
    println!("Working directory: {}", tmp_dir.path().display());

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = File::create(file_path)?;
    writeln!(tmp_file, "Brian was here. Briefly.")?;
    drop(tmp_file);
    tmp_dir.close()?;
    Ok(())
}