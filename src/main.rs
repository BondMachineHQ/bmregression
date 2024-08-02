extern crate tempdir;
use clap::{Parser, Subcommand};

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
                ::std::process::exit(1);
            }
        }
        Commands::Describe { name } => {
            if let Err(_) = describe_regression(&name.unwrap_or("".to_string()), args.debug) {
                ::std::process::exit(1);
            }
        }
        Commands::Run { name } => {
            if let Err(_) = execute_regression(&name.unwrap_or("".to_string()), args.debug) {
                ::std::process::exit(1);
            }
        }
        Commands::Reset { name } => {
            if let Err(_) = reset_regression(&name.unwrap_or("".to_string()), args.debug) {
                ::std::process::exit(1);    
            }
        }
        Commands::Diff { name } => {
            if let Err(_) = diff_regression(&name.unwrap_or("".to_string()), args.debug) {
                ::std::process::exit(1);    
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
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}

fn describe_regression( regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Describe regressions matching: \"{}\"", regression_name);
    }
    Ok(())
}

fn reset_regression( regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Reset regressions matching: \"{}\"", regression_name);
    }
    Ok(())
}

fn diff_regression( regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Diff regressions matching: \"{}\"", regression_name);
    }
    Ok(())
}

fn execute_regression( regression_name: &str, debug: bool) -> Result<(), io::Error> {
    if debug {
        println!("Execute regressions matching: \"{}\"", regression_name);
    }
    Ok(())
}