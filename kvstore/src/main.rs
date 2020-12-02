fn main() {
    let mut arguments = std::env::args().skip(1);
    let _key = arguments.next().unwrap();
    let _value = arguments.next().unwrap();
    println!("Key: {} Value: {}", _key, _value);
    let write_result = write_database(_key, _value);
    match write_result {
        Ok(()) => {
            print!("Done");
        }
        Err(this_error) => {
            print!("Error {}", this_error)
        }
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    let contents = format!("{} {}", key, value);
    return std::fs::write("kv.db", contents);
    
}
