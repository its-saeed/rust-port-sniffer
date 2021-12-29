use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc::Sender;

#[derive(Debug)]
struct Arguments {
    flag: String,
    ip_address: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            std::process::exit(0);
        }else {
            eprintln!("{} problem parsing arugment: {}", args[0], err);
            std::process::exit(-1);
        }
    });

    let (tx, rx) = std::sync::mpsc::channel::<u16>();
    for i in 0..arguments.threads {
        let tx = tx.clone(); 
        std::thread::spawn(move || {
            check_port(tx, i, arguments.threads, &arguments.ip_address);
        });
    }

    for i in rx {
        println!("{} is open", i);
    }
}

fn check_port(tx: Sender<u16>, index: u16, threads: u16, ipaddr: &IpAddr) {
    let section: usize = (65536_usize / threads as usize).into();
    (0..=65535).skip(section * index as usize).take(section).for_each(|n| {
        match std::net::TcpStream::connect(format!("{}:{}", ipaddr.to_string(), n)) {
            Ok(_) => tx.send(n).unwrap(),
            Err(_) => (),
        }
    });
}