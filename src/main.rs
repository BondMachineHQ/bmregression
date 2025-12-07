//! # bmregression
//!
//! A regression testing tool for BondMachine examples.
//!
//! This tool manages and executes regression tests for the BondMachine project by:
//! - Running commands defined in test configurations
//! - Comparing output against expected results
//! - Tracking test status and differences
//!
//! ## Architecture
//!
//! The tool works with two repositories:
//! - **bmexamples**: Contains example projects with source code and build configurations
//! - **bmregressiondata**: Stores test configurations (`config.yaml`) and expected outputs
//!
//! ## Workflow
//!
//! 1. Clones or uses existing copies of bmexamples and bmregressiondata repositories
//! 2. Reads test configurations from bmregressiondata
//! 3. Executes commands in corresponding bmexamples directories
//! 4. Compares generated output with stored expected results
//! 5. Reports test status (passed/failed/differences)

extern crate tempdir;
use clap::{Parser, Subcommand};
use yaml_rust::YamlLoader;

use std::fs;
use std::io::{self};
use std::process::Command;
use tempdir::TempDir;

/// Command-line interface for the bmregression tool.
///
/// Provides options for running, listing, describing, resetting, and diffing regression tests.
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// The regression to run. If not specified, all regressions are considered. If the command is 'run', the regressions will be run according to the configured frequency
    #[clap(short, long, default_value = "")]
    reg_name: String,
    /// Debug flag
    #[clap(short, long, default_value = "false")]
    debug: bool,
    /// The directory where the regression data is stored, if not specified, the data will be cloned from the data repository and discarded after the run
    #[clap(long, default_value = "")]
    data_dir: String,
    /// The directory where the examples repository is stored, if not specified, the data will be cloned from the data repository and discarded after the run
    #[clap(long, default_value = "")]
    examples_dir: String,
    /// Example repository URL
    #[clap(
        long,
        default_value = "https://github.com/BondMachineHQ/bmexamples.git"
    )]
    examples_url: String,
    /// Data repository URL
    #[clap(
        long,
        default_value = "https://github.com/BondMachineHQ/bmregressiondata.git"
    )]
    data_url: String,
    /// Use the tools in the system instead of the ones installed from the official sources
    #[clap(short, long, default_value = "false")]
    system_tools: bool,
    /// Filter tests by tag(s). Multiple tags can be specified comma-separated. If not specified, only tests with 'default' tag are selected
    #[clap(short, long, default_value = "default")]
    tag: String,
}

/// Available subcommands for regression test operations.
#[derive(Subcommand)]
enum Commands {
    /// List the available regressions
    List { name: Option<String> },
    /// Describe one or more regressions
    Describe { name: Option<String> },
    /// Run one or more regressions
    Run { name: Option<String> },
    /// Reset one or more regressions
    Reset { name: Option<String> },
    /// Diff the results of one or more regressions
    Diff { name: Option<String> },
}

