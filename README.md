# Rusty Port Sniffer
Port sniffer command line interface using only the standard library of Rust

# Usage
port-sniffer 0.1.0

* USAGE:

    rust-port-sniffer.exe [OPTIONS] --ip-address <ip-address> --port-range <port-range>

* FLAGS:

    * -h, --help       Prints help information
    * -V, --version    Prints version information

* OPTIONS:

    * -i, --ip-address <ip-address>
    * -p, --port-range <port-range>
        * single port: 80
        * range: 10-20
        * multiport: 1,2,3,4,5
    * -t, --threads <threads>           [default: 8]