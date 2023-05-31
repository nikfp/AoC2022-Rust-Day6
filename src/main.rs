use std::process;

fn main() {
    let path = get_path().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let contents = get_contents(&path).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
}
