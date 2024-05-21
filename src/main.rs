use figlet_rs::FIGfont;
use qbfrt::config::Config;
use qbfrt::db::{save_path, DB};
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

    if let Some(save_path) = config.save_path {
        save_path::change_save_path(&db, save_path, config.verbose).unwrap_or_else(|err| {
            println!("Could not update save paths: {err}");
            process::exit(1);
        });
    }

    Ok(())
}
