use std::{env, error::Error, fs};


pub fn get_path() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("File path not provided");
    }

    Ok(args.get(1).unwrap().to_string())
}

pub fn get_contents(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}



