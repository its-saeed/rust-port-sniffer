mod port_status;
mod arguments;


use port_status::PortStatus;
use arguments::Arguments;
use std::env;
use std::net::IpAddr;
use std::sync::mpsc::Sender;
use loading::Loading;

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

    let (tx, rx) = std::sync::mpsc::channel::<PortStatus>();
    for i in 0..arguments.threads {
        let tx = tx.clone(); 
        std::thread::spawn(move || {
            check_port(tx, i, arguments.threads, &arguments.ip_address);
        });
    }

    let mut loading = Loading::new();
    loading.start();
    let mut done = 0;
    for i in rx {
        done += 1;
        loading.text(format!("{}/{}", done, 65536));
        match i {
            PortStatus::Open(p) => println!("{} is open", p),
            PortStatus::Close(_) => (),
        }
    }

    loading.success("Finished");
    loading.end();
}

fn check_port(tx: Sender<PortStatus>, index: u16, threads: u16, ipaddr: &IpAddr) {
    let section: usize = (65536_usize / threads as usize).into();
    (0..=65535).skip(section * index as usize).take(section).for_each(|n| {
        match std::net::TcpStream::connect(format!("{}:{}", ipaddr.to_string(), n)) {
            Ok(_) => tx.send(PortStatus::Open(n)).unwrap(),
            Err(_) => tx.send(PortStatus::Close(n)).unwrap(),
        }
    });
}