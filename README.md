
# CSV Filter with Polars

A simple command-line tool built in Rust, utilizing the Polars library to filter rows from a CSV file based on a specified column's value exceeding a given threshold. This project showcases efficient data processing and error handling in Rust, leveraging the `anyhow` and `clap` crates for an improved developer and user experience.

## Features

- Filters a CSV file to only include rows where the specified column's value exceeds a given numeric threshold.
- Utilizes `clap` for easy command-line argument parsing.
- Implements error handling using `anyhow` for clearer error messages and propagation.
- Includes basic unit tests to validate functionality.

## Usage

To use this tool, you must specify three command-line arguments:

- `-f` or `--file`: The path to the input CSV file.
- `-c` or `--column`: The name of the column to filter on.
- `-t` or `--threshold`: The numeric threshold for filtering.

Example command:

```bash
cargo run -- -f path/to/data.csv -c age -t 30
```

This command filters the `data.csv` file, returning rows where the value in the `age` column is greater than 30.

## Building from Source

Ensure you have Rust installed on your machine. Then, clone this repository, navigate to the project directory, and build the project using Cargo:

```bash
cargo build --release
```

## Running Unit Tests

This project includes unit tests for the `filter_csv` function to ensure its correct operation under various conditions, including successful filtering, no matches, and invalid column names. To run these tests, execute the following command in the project directory:

```bash
cargo test
```

The tests utilize temporary CSV files created at runtime to verify the filtering logic, ensuring robustness and correctness of the tool.

## Dependencies

- `polars`: For efficient data processing and filtering.
- `clap`: For parsing command-line arguments.
- `anyhow`: For simplified error handling.
- `tempfile`: Used in tests for creating temporary CSV files.

Ensure all dependencies are added to your `Cargo.toml` file to build and run the project successfully.
