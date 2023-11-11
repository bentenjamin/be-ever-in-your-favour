use std::io;
mod odd;
use odd::odd::get_odd;


fn main() {
    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        match buffer.as_ref() {
            "q" => return,
            _ => println!("{}", get_odd(buffer))
        }
    }
}