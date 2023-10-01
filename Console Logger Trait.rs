pub trait Logger {
    fn log( message : &str ) {
        println!("{}",message);
    }
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {}

fn main() {
    ConsoleLogger::log("this is a console message");
}