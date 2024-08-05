extern crate tempdir;
use clap::{Parser, Subcommand};
use yaml_rust::{YamlLoader};

use std::fs;
use std::io::{self};
use std::process::Command;
use tempdir::TempDir;

/// bmregression is a tool to run the regression tests for the bmexamples repository
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
    #[clap(long, default_value = "https://github.com/BondMachineHQ/bmexamples.git")]
    examples_url: String,
    /// Data repository URL
    #[clap(long, default_value = "https://github.com/BondMachineHQ/bmregressiondata.git")]
    data_url: String,
    /// Use the tools in the system instead of the ones installed from the official sources
    #[clap(short, long, default_value = "false")]
    system_tools: bool,
}

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

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    // Check if the command is set
    if args.command.is_none() {
        println!("No command specified");
        ::std::process::exit(1);
    }

    let tmp_dir = TempDir::new("bmregression")?;
    if args.debug {
        println!("Working directory: {}", tmp_dir.path().display());
    }

    let mut srcdir = args.examples_dir.clone();
    if args.examples_dir.is_empty() {
        // Clone the examples repository from github
        let clone_dir = tmp_dir.path().join("examples");
        let clone_url = args.examples_url;
        if args.debug {
            println!("Cloning examples repository from: {} to {}", clone_url, clone_dir.display());
        }

        // Execute the git clone command
        let git_clone = Command::new("git")
            .arg("clone")
            .arg(clone_url)
            .arg(clone_dir)
            .output()?;
        if !git_clone.status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Error cloning examples repository"));
        }
        srcdir = tmp_dir.path().join("examples").to_str().unwrap().to_string();
    }

    let mut tgtdir = args.data_dir;
    if tgtdir.is_empty() {
        // Clone the regression data repository from github
        let clone_dir = tmp_dir.path().join("regressiondata");
        let clone_url = args.data_url;
        if args.debug {
            println!("Cloning regression data repository from: {} to {}", clone_url, clone_dir.display());
        }

        // Execute the git clone command
        let git_clone = Command::new("git")
            .arg("clone")
            .arg(clone_url)
            .arg(clone_dir)
            .output()?;
        if !git_clone.status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Error cloning regression data repository"));
        }
        tgtdir = tmp_dir.path().join("regressiondata").to_str().unwrap().to_string();
    }


    // Process the command
    match args.command.unwrap() {
        Commands::List { name } => {
            if let Err(_) = list_regressions(&srcdir, &tgtdir, &name.unwrap_or("".to_string()), args.debug) {
            }
        }
        Commands::Describe { name } => {
            if let Err(_) = describe_regressions(&name.unwrap_or("".to_string()), args.debug) {
            }
        }
        Commands::Run { name } => {
            if let Err(err) = run_regressions(&srcdir, &tgtdir, &name.unwrap_or("".to_string()), args.debug) {
                println!("Error executing regression: {}", err);
            }
        }
        Commands::Reset { name } => {
            if let Err(_) = reset_regressions(&srcdir, &tgtdir, &name.unwrap_or("".to_string()), args.debug) {
            }
        }
        Commands::Diff { name } => {
            if let Err(_) = diff_regressions(&name.unwrap_or("".to_string()), args.debug) {
            }
        }
    }

    tmp_dir.close()?;
    Ok(())

}

// override the tools installation (uses the tools in the system)

fn list_regressions(_source: &str, target: &str, regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("List of regressions matching: \"{}\"", regression_name);
    }

    // List the regressions
    println!("Regressions found:");
    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // filter out the files that do not match the pattern
        if filename.to_str().unwrap().contains(regression_name) {
            println!("\t{}", filename.to_str().unwrap());
        }
        
    }

    Ok(())
}

fn describe_regressions( regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Describe regressions matching: \"{}\"", regression_name);
    }
    Ok(())
}

fn run_regressions(source: &str, target: &str, regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Run regressions matching: \"{}\"", regression_name);
    }

    // run the regressions
    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // filter out the files that do not match the pattern
        if filename.to_str().unwrap().contains(regression_name) {
            if let Err(err) = execute_regression(source, target, "run", filename.to_str().unwrap(), debug) {
                println!("Error executing regression {}: {}", filename.to_str().unwrap(), err);
            }
        }
    }

    Ok(())
}

