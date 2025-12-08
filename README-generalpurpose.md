# bmregression - General Purpose Regression Testing Tool

A command-line regression testing tool for any project that generates file outputs that can be compared over time.

> **Note**: This tool was originally developed for BondMachine testing. For BondMachine-specific usage, see [README.md](README.md).

## Overview

`bmregression` is a Rust-based tool designed to automate regression testing for any software project that generates files as output. It manages test execution, output comparison, and reporting across multiple test cases. The tool works by executing commands in project directories and comparing the generated outputs against expected results.

## How It Works

The tool operates on two directory structures:

1. **Examples/Test Cases Directory**: Contains your test projects with:
   - Source code files
   - Build configurations (e.g., `Makefile`, scripts)
   - Any necessary input files
   - Configuration files

2. **Regression Data Directory**: Stores regression test data with:
   - Test configuration files (`config.yaml`)
   - Expected output files

### Directory Structure

```
your-examples/                     # Test cases directory
├── test_case_1/                   # Individual test case
│   ├── src/                       # Source files
│   ├── Makefile                   # Build configuration
│   └── config.json                # Test-specific config
└── test_case_2/
    └── ...

your-regression-data/              # Regression data directory
├── test_case_1/                   # Regression test directory
│   ├── config.yaml                # Test configuration
│   └── expected_output.txt        # Expected output file
└── test_case_2/
    └── ...
```

## Installation

### Prerequisites

- **Rust toolchain**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: Required if using remote repositories
- **sdiff**: Used for showing differences (typically pre-installed on Linux/macOS)

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

## Setting Up Your Project

### 1. Create Test Configuration Files

For each test case, create a `config.yaml` file in your regression data directory:

```yaml
regbase: test_case_1                    # Directory name in examples repository
sourcedata: output/result.txt           # Path to generated output file
targetdata: expected_result.txt         # Expected output filename in regression data
regcommand: make build && ./run_test    # Command to generate output
tags: [default, quick]                  # Optional: Tags for filtering
```

### 2. Example Configurations for Different Project Types

#### C/C++ Project
```yaml
regbase: my_cpp_project
sourcedata: build/output.log
targetdata: expected_output.log
regcommand: make clean && make test
tags: [default, cpp]
```

#### Python Project
```yaml
regbase: my_python_script
sourcedata: results/analysis.csv
targetdata: expected_analysis.csv
regcommand: python analyze.py --output results/analysis.csv
tags: [default, python]
```

#### Compiler/Code Generator
```yaml
regbase: compiler_test_1
sourcedata: generated/output.c
targetdata: expected_output.c
regcommand: ./my_compiler input.src --output generated/
tags: [default, compiler]
```

#### Data Processing Pipeline
```yaml
regbase: data_pipeline
sourcedata: processed/final_report.json
targetdata: expected_report.json
regcommand: ./process_data.sh input.csv
tags: [default, data]
```

## Usage

### Basic Syntax

```bash
bmregression [OPTIONS] <COMMAND>
```

### Commands

#### 1. List Available Tests

```bash
# List all regression tests
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             list

# List tests matching a pattern
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             list my_project
```

#### 2. Describe Test Configurations

```bash
# Describe all tests
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             describe

# Describe specific test
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             describe my_test_case
```

#### 3. Run Regression Tests

```bash
# Run all tests
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             run

# Run specific test
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             run my_test_case

# Run with debug output
bmregression --debug \
             --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             run my_test_case
```

#### 4. Update Expected Outputs

After making intentional changes that modify outputs:

```bash
# Reset specific test expectations
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             reset my_test_case

# Reset all test expectations
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             reset
```

#### 5. Show Differences

```bash
# Show differences for all tests
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             diff

# Show differences for specific test
bmregression --examples-dir /path/to/your/examples \
             --data-dir /path/to/your/regression-data \
             diff my_test_case
```

### Using Git Repositories

If your examples and regression data are in Git repositories:

```bash
# The tool can automatically clone repositories
bmregression --examples-url https://github.com/yourorg/your-examples.git \
             --data-url https://github.com/yourorg/your-regression-data.git \
             run
```

### Working with Tags

Tags help organize and selectively run groups of tests:

