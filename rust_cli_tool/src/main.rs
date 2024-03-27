use anyhow::Result;
use clap::{App, Arg};
use polars::prelude::*;
use std::fs::File;

fn main() -> Result<()> {
    let matches = App::new("CSV Filter with Polars")
        .version("0.1.0")
        .author("Sanjeev")
        .about("Filters rows from a CSV file based on a column value")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input CSV file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("column")
                .short('c')
                .long("column")
                .value_name("COLUMN")
                .help("Specifies the column to filter on")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .value_name("THRESHOLD")
                .help("Sets the threshold for filtering")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let column_name = matches.value_of("column").unwrap();
    let threshold: f64 = matches.value_of("threshold").unwrap().parse()?;

    filter_csv(file_path, column_name, threshold)
}

fn filter_csv(file_path: &str, column_name: &str, threshold: f64) -> Result<()> {
    let file = File::open(file_path)?;
    let df = CsvReader::new(file).has_header(true).finish()?;

    let filtered_df = df
        .lazy()
        .filter(col(column_name).gt(lit(threshold)))
        .collect()?;

    println!("{}", filtered_df);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_csv(content: &str) -> Result<NamedTempFile> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "{}", content)?;
        Ok(temp_file)
    }

    #[test]
    fn test_filter_csv_success() -> Result<()> {
        let temp_file = create_temp_csv("name,age\nAlice,30\nBob,25\nCharlie,35")?;
        let temp_path = temp_file.path().to_str().unwrap();

        filter_csv(temp_path, "age", 29.0)?;

        // Add assertions here to check the filtered data
        // This might involve reading the output back if you modify the function to return data or write to a file
        // For simplicity, this example won't include those steps

        Ok(())
    }

    #[test]
    fn test_filter_csv_no_matches() -> Result<()> {
        let temp_file = create_temp_csv("name,age\nAlice,30\nBob,25\nCharlie,35")?;
        let temp_path = temp_file.path().to_str().unwrap();

        filter_csv(temp_path, "age", 40.0)?;

        // Add assertions here similar to the above

        Ok(())
    }

    #[test]
    fn test_filter_csv_invalid_column() -> Result<()> {
        let temp_file = create_temp_csv("name,age\nAlice,30\nBob,25\nCharlie,35")?;
        let temp_path = temp_file.path().to_str().unwrap();

        let result = filter_csv(temp_path, "height", 29.0);

        assert!(result.is_err());

        Ok(())
    }
}