/// Main entry point for the bmregression tool.
///
/// # Workflow
///
/// 1. Parses command-line arguments
/// 2. Creates a temporary working directory
/// 3. Clones or uses existing repositories (bmexamples and bmregressiondata)
/// 4. Executes the requested command
/// 5. Cleans up temporary resources
///
/// # Errors
///
/// Returns an error if:
/// - No command is specified
/// - Repository cloning fails
/// - Temporary directory creation fails
/// - Any regression operation fails
fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    // Ensure a command is specified
    if args.command.is_none() {
        println!("No command specified");
        ::std::process::exit(1);
    }

    // Create a temporary directory for cloned repositories and intermediate files
    let tmp_dir = TempDir::new("bmregression")?;
    if args.debug {
        println!("Working directory: {}", tmp_dir.path().display());
    }

    // Setup examples repository (either use provided directory or clone)
    let mut srcdir = args.examples_dir.clone();
    if args.examples_dir.is_empty() {
        let clone_dir = tmp_dir.path().join("examples");
        let clone_url = args.examples_url;
        if args.debug {
            println!(
                "Cloning examples repository from: {} to {}",
                clone_url,
                clone_dir.display()
            );
        }

        let git_clone = Command::new("git")
            .arg("clone")
            .arg(clone_url)
            .arg(clone_dir)
            .output()?;
        if !git_clone.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error cloning examples repository",
            ));
        }
        srcdir = tmp_dir
            .path()
            .join("examples")
            .to_str()
            .unwrap()
            .to_string();
    }

    // Setup regression data repository (either use provided directory or clone)
    let mut tgtdir = args.data_dir;
    if tgtdir.is_empty() {
        let clone_dir = tmp_dir.path().join("regressiondata");
        let clone_url = args.data_url;
        if args.debug {
            println!(
                "Cloning regression data repository from: {} to {}",
                clone_url,
                clone_dir.display()
            );
        }

        let git_clone = Command::new("git")
            .arg("clone")
            .arg(clone_url)
            .arg(clone_dir)
            .output()?;
        if !git_clone.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error cloning regression data repository",
            ));
        }
        tgtdir = tmp_dir
            .path()
            .join("regressiondata")
            .to_str()
            .unwrap()
            .to_string();
    }

    // Parse tags into a vector for easier filtering
    let tags: Vec<String> = args.tag.split(',').map(|s| s.trim().to_string()).collect();

    // Execute the requested command
    match args.command.unwrap() {
        Commands::List { name } => {
            if let Err(_) = list_regressions(
                &srcdir,
                &tgtdir,
                &name.unwrap_or("".to_string()),
                &tags,
                args.debug,
            ) {
                println!("Error listing regressions");
            }
        }
        Commands::Describe { name } => {
            if let Err(_) = describe_regressions(
                &srcdir,
                &tgtdir,
                &name.unwrap_or("".to_string()),
                &tags,
                args.debug,
            ) {
                println!("Error describing regressions");
            }
        }
        Commands::Run { name } => {
            if let Err(err) = run_regressions(
                &srcdir,
                &tgtdir,
                &name.unwrap_or("".to_string()),
                &tags,
                args.debug,
            ) {
                println!("Error executing regression: {}", err);
            }
        }
        Commands::Reset { name } => {
            if let Err(_) = reset_regressions(
                &srcdir,
                &tgtdir,
                &name.unwrap_or("".to_string()),
                &tags,
                args.debug,
            ) {
                println!("Error resetting regressions");
            }
        }
        Commands::Diff { name } => {
            if let Err(_) = diff_regressions(
                &srcdir,
                &tgtdir,
                &name.unwrap_or("".to_string()),
                &tags,
                args.debug,
            ) {
                println!("Error diffing regressions");
            }
        }
    }

    tmp_dir.close()?;
    Ok(())
}

/// Lists available regression tests matching the given pattern.
///
/// # Arguments
///
/// * `_source` - Path to the examples directory (unused in listing)
/// * `target` - Path to the regression data directory
/// * `regression_name` - Filter pattern for regression names (empty string matches all)
/// * `tags` - List of tags to filter by (tests must match at least one tag)
/// * `debug` - Enable debug output
///
/// # Errors
///
/// Returns an error if the target directory cannot be read.
///
/// # Examples
///
/// Lists all regressions:
/// ```text
/// Regressions found:
///     basys3_blink
///     basys3_counter
/// ```
fn list_regressions(
    _source: &str,
    target: &str,
    regression_name: &str,
    tags: &[String],
    debug: bool,
) -> Result<(), io::Error> {
    if debug {
        println!("List of regressions matching: \"{}\"", regression_name);
        println!("Filtering by tags: {:?}", tags);
    }

    println!("Regressions found:");
    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // Skip .git directory
        if filename.to_str().unwrap() == ".git" {
            continue;
        }
        // Filter regressions by name pattern
        if filename.to_str().unwrap().contains(regression_name) {
            // Check if regression matches any of the requested tags
            if check_regression_tags(target, filename.to_str().unwrap(), tags, debug) {
                println!("\t{}", filename.to_str().unwrap());
            }
        }
    }

    Ok(())
}