```bash
# Run only quick tests
bmregression --examples-dir /path/to/examples \
             --data-dir /path/to/data \
             --tag quick \
             run

# Run tests with multiple tags
bmregression --examples-dir /path/to/examples \
             --data-dir /path/to/data \
             --tag quick,smoke \
             run
```

## Common Use Cases

### 1. Compiler Testing

Test that your compiler generates the same output for given inputs:

```yaml
# config.yaml for each test case
regbase: compiler_test_basic_arithmetic
sourcedata: generated/output.asm
targetdata: expected_basic_arithmetic.asm
regcommand: ./my_compiler tests/basic_arithmetic.src
tags: [default, compiler, basic]
```

### 2. Data Analysis Tools

Verify that data processing scripts produce consistent results:

```yaml
# config.yaml
regbase: analysis_sales_data
sourcedata: reports/monthly_summary.json
targetdata: expected_monthly_summary.json
regcommand: python analyze_sales.py --month 2024-01
tags: [default, analysis]
```

### 3. Code Generators

Test code generation tools:

```yaml
# config.yaml
regbase: api_generator_rest
sourcedata: generated/api_client.py
targetdata: expected_api_client.py
regcommand: ./generate_api.sh config/rest_api.yaml
tags: [default, generator]
```

### 4. Build System Testing

Test build outputs:

```yaml
# config.yaml
regbase: build_system_test
sourcedata: dist/bundle.js
targetdata: expected_bundle.js
regcommand: npm run build
tags: [default, build]
```

## Configuration File Reference

### Required Fields

- `regbase`: Directory name in your examples repository
- `sourcedata`: Path to the generated output file (relative to example directory)
- `targetdata`: Filename of expected output in regression data directory
- `regcommand`: Command to execute to generate the output

### Optional Fields

- `tags`: List of tags for categorizing tests (defaults to `["default"]`)

### Complete Example

```yaml
regbase: my_complex_test
sourcedata: output/results/final.json
targetdata: expected_final.json
regcommand: |
  ./setup.sh &&
  make clean &&
  make all &&
  ./run_analysis --config test.conf
tags: [default, integration, slow]
```

## Command Line Options

### Global Options

- `--debug` or `-d`: Enable debug output
- `--examples-dir <PATH>`: Path to local examples directory
- `--data-dir <PATH>`: Path to local regression data directory
- `--examples-url <URL>`: Git URL for examples repository
- `--data-url <URL>`: Git URL for regression data repository
- `--system-tools` or `-s`: Use system-installed tools
- `--tag <TAGS>` or `-t <TAGS>`: Filter by tags (comma-separated)

## Best Practices

### 1. Organizing Tests

- Use descriptive directory names for test cases
- Group related tests using tags
- Keep test cases focused and independent

### 2. Managing Expected Outputs

- Use `bmregression reset` after intentional changes
- Review diffs carefully before resetting expectations
- Version control your regression data repository

### 3. Test Design

- Make tests deterministic (avoid timestamps, random data)
- Use relative paths in configurations
- Keep generated outputs in predictable locations

### 4. Continuous Integration

Example GitHub Actions workflow:

```yaml
name: Regression Tests
on: [push, pull_request]

jobs:
  regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build bmregression
        run: |
          git clone https://github.com/BondMachineHQ/bmregression.git
          cd bmregression
          cargo build --release
          
      - name: Run regression tests
        run: |
          ./bmregression/target/release/bmregression \
            --examples-dir ./examples \
            --data-dir ./regression-data \
            run
```

## Troubleshooting

### Common Issues

1. **Command execution fails**
   - Verify your `regcommand` works when run manually
   - Check that all dependencies are installed
   - Use `--debug` flag to see detailed error messages

2. **File not found errors**
   - Verify `sourcedata` path is correct relative to example directory
   - Ensure `regcommand` actually generates the expected file
   - Check file permissions

3. **Directory structure issues**
   - Ensure `regbase` matches actual directory names
   - Check that both examples and data directories exist
   - Verify directory structure matches expectations

### Debug Mode

Use `--debug` to see detailed execution information:

```bash
bmregression --debug --examples-dir ./examples --data-dir ./data run
```

## Contributing

This tool is open source and contributions are welcome! The main repository is at [BondMachineHQ/bmregression](https://github.com/BondMachineHQ/bmregression).

## License

See the [LICENSE](LICENSE) file for details.