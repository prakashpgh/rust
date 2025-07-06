use csv::{ErrorKind, Reader, StringRecord}; // Import ErrorKind
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path; // To work with file paths more robustly

pub fn read_securities(file_path: &Path) -> Result<Vec<entities::Security>, Box<dyn Error>> {
    let csv_content = "ticker,fin_type,geo,geo_2\nn1,1,1,1\nn2,1,1,1\nn3,1,1,1\nn4,1,1,1";
    std::fs::write("products.csv", csv_content)?;

    let file = File::open("products.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut securities = Vec::new();
    // Iterate over deserialized records
    for result in rdr.deserialize() {
        let security: entities::Security = result?; // Deserialize into our Person struct
        securities.push(security);
        //println!("{:?}", security);
    }

    Ok(securities)
}

// --- Custom Error Type for Single Records (no change) ---
#[derive(Debug)]
enum RecordParseError {
    MissingField(String),
    ParseFloatError {
        field_name: String,
        value: String,
        source: std::num::ParseFloatError,
    },
}

impl fmt::Display for RecordParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordParseError::MissingField(field_name) => {
                write!(f, "Missing field: '{}'", field_name)
            }
            RecordParseError::ParseFloatError {
                field_name,
                value,
                source,
            } => {
                write!(
                    f,
                    "Failed to parse field '{}' ('{}'): {}",
                    field_name, value, source
                )
            }
        }
    }
}

impl Error for RecordParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RecordParseError::ParseFloatError { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn extract_double(
    record: &StringRecord,
    index: usize,
    name: &str,
) -> Result<f32, RecordParseError> {
    let s = record
        .get(index)
        .ok_or_else(|| RecordParseError::MissingField(name.to_string()))?;
    //let mut processed_s = s.trim();
    let mut processed_s = s.replace("$", "");
    processed_s = processed_s.replace("+", "");
    processed_s = processed_s.replace("-", "");
    processed_s = processed_s.replace("%", "");
    processed_s = processed_s.replace("(", "");
    processed_s = processed_s.replace(")", "");
    processed_s = processed_s.replace(" ", "");
    processed_s = processed_s.replace(",", "");
    //println!("{} => {}", s, processed_s);
    let val = processed_s
        .parse::<f32>()
        .map_err(|e| RecordParseError::ParseFloatError {
            field_name: name.to_string(),
            value: processed_s.to_string(),
            source: e,
        })?;
    Ok(val)
}

/// Extracts 'name' and 'city' from a CSV record given their respective column indices.
/// Returns `Some(PersonSpecificInfo)` if both fields are found, otherwise `None`.
fn extract_security_info(
    portfolio: &str,
    record: &StringRecord,
    ticker_idx: usize,
    market_value_idx: usize,
    today_pl_index: usize,
    today_percent_pl_index: usize,
    pl_index: usize,
    percent_pl_index: usize,
) -> Result<entities::SecuritySpecificInfo, RecordParseError> {
    let ticker = record
        .get(ticker_idx)
        .ok_or_else(|| RecordParseError::MissingField("ticker".to_string()))?;
    let market_value: Option<f32> = Some(extract_double(record, market_value_idx, "market_value")?);
    let today_pl: Option<f32> = Some(extract_double(record, today_pl_index, "today_pl")?);
    let today_percent_pl: Option<f32> = Some(extract_double(record, today_percent_pl_index, "today_percent_pl")?);
    let pl: Option<f32> = Some(extract_double(record, pl_index, "pl")?);
    let percent_pl: Option<f32> = Some(extract_double(record, percent_pl_index, "percent_pl")?);

    Ok(entities::SecuritySpecificInfo {
        portfolio: portfolio.to_string(),
        ticker: ticker.to_string(),
        market_value,
        today_pl,
        today_percent_pl,
        pl,
        percent_pl
    })
}

pub fn read_csv(
    portfolio: &str,
    file_path: &str,
) -> Result<Vec<entities::SecuritySpecificInfo>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    // Get the header record to find column indices by name (optional, but good practice)
    let headers = rdr.headers()?.clone();
    let ticker_index = headers
        .iter()
        .position(|h| h == "Symbol")
        .ok_or_else(|| format!("Column 'Symbol' not found in {}", file_path))?;
    let market_value_index = headers
        .iter()
        .position(|h| h == "Current Value" || h == "Mkt Val (Market Value)")
        .ok_or_else(|| format!("Column 'market value' not found in {}", file_path))?;
    let today_pl_index = headers
        .iter()
        .position(|h| h == "Today's Gain/Loss Dollar" || h == "Day Chng $ (Day Change $)")
        .ok_or_else(|| {
            format!(
                "Column 'Today's Gain/Loss Dollar' not found in {}",
                file_path
            )
        })?;
    let today_percent_pl_index = headers
        .iter()
        .position(|h| h == "Day Chng % (Day Change %)" || h == "Today's Gain/Loss Percent")
        .ok_or_else(|| {
            format!(
                "Column 'Today's Gain/Loss Percent' not found in {}",
                file_path
            )
        })?;
    let pl_index = headers
        .iter()
        .position(|h| h == "Total Gain/Loss Dollar" || h == "Gain $ (Gain/Loss $)")
        .ok_or_else(|| format!("Column 'Today_pl' not found in {}", file_path))?;
    let percent_pl_index = headers
        .iter()
        .position(|h| h == "Total Gain/Loss Percent" || h == "Gain % (Gain/Loss %)")
        .ok_or_else(|| {
            format!(
                "Column 'Total Gain/Loss Percent' not found in {}",
                file_path
            )
        })?;

    let mut securities_data: Vec<entities::SecuritySpecificInfo> = Vec::new();
    let mut record_num = 0;
    for result in rdr.records() {
        record_num += 1;
        match result {
            Ok(record) => {
                match extract_security_info(
                    &portfolio,
                    &record,
                    ticker_index,
                    market_value_index,
                    today_pl_index,
                    today_percent_pl_index,
                    pl_index,
                    percent_pl_index,
                ) {
                    Ok(security_info) => {
                        securities_data.push(security_info); // Add the parsed struct to the vector
                    }
                    Err(e) => {
                        // This record was problematic; log a warning but continue processing others.
                        eprintln!("Warning: Failed to extract or parse all required fields (ticker, quantity) from record: {:?}", record);
                    }
                };
            }
            Err(e) => {
                // This 'e' is a csv::Error
                if let ErrorKind::UnequalLengths { .. } = *e.kind() {
                    eprintln!("Error processing record {} (CSV format error - unequal lengths): {}. Skipping record.", record_num, e);
                    // We skip this record and continue to the next one
                    continue;
                } else {
                    eprint!("error processing record {} {}", record_num, e);
                    return Err(Box::new(e));
                }
            }
        }
    }
    //println!("csv processing complete");
    Ok(securities_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
