use figlet_rs::FIGfont;
use qbfrt::config::Config;
use qbfrt::db::DB;
use qbfrt::fastresume_db;
use serde_rusqlite::from_rows;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", FIGfont::standard().unwrap().convert("qbfrt").unwrap());

    let config = Config::build().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    DB::backup(&config).unwrap_or_else(|err| {
        println!("Could not backup database: {err}");
        process::exit(1);
    });

    let db = DB::connect(&config).unwrap_or_else(|err| {
        println!("Could not connect to database: {err}");
        process::exit(1);
    });

    // temp query to show we can do things!
    let mut stmt = db.prepare("SELECT * FROM torrents")?;
    let rows = from_rows::<fastresume_db::DatabaseData>(stmt.query([])?);
    for row in rows {
        let test = row?;
        println!("{:?}", test.torrent_id);
    }

    Ok(())
}