fn reset_regressions(source: &str, target: &str, regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Reset regressions matching: \"{}\"", regression_name);
    }

    // reset the regressions
    let entries = fs::read_dir(target)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        // filter out the files that do not match the pattern
        if filename.to_str().unwrap().contains(regression_name) {
            if let Err(err) = execute_regression(source, target, "reset", filename.to_str().unwrap(), debug) {
                println!("Error executing regression {}: {}", filename.to_str().unwrap(), err);
            }
        }
    }

    Ok(())
}

fn diff_regressions( regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Diff regressions matching: \"{}\"", regression_name);
    }
    Ok(())
}

fn execute_regression(source: &str, target: &str, action: &str, regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Execute regression: \"{}\"", regression_name);
    }

    // Check if the regression exists in the target directory
    let regression_dir = format!("{}/{}", target, regression_name);
    if !fs::metadata(&regression_dir).is_ok() {
        return Err(io::Error::new(io::ErrorKind::Other, "getting regression directory failed"));
    }

    // Read the regression file configuration
    let regression_config = regression_dir + "/config.yaml";

    // Check if the regression configuration file exists
    if !fs::metadata(&regression_config).is_ok() {
        return Err(io::Error::new(io::ErrorKind::Other, "getting regression configuration file failed"));
    }

    // Read the regression configuration file and parse it
    let regression_config = fs::read_to_string(&regression_config)?;
    let regression_config = yaml_rust::YamlLoader::load_from_str(&regression_config);

    // Show the regression configuration
    if debug {
        println!("Regression configuration:");
        println!("{:?}", regression_config);
    }

    let config = &regression_config.unwrap();

    let regbase = config[0]["regbase"].as_str().unwrap();
    let sourcedata = config[0]["sourcedata"].as_str().unwrap();
    let targetdata = config[0]["targetdata"].as_str().unwrap();
    let regcommand = config[0]["regcommand"].as_str().unwrap();

    if debug {
        println!("regbase: {}", regbase);
        println!("sourcedata: {}", sourcedata);
        println!("targetdata: {}", targetdata);
        println!("regcommand: {}", regcommand);
    }

    // Check if the regression base directory exists
    let examplesource = format!("{}/{}", source, regbase);

    if debug {
        println!("examplesource: {}", examplesource);
    }

    if !fs::metadata(&examplesource).is_ok() {
        return Err(io::Error::new(io::ErrorKind::Other, "getting regression base directory failed"));
    }

    // Execute the regression command
    let regcommand = regcommand.split_whitespace().collect::<Vec<&str>>();
    let regcommand = Command::new(regcommand[0])
        .current_dir(&examplesource)
        .args(&regcommand[1..])
        .output()?;

    if debug {
        println!("regcommand: {:?}", regcommand);
    }

    if !regcommand.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "executing regression command failed"));
    }

    // Check if the result exists
    let result = format!("{}/{}", examplesource, sourcedata);

    if debug {
        println!("result: {}", result);
    }

    if !fs::metadata(&result).is_ok() {
        return Err(io::Error::new(io::ErrorKind::Other, "getting regression result failed"));
    }

    // Load the result
    let result_data = fs::read_to_string(&result)?;

    let regression_dir = format!("{}/{}", target, regression_name);

    // Check if the target data directory exists
    let targetdatafull = format!("{}/{}", regression_dir, targetdata);

    if debug {
        println!("targetdatafull: {}", targetdatafull);
    }

    if !fs::metadata(&targetdatafull).is_ok() {
        return Err(io::Error::new(io::ErrorKind::Other, "getting regression target data directory failed"));
    }

    // Load the target data
    let target_data = fs::read_to_string(&targetdatafull)?;

    if action == "run" {
        // Compare the result with the target data
        if result_data == target_data {
            println!("Regression {}: \x1b[0;32mpassed\x1b[0m", regression_name);
        } else {
            println!("Regression {}: \x1b[0;31mfailed\x1b[0m", regression_name);
        }
    } else if action == "reset" {
        // Copy the result to the target data
        fs::copy(result, targetdatafull)?;

        println!("Regression {}: \x1b[0;33mreset\x1b[0m", regression_name);
    }

    Ok(())
}