use std::net::IpAddr;
use structopt::StructOpt;

use super::port_range::PortRange;

#[derive(StructOpt, Debug)]
#[structopt(name = "port-sniffer")]
pub struct Arguments {
    #[structopt(short, long)]
    pub ip_address: IpAddr,
    #[structopt(short, long, default_value ="8")]
    pub threads: u16,
    #[structopt(short, long)]
    pub port_range: PortRange,
}
