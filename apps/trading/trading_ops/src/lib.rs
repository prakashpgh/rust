use csv;
use dblib;
use entities;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn load_securities() -> Result<(), Box<dyn std::error::Error>> {
    //read from the file...
    let path_buf = PathBuf::from("./report.txt");
    println!("PathBuf 1: {:?}", path_buf);
    let mut file_securities: HashMap<String, entities::Security> = HashMap::new();
    let securities_from_csv = csv::read_securities(path_buf.as_path()).expect(&format!(
        "Failed to read securities from {:?}. Panicking!",
        path_buf
    ));

    for r in securities_from_csv {
        file_securities.insert(r.ticker.clone(), r);
    }
    println!("Loaded {} securities from CSV.", file_securities.len());

    // 1. Create a new DatabaseManager instance
    let db_manager = dblib::DatabaseManager::new(r"C:\tools\trading\trading.db")?;
    let securities_from_db = db_manager.get_all_securities()?;
    println!(
        "Loaded {} securities from Database.",
        securities_from_db.len()
    );

    let mut db_securities: HashMap<String, entities::Security> = HashMap::new();
    for s in securities_from_db {
        db_securities.insert(s.ticker.clone(), s);
        //println!("{:?}", s);
    }

    //check if it needs insert or update
    for (kf, vf) in &file_securities {
        if db_securities.contains_key(kf) {
            //update
            db_manager.delete_security(kf);
            db_manager.insert_security(vf);
            //println!("update: {}", kf);
        } else {
            //insert
            println!("insert: {}", kf);
            db_manager.insert_security(vf);
        }
    }

    // If everything above succeeded, return Ok(())
    Ok(())
}

pub fn read_csvs() {
    let path_buf = PathBuf::from("./report.txt");
    println!("PathBuf 1: {:?}", path_buf);
    let mut file_securities: HashMap<String, entities::Security> = HashMap::new();
    let securities_from_csv = csv::read_securities(path_buf.as_path()).expect(&format!(
        "Failed to read securities from {:?}. Panicking!",
        path_buf
    ));

    for r in securities_from_csv {
        file_securities.insert(r.ticker.clone(), r);
    }
    println!("Loaded {} securities from CSV.", file_securities.len());
}

fn delete_lines_until_symbol<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    let path = file_path.as_ref();
    // Create a temporary file to write the modified content
    let temp_path = path.with_extension("tmp");

    let input_file = File::open(path)?;
    let reader = BufReader::new(input_file);

    let output_file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut found_symbol = false;

    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors reading lines
        let mut lower_case = line.to_lowercase();

        if !found_symbol {
            if lower_case.contains("symbol") {
                found_symbol = true;
                // If "symbol" is found, include this line and all subsequent lines
                writeln!(writer, "{}", line)?;
            }
            // If "symbol" not yet found, and current line doesn't contain it, skip this line
        } else {
            // Once "symbol" is found, write all subsequent lines
            writeln!(writer, "{}", line)?;
        }
    }

    // Ensure all buffered data is written to the temporary file
    writer.flush()?;

    // Replace the original file with the temporary file
    fs::rename(&temp_path, path)?;

    Ok(())
}

pub fn read_csv_from_site(
) -> Result<HashMap<(String, String), entities::SecuritySpecificInfo>, Box<dyn std::error::Error>> {
    //files that we have to process
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(
        "fid".to_string(),
        r"C:\tools\trading\files\fidelity.csv".to_string(),
    );
    map.insert(
        "ch".to_string(),
        r"C:\tools\trading\files\charles.csv".to_string(),
    );

    let mut security_data: HashMap<(String, String), entities::SecuritySpecificInfo> =
        HashMap::new();
    //process the files.
    for (key, value) in map {
        let file_path = value;
        //delete lines in file till we find symbol.. thats the header
        delete_lines_until_symbol(&file_path).unwrap();

        //push the securities data into vector.
        match csv::read_csv(&key, &file_path) {
            Ok(securities_data) => {
                for data in securities_data {
                    // Borrow for initial print
                    //println!("ticker: {}, quantity: {}, today_pl: {:?}, today_percent_pl: {:?}, pl: {:?}, percent_pl: {:?} ", data.ticker, data.quantity, data.today_pl, data.today_percent_pl, data.pl, data.percent_pl);
                    security_data.insert((key.clone(), data.ticker.clone()), data);
                }
            }
            Err(e) => {
                eprint!("error {}", e);
            }
        }
        println!("successfully processed {}", &file_path);
    }

    //println!("\n--- All processed security data ---");
    //for ((broker, ticker), info) in &security_data {
    //    println!("Broker: {}, Ticker: {}, Info: {:?}", broker, ticker, info);
    //}
    Ok(security_data)
}

pub fn process_holdings() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a new DatabaseManager instance
    let db_manager = dblib::DatabaseManager::new(r"C:\tools\trading\trading.db")?;
    //delete all data.
    db_manager.delete_all_pf_data();
    db_manager.delete_all_security();

    //read positions from the files.
    let security_data = read_csv_from_site()?;

    //println!("\n--- All processed security data ---");
    //for ((broker, ticker), info) in &security_data {
    //    println!("Broker: {}, Ticker: {}, Info: {:?}", broker, ticker, info);
    //}

    println!("Loaded {} securities from CSV.", security_data.len());

    let securities_from_db = db_manager.get_all_securities()?;
    println!(
        "Before Loaded {} securities from Database.",
        securities_from_db.len()
    );

    let mut db_securities_map: HashMap<String, entities::Security> = HashMap::new();
    for s in securities_from_db {
        db_securities_map.insert(s.ticker.clone(), s);
        //println!("{:?}", s);
    }

    //check if it needs insert or update
    let mut inserts: i32 = 0;
    let mut updates: i32 = 0;
    for (kf, vf) in &security_data {
        if db_securities_map.contains_key(&(kf.1)) {
            //update
            let sec = entities::Security {
                id: Some(-1),
                ticker: vf.ticker.clone(),
                fin_type: 1,
                geo: 1,
                geo_2: Some(-1),
            };
            //db_manager.delete_security(&(kf.1));
            //db_manager.insert_security(&sec);
            //updates += 1;
            //println!("updated: {:?}", kf);
        } else {
            //insert
            println!("insert: {:?}", kf);
            let sec = entities::Security {
                id: Some(-1),
                ticker: vf.ticker.clone(),
                fin_type: 1,
                geo: 1,
                geo_2: Some(-1),
            };
            db_manager.insert_security(&sec);
            inserts += 1;
        }
    }
    println!("Inserts {} updates {}", inserts, updates);

    let securities_from_db = db_manager.get_all_securities()?;
    println!(
        "After Loaded {} securities from Database.",
        securities_from_db.len()
    );

    let mut db_securities_map: HashMap<String, entities::Security> = HashMap::new();
    for s in securities_from_db {
        db_securities_map.insert(s.ticker.clone(), s);
        //println!("{:?}", s);
    }

    println!("inserting the pf holdings: {}", security_data.len());
    //insert holdings.
    for (kf, vf) in &security_data {
        let mut sec_id = -1;
        //println!("inserting the pf holdings: {}", &(kf.1));
        if db_securities_map.contains_key(&(kf.1)) {
            sec_id = db_securities_map
                .get(&(kf.1))
                .unwrap()
                .id
                .expect("error while getting the id");
        } else {
            println!("securityID not found {}", &(kf.1));
        }
        let inserted = db_manager.insert_pf_holding(vf, sec_id)?;
        println!("{:?}", vf);
        if inserted == 1 {
            println!("inserted");
        } else {
            println!("NOT inserted");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
