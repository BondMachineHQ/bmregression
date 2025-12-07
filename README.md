# bmregression

A command-line regression testing tool for the BondMachine project.

## Overview

`bmregression` is a Rust-based tool designed to automate regression testing for BondMachine examples. It manages test execution, output comparison, and reporting across multiple example projects. The tool works by executing build commands in example directories and comparing the generated outputs against expected results stored in a separate data repository.

## Architecture

The tool operates on two repositories:

1. **[bmexamples](https://github.com/BondMachineHQ/bmexamples)**: Contains example projects with:
   - Source code (e.g., `.basm` files)
   - Build configurations (`Makefile`, `.config`)
   - Hardware constraints (`.xdc` files)
   - Mapping configurations (`.json` files)

2. **[bmregressiondata](https://github.com/BondMachineHQ/bmregressiondata)**: Stores regression test data with:
   - Test configuration files (`config.yaml`)
   - Expected output files (e.g., `bondmachine.sv`)

### Repository Structure

```
bmexamples/
├── basys3_blink/          # Example project directory
│   ├── blink.basm         # Source code
│   ├── Makefile           # Build configuration
│   └── ...
└── basys3_counter/
    └── ...

bmregressiondata/
├── basys3_blink/          # Regression test directory
│   ├── config.yaml        # Test configuration
│   └── bondmachine.sv     # Expected output
└── basys3_counter/
    └── ...
```

## Prerequisites

- **Rust toolchain**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: Required for cloning repositories
- **sdiff**: Used for showing differences (typically pre-installed on Linux/macOS)
- **BondMachine tools** (optional): If running actual tests, you'll need the BondMachine toolchain

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/BondMachineHQ/bmregression.git
cd bmregression

# Build the project
cargo build --release

# The binary will be available at:
# target/release/bmregression
```

### Add to PATH (Optional)

```bash
# Copy to a directory in your PATH
sudo cp target/release/bmregression /usr/local/bin/

# Or add the target directory to your PATH
export PATH="$PATH:$(pwd)/target/release"
```

## Usage

The tool provides several commands for managing regression tests:

### Basic Syntax

```bash
bmregression [OPTIONS] <COMMAND>
```

### Commands

#### 1. List Available Regressions

List all available regression tests or filter by name:

```bash
# List all regressions
bmregression list

# List regressions matching a pattern
bmregression list basys3
```

**Example output:**
```
Regressions found:
	basys3_blink
	basys3_counter
	basys3_led_on_off_shell
```

#### 2. Describe Regressions

Display configuration details for regression tests:

```bash
# Describe all regressions
bmregression describe

# Describe specific regression
bmregression describe basys3_blink
```

**Example output:**
```
Regression: basys3_blink
  regbase: basys3_blink
  sourcedata: working_dir/bondmachine.sv
  targetdata: bondmachine.sv
  regcommand: make hdl
```

#### 3. Run Regressions

Execute regression tests and compare outputs:

```bash
# Run all regressions
bmregression run

# Run specific regression
bmregression run basys3_blink

# Run with debug output
bmregression --debug run basys3_blink
```

**Example output:**
```
Regression basys3_blink: passed
Regression basys3_counter: failed
```

#### 4. Reset Regressions

Update expected outputs with current results (use after intentional changes):

```bash
# Reset all regressions
bmregression reset

# Reset specific regression
bmregression reset basys3_blink
```

**Example output:**
```
Regression basys3_blink: reset
```

#### 5. Diff Regressions

Show detailed differences between current and expected outputs:

```bash
# Diff all regressions
bmregression diff

# Diff specific regression
bmregression diff basys3_blink
```

**Example output:**
```
Regression basys3_blink: differences found
[diff output showing line-by-line differences]
```

### Global Options

- `--debug` or `-d`: Enable debug output showing detailed execution steps
- `--data-dir <PATH>`: Use local regression data directory instead of cloning
- `--examples-dir <PATH>`: Use local examples directory instead of cloning
- `--data-url <URL>`: Custom URL for regression data repository
- `--examples-url <URL>`: Custom URL for examples repository
- `--system-tools` or `-s`: Use system-installed tools instead of official sources

### Configuration File Format

Each regression test requires a `config.yaml` file with the following structure:

```yaml
regbase: basys3_blink              # Directory name in bmexamples repository
sourcedata: working_dir/bondmachine.sv  # Path to generated output file
targetdata: bondmachine.sv         # Path to expected output file in regression data
regcommand: make hdl               # Command to execute to generate output
```

**Field descriptions:**
- `regbase`: The example project directory name in the bmexamples repository
- `sourcedata`: Relative path to the generated output file within the example directory
- `targetdata`: Filename of the expected output in the regression data directory
- `regcommand`: Shell command to execute in the example directory to generate output

## Examples

### Example 1: Run a Single Test with Local Repositories

If you have already cloned the repositories:

```bash
bmregression \
  --examples-dir ~/projects/bmexamples \
  --data-dir ~/projects/bmregressiondata \
  run basys3_blink
```

### Example 2: Run All Tests with Debug Output

```bash
bmregression --debug run
```

This will:
1. Create a temporary directory
2. Clone both repositories
3. Execute all regression tests
4. Display detailed debug information
5. Clean up temporary files

### Example 3: Update Test Expectations

After making intentional changes to the examples:

```bash
# Run tests to see what changed
bmregression diff basys3_blink

# If changes are correct, update expectations
bmregression reset basys3_blink
```

### Example 4: Working with a Subset of Tests

```bash
# List all tests containing "basys3"
bmregression list basys3

# Run only those tests
bmregression run basys3
```

## Development

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

### Project Structure

```
bmregression/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
├── LICENSE             # License information
└── src/
    └── main.rs         # Main source code with all functionality
```

## How It Works

### Test Execution Flow

1. **Initialization**
   - Parse command-line arguments
   - Create temporary working directory
   - Clone or reference repositories

2. **Test Discovery**
   - Scan regression data directory for test configurations
   - Filter tests based on name pattern (if provided)

3. **Test Execution** (for each matching test)
   - Read `config.yaml` to get test parameters
   - Navigate to corresponding example directory
   - Execute the configured command
   - Capture the generated output

4. **Comparison/Action**
   - **Run**: Compare generated output with expected output
   - **Reset**: Copy generated output to expected output location
   - **Diff**: Show side-by-side differences
   - **Describe**: Display configuration details

5. **Cleanup**
   - Remove temporary directories (if repositories were cloned)

### Color-Coded Output

The tool uses ANSI color codes for better readability:
- 🟢 **Green**: Passed tests, no differences
- 🔴 **Red**: Failed tests, differences found
- 🟡 **Yellow**: Reset operations

## Troubleshooting

### Common Issues

1. **"Error cloning examples repository"**
   - Check internet connection
   - Verify Git is installed
   - Check repository URLs are accessible

2. **"Error executing regression command failed"**
   - Ensure BondMachine tools are installed
   - Check that example project has all required dependencies
   - Run with `--debug` to see detailed error messages

3. **"getting regression base directory failed"**
   - Verify the `regbase` in `config.yaml` matches an actual directory in bmexamples
   - Check for typos in the configuration

4. **Missing sdiff command**
   - Install diffutils package: `apt-get install diffutils` (Ubuntu/Debian)
   - Or use alternative diff tools by modifying the source

## Contributing

Contributions are welcome! Please ensure your code:
- Follows Rust best practices and idioms
- Includes appropriate documentation comments
- Passes `cargo fmt` and `cargo test`
- Maintains minimal changes philosophy

## License

See the [LICENSE](LICENSE) file for details.

## Related Projects

- [BondMachine](https://github.com/BondMachineHQ/BondMachine) - The main BondMachine project
- [bmexamples](https://github.com/BondMachineHQ/bmexamples) - Example projects repository
- [bmregressiondata](https://github.com/BondMachineHQ/bmregressiondata) - Regression test data repository