/// Checks if a regression's tags match any of the requested tags.
///
/// # Arguments
///
/// * `target` - Path to the regression data directory
/// * `regression_name` - Name of the regression to check
/// * `requested_tags` - List of tags to match against
/// * `debug` - Enable debug output
///
/// # Returns
///
/// Returns true if the regression has at least one tag that matches the requested tags,
/// or if the regression has no tags defined and "default" is in the requested tags.
fn check_regression_tags(
    target: &str,
    regression_name: &str,
    requested_tags: &[String],
    debug: bool,
) -> bool {
    let config_path = format!("{}/{}/config.yaml", target, regression_name);

    // If config doesn't exist, skip this regression
    if !std::path::Path::new(&config_path).exists() {
        return false;
    }

    // Read and parse the config file
    if let Ok(config_content) = fs::read_to_string(&config_path) {
        if let Ok(parsed_config) = YamlLoader::load_from_str(&config_content) {
            if let Some(config) = parsed_config.get(0) {
                // Get tags from config, default to ["default"] if not present
                let regression_tags = extract_tags_from_config(config);

                if debug {
                    println!(
                        "Regression {} has tags: {:?}",
                        regression_name, regression_tags
                    );
                }

                // Check if any requested tag matches any regression tag
                return requested_tags
                    .iter()
                    .any(|tag| regression_tags.contains(tag));
            }
        }
    }

    false
}

/// Extracts tags from a YAML config, defaulting to ["default"] if not present.
///
/// # Arguments
///
/// * `config` - The parsed YAML configuration
///
/// # Returns
///
/// A vector of tag strings
fn extract_tags_from_config(config: &yaml_rust::Yaml) -> Vec<String> {
    if let Some(tags_yaml) = config["tags"].as_vec() {
        tags_yaml
            .iter()
            .filter_map(|t| t.as_str().map(|s| s.to_string()))
            .collect()
    } else {
        vec!["default".to_string()]
    }
}

/// Describes regression tests by displaying their configuration details.
///
/// # Arguments
///
/// * `_source` - Path to the examples directory (unused in describing)
/// * `target` - Path to the regression data directory
/// * `regression_name` - Filter pattern for regression names (empty string matches all)
/// * `tags` - List of tags to filter by (tests must match at least one tag)
/// * `debug` - Enable debug output
///
/// # Errors
///
/// Returns an error if the target directory cannot be read or if
/// executing a regression description fails.
///
/// # Output Format
///
/// For each matching regression, displays:
/// - Regression name (in green)
/// - regbase: Base directory in examples repository
/// - sourcedata: Path to generated output file
/// - targetdata: Path to expected output file
/// - regcommand: Command to execute
/// - tags: List of tags for this regression
fn describe_regressions(
    _source: &str,
    target: &str,
    regression_name: &str,
    tags: &[String],
    debug: bool,
) -> Result<(), io::Error> {
    if debug {
        println!("Describe regressions matching: \"{}\"", regression_name);
        println!("Filtering by tags: {:?}", tags);
    }

    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // Skip .git directory
        if filename.to_str().unwrap() == ".git" {
            continue;
        }
        // Filter regressions by name pattern and tag
        if filename.to_str().unwrap().contains(regression_name) {
            if check_regression_tags(target, filename.to_str().unwrap(), tags, debug) {
                if let Err(err) =
                    execute_regression("", target, "describe", filename.to_str().unwrap(), debug)
                {
                    println!(
                        "Error describing regression {}: {}",
                        filename.to_str().unwrap(),
                        err
                    );
                }
            }
        }
    }

    Ok(())
}

/// Runs regression tests and compares results against expected outputs.
///
/// # Arguments
///
/// * `source` - Path to the examples directory
/// * `target` - Path to the regression data directory
/// * `regression_name` - Filter pattern for regression names (empty string matches all)
/// * `tags` - List of tags to filter by (tests must match at least one tag)
/// * `debug` - Enable debug output
///
/// # Errors
///
/// Returns an error if the target directory cannot be read or if
/// executing a regression test fails.
///
/// # Output
///
/// For each test:
/// - "Regression `<name>`: passed" (in green) if output matches expected
/// - "Regression `<name>`: failed" (in red) if output differs
fn run_regressions(
    source: &str,
    target: &str,
    regression_name: &str,
    tags: &[String],
    debug: bool,
) -> Result<(), io::Error> {
    if debug {
        println!("Run regressions matching: \"{}\"", regression_name);
        println!("Filtering by tags: {:?}", tags);
    }

    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // Skip .git directory
        if filename.to_str().unwrap() == ".git" {
            continue;
        }
        // Filter regressions by name pattern and tag
        if filename.to_str().unwrap().contains(regression_name) {
            if check_regression_tags(target, filename.to_str().unwrap(), tags, debug) {
                if let Err(err) =
                    execute_regression(source, target, "run", filename.to_str().unwrap(), debug)
                {
                    println!(
                        "Error executing regression {}: {}",
                        filename.to_str().unwrap(),
                        err
                    );
                }
            }
        }
    }

    Ok(())
}

/// Resets regression tests by updating expected outputs with current results.
///
/// This command is useful when the expected output needs to be updated,
/// such as after intentional changes to the tool or examples.
///
/// # Arguments
///
/// * `source` - Path to the examples directory
/// * `target` - Path to the regression data directory
/// * `regression_name` - Filter pattern for regression names (empty string matches all)
/// * `tags` - List of tags to filter by (tests must match at least one tag)
/// * `debug` - Enable debug output
///
/// # Errors
///
/// Returns an error if the target directory cannot be read or if
/// resetting a regression fails.
///
/// # Output
///
/// For each reset test:
/// - "Regression `<name>`: reset" (in yellow)
fn reset_regressions(
    source: &str,
    target: &str,
    regression_name: &str,
    tags: &[String],
    debug: bool,
) -> Result<(), io::Error> {
    if debug {
        println!("Reset regressions matching: \"{}\"", regression_name);
        println!("Filtering by tags: {:?}", tags);
    }

    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // Skip .git directory
        if filename.to_str().unwrap() == ".git" {
            continue;
        }
        // Filter regressions by name pattern and tag
        if filename.to_str().unwrap().contains(regression_name) {
            if check_regression_tags(target, filename.to_str().unwrap(), tags, debug) {
                if let Err(err) =
                    execute_regression(source, target, "reset", filename.to_str().unwrap(), debug)
                {
                    println!(
                        "Error executing regression {}: {}",
                        filename.to_str().unwrap(),
                        err
                    );
                }
            }
        }
    }

    Ok(())
}

/// Shows differences between current and expected regression outputs.
///
/// Uses `sdiff` to display side-by-side comparison of files.
///
/// # Arguments
///
/// * `source` - Path to the examples directory
/// * `target` - Path to the regression data directory
/// * `regression_name` - Filter pattern for regression names (empty string matches all)
/// * `tags` - List of tags to filter by (tests must match at least one tag)
/// * `debug` - Enable debug output
///
/// # Errors
///
/// Returns an error if the target directory cannot be read or if
/// diffing a regression fails.
///
/// # Output
///
/// For each test:
/// - "Regression `<name>`: no differences" (in green) if outputs match
/// - "Regression `<name>`: differences found" (in red) followed by diff output
fn diff_regressions(
    source: &str,
    target: &str,
    regression_name: &str,
    tags: &[String],
    debug: bool,
) -> Result<(), io::Error> {
    if debug {
        println!("Diff regressions matching: \"{}\"", regression_name);
        println!("Filtering by tags: {:?}", tags);
    }

    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // Skip .git directory
        if filename.to_str().unwrap() == ".git" {
            continue;
        }
        // Filter regressions by name pattern and tag
        if filename.to_str().unwrap().contains(regression_name) {
            if check_regression_tags(target, filename.to_str().unwrap(), tags, debug) {
                if let Err(err) =
                    execute_regression(source, target, "diff", filename.to_str().unwrap(), debug)
                {
                    println!(
                        "Error executing regression {}: {}",
                        filename.to_str().unwrap(),
                        err
                    );
                }
            }
        }
    }

    Ok(())
}

/// Executes a single regression test action.
///
/// This is the core function that handles all regression operations.
/// It reads the configuration file, executes the regression command,
/// and performs the requested action (describe, run, reset, or diff).
///
/// # Arguments
///
/// * `source` - Path to the examples directory
/// * `target` - Path to the regression data directory
/// * `action` - The action to perform: "describe", "run", "reset", or "diff"
/// * `regression_name` - Name of the specific regression to execute
/// * `debug` - Enable debug output
///
/// # Configuration File Format
///
/// Each regression has a `config.yaml` file with the following structure:
/// ```yaml
/// regbase: basys3_blink           # Directory name in bmexamples
/// sourcedata: working_dir/output.sv # Generated output file path
/// targetdata: output.sv           # Expected output file path
/// regcommand: make hdl            # Command to execute
/// tags: [default, quick]          # Optional tags (defaults to ["default"])
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The regression directory doesn't exist
/// - The config.yaml file is missing or invalid
/// - The example base directory doesn't exist
/// - The regression command fails
/// - The generated output file is missing
/// - The expected output file is missing (for run/diff/reset)
/// - File operations fail
fn execute_regression(
    source: &str,
    target: &str,
    action: &str,
    regression_name: &str,
    debug: bool,
) -> Result<(), io::Error> {
    if debug {
        println!("Execute regression: \"{}\"", regression_name);
    }

    // Verify regression directory exists
    let regression_dir = format!("{}/{}", target, regression_name);
    if !fs::metadata(&regression_dir).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "getting regression directory failed",
        ));
    }

    // Load configuration file
    let config_path = regression_dir + "/config.yaml";

    if !fs::metadata(&config_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "getting regression configuration file failed",
        ));
    }

    // Parse YAML configuration
    let config_content = fs::read_to_string(&config_path)?;
    let parsed_config = YamlLoader::load_from_str(&config_content);

    if debug {
        println!("Regression configuration:");
        println!("{:?}", parsed_config);
    }

    let config = &parsed_config.unwrap();

    // Extract configuration values
    let regbase = config[0]["regbase"].as_str().unwrap();
    let sourcedata = config[0]["sourcedata"].as_str().unwrap();
    let targetdata = config[0]["targetdata"].as_str().unwrap();
    let regcommand = config[0]["regcommand"].as_str().unwrap();

    // Extract tags using helper function
    let tags = extract_tags_from_config(&config[0]);

    if debug {
        println!("regbase: {}", regbase);
        println!("sourcedata: {}", sourcedata);
        println!("targetdata: {}", targetdata);
        println!("regcommand: {}", regcommand);
        println!("tags: {:?}", tags);
    }

    // For describe action, just print configuration and return
    if action == "describe" {
        println!("Regression: \x1b[0;32m{}\x1b[0m", regression_name);
        println!("  regbase: {}", regbase);
        println!("  sourcedata: {}", sourcedata);
        println!("  targetdata: {}", targetdata);
        println!("  regcommand: {}", regcommand);
        println!("  tags: {:?}", tags);
        return Ok(());
    }

    // Verify example source directory exists
    let examplesource = format!("{}/{}", source, regbase);

    if debug {
        println!("examplesource: {}", examplesource);
    }

    if !fs::metadata(&examplesource).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "getting regression base directory failed",
        ));
    }

    // Execute the regression command in the example directory
    let regcommand = Command::new("sh")
        .current_dir(&examplesource)
        .arg("-c")
        .arg(regcommand)
        .output()?;

    if debug {
        println!("regcommand: {:?}", regcommand);
    }

    if !regcommand.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "executing regression command failed",
        ));
    }

    // Verify the generated output file exists
    let result = format!("{}/{}", examplesource, sourcedata);

    if debug {
        println!("result: {}", result);
    }

    if !fs::metadata(&result).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "getting regression result failed",
        ));
    }

    // Load the generated output
    let result_data = fs::read_to_string(&result)?;

    let regression_dir = format!("{}/{}", target, regression_name);

    // Verify the expected output file exists
    let targetdatafull = format!("{}/{}", regression_dir, targetdata);

    if debug {
        println!("targetdatafull: {}", targetdatafull);
    }

    if !fs::metadata(&targetdatafull).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "getting regression target data directory failed",
        ));
    }

    // Load the expected output
    let target_data = fs::read_to_string(&targetdatafull)?;

    // Perform the requested action
    if action == "run" {
        // Compare generated output with expected output
        if result_data == target_data {
            println!("Regression {}: \x1b[0;32mpassed\x1b[0m", regression_name);
        } else {
            println!("Regression {}: \x1b[0;31mfailed\x1b[0m", regression_name);
        }
    } else if action == "reset" {
        // Update expected output with current generated output
        fs::copy(result, targetdatafull)?;

        println!("Regression {}: \x1b[0;33mreset\x1b[0m", regression_name);
    } else if action == "diff" {
        // Show differences using sdiff
        let diff = Command::new("sdiff")
            .arg("--suppress-common-lines")
            .arg(result)
            .arg(targetdatafull)
            .output()?;

        if debug {
            println!("diff: {:?}", diff);
        }

        if diff.status.success() {
            println!(
                "Regression {}: \x1b[0;32mno differences\x1b[0m",
                regression_name
            );
        } else {
            println!(
                "Regression {}: \x1b[0;31mdifferences found\x1b[0m",
                regression_name
            );
            println!("{}", String::from_utf8_lossy(&diff.stdout));
        }
    }

    Ok(())
}
