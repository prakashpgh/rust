use entities;
use rusqlite::{params, Connection, Result, Row};

/// A struct to manage database operations for the 'people' table.
pub struct DatabaseManager {
    conn: Connection,
}

impl DatabaseManager {
    /// Establishes a connection to the SQLite database and creates the 'people' table.
    ///
    /// # Arguments
    /// * `db_path` - The path to the SQLite database file.
    ///
    /// # Returns
    /// A `Result` containing a new `DatabaseManager` instance or an error.
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        println!("Successfully connected to {}.", db_path);
        Ok(DatabaseManager { conn })
    }

    pub fn select_security_by_ticker(&self, ticker: &str) -> Result<entities::Security> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, ticker, fin_type, geo, geo_2 FROM Security WHERE ticker = ?1")?;
        let security = stmt.query_row(params![ticker], entities::Security::from_row)?;
        Ok(security)
    }

    pub fn get_all_securities(&self) -> Result<Vec<entities::Security>> {
        let mut securities = Vec::new();
        let mut stmt = self
            .conn
            .prepare("SELECT id, ticker, fin_type, geo, geo_2 FROM Security")?;

        // Use query_map to iterate over rows and map them to Security objects
        let security_iter = stmt.query_map([], entities::Security::from_row)?;

        // Iterate over the results and push them into the vector
        for security_result in security_iter {
            securities.push(security_result?); // Push the successfully parsed Security
        }
        Ok(securities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
