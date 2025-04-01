use std::env;
use std::net IpAddr;

struct Arguments {
    flag: String,
    ipadd: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'statis str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        else
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
