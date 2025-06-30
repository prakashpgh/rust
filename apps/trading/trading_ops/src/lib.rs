use dblib;

pub fn load_securities() {
    // 1. Create a new DatabaseManager instance
    let db_manager = dblib::DatabaseManager::new(r"C:\tools\trading\trading.db");
    let securities = db_manager.expect("error in getting all securities").get_all_securities();
   // println!("number of securities: {}", securities.expect("exception in getting securities").len());
    for s in securities {
        println!("{:?}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
