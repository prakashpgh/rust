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

    pub fn insert_security(
        &self,
        security: &entities::Security,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = self.conn.execute(
            "insert into security(ticker, fin_type, geo, geo_2) values(?1, ?2, ?3, ?4)",
            params![
                security.ticker,
                security.fin_type,
                security.geo,
                security.geo_2
            ], // Parameters go here
        )?;
        Ok(())
    }

    pub fn delete_security(&self, ticker: &String) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = self.conn.execute(
            "delete from security where ticker = ?1",
            params![ticker], // Parameters go here
        )?;
        Ok(())
    }

    pub fn delete_all_security(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = self.conn.execute("delete from security", [])?;
        Ok(())
    }

    pub fn delete_pf_data(&self, ticker: &String) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = self.conn.execute(
            "delete from pf_holding where security_id = (select security_id from security where ticker = ?1)",
            params![ticker], // Parameters go here
        )?;
        Ok(())
    }

    pub fn delete_all_pf_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = self.conn.execute("delete from pf_holding", [])?;
        Ok(())
    }

    pub fn insert_pf_holding(
        &self,
        security: &entities::SecuritySpecificInfo,
        sec_id: i32,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        println!("insert {:?} {:?}", security, sec_id);
        let rows_affected = self.conn.execute(
            "insert into pf_holding(pf_id, security_id, market_value, today_pl, today_percent_pl, pl, percent_pl) values(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                security.portfolio,
                sec_id,
                security.market_value,
                security.today_pl,
                security.today_percent_pl,
                security.pl,
                security.percent_pl
            ], // Parameters go here
        )?;
        Ok(rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
