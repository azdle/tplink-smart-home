extern crate tplink_smart_home;
extern crate clap;

fn main() {
    use std::net::SocketAddr;
    use clap::{App, Arg, SubCommand};
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("DEVICE")
             .help("the IP address of a device")
             .required(true)
             .index(1))
        .subcommand(SubCommand::with_name("on")
                    .about("turn on bulb"))
        .subcommand(SubCommand::with_name("off")
                    .about("turn on bulb"))
        .get_matches();

    let device_addr_str = matches.value_of("DEVICE").unwrap();
    let device_port_str = matches.value_of("port").unwrap_or("9999");

    let device_addr = SocketAddr::new(device_addr_str.parse().unwrap(), device_port_str.parse().unwrap());

    if let Some(_matches) = matches.subcommand_matches("on") {
        tplink_smart_home::on(device_addr);
    } else if let Some(_matches) = matches.subcommand_matches("off") {
        tplink_smart_home::off(device_addr);
    } else {
        tplink_smart_home::get_details(device_addr);
    }
}

