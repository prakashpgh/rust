use rusqlite::Row;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Security {
    pub id: i64,
    pub ticker: String,
    pub fin_type: i32,
    pub geo: i32,
    pub geo_2: Option<i32>,
}

impl Security {
    pub fn from_row(row: &Row) -> Result<Security, rusqlite::Error> {
        Ok(Security {
            id: row.get(0)?,
            ticker: row.get(1)?,
            fin_type: row.get(2)?,
            geo: row.get(3)?,
            geo_2: row.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
