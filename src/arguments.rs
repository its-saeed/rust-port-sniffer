use std::str::FromStr;
use std::net::IpAddr;

#[derive(Debug)]
pub struct Arguments {
    pub flag: String,
    pub ip_address: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        match IpAddr::from_str(&args[1]) {
            Ok(ipaddr) => {
                return Ok(Arguments{
                    ip_address: ipaddr,
                    threads: 1,
                    flag: String::from("")
                })
            }
            Err(_) => {
                let flag = &args[1];
                if flag.contains("-h") || flag.contains("-help") {
                    println!("Usage: -j to select how many threads you want\r\n
                    -h or -help to show this message");
            
                    return Err("help");
                } else if flag.contains("-j") {
                    let threads: u16 = match args[2].parse() {
                        Ok(t) => t,
                        Err(_) => return Err("Failed to parse thread number."),
                    };
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(i) => i,
                        Err(_) => return Err("not valid ip address")
                    };
                    return Ok(Arguments{ip_address: ipaddr, threads, flag: flag.clone()})
                }
            }
        }
        return Err("Invalid")
    }
}