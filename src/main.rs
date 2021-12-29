mod port_status;
mod arguments;
mod port_range;


use port_range::PortRange;
use port_status::PortStatus;
use arguments::Arguments;
use structopt::StructOpt;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use loading::Loading;

fn main() {
    let arguments = Arguments::from_args();

    let (tx, rx) = std::sync::mpsc::channel::<PortStatus>();
    let port_range = Arc::new(arguments.port_range);
    for i in 0..arguments.threads {
        let tx = tx.clone(); 
        let port_range = port_range.clone();
        std::thread::spawn(move || {
            check_port_range(tx, i, arguments.threads, &arguments.ip_address, port_range);
        });
    }
    drop(tx);

    let mut loading = Loading::new();
    loading.start();
    let mut done = 0;
    for i in rx {
        done += 1;
        loading.text(format!("{}/{}", done, port_range.get_port_count()));
        match i {
            PortStatus::Open(p) => println!("{} is open", p),
            PortStatus::Close(_) => (),
        }
    }

    loading.success("Finished");
    loading.end();
    thread::sleep(std::time::Duration::from_millis(10));
}

fn check_port_range(tx: Sender<PortStatus>, index: u16, threads: u16, ipaddr: &IpAddr, port_range: Arc<PortRange>) {
    let port_count = port_range.get_port_count();
    match &*port_range {
        PortRange::Single(p) => {
            if index != 0 {
                return;
            }
            check_port(&tx, *p, ipaddr);
        },
        PortRange::Range((start, end)) => {
            let section: usize = (port_count as f64 / threads as f64).ceil() as usize;
            (*start..*end).skip(section * index as usize).take(section).for_each(|n| {
                check_port(&tx, n, ipaddr);
            });
        }
        PortRange::Multi(ports) => {
            let section: usize = (port_count as f64 / threads as f64).ceil() as usize;
            ports.iter().skip(section * index as usize).take(section).for_each(|n| {
                check_port(&tx, *n, ipaddr);
            });
        }
    }
}

fn check_port(tx: &Sender<PortStatus>, port: u16, ipaddr: &IpAddr) {
    match std::net::TcpStream::connect(format!("{}:{}", ipaddr.to_string(), port)) {
        Ok(_) => tx.send(PortStatus::Open(port)).unwrap(),
        Err(_) => tx.send(PortStatus::Close(port)).unwrap(),
    }
}