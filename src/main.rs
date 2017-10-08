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
        .subcommand(SubCommand::with_name("sysinfo")
                    .about("get device sysinfo"))
        .subcommand(SubCommand::with_name("on")
                    .about("turn on bulb"))
        .subcommand(SubCommand::with_name("off")
                    .about("turn on bulb"))
        .subcommand(SubCommand::with_name("hsv")
                    .about("set bulb color using HSV color space")
                    .arg(Arg::with_name("HUE")
                         .required(true))
                    .arg(Arg::with_name("SATURATION")
                         .required(true))
                    .arg(Arg::with_name("VALUE")
                         .required(true)))
        .get_matches();

    let device_addr_str = matches.value_of("DEVICE").unwrap();
    let device_port_str = matches.value_of("port").unwrap_or("9999");

    let device_addr = SocketAddr::new(device_addr_str.parse().unwrap(), device_port_str.parse().unwrap());

    if let Some(_matches) = matches.subcommand_matches("on") {
        tplink_smart_home::on(device_addr);
    } else if let Some(_matches) = matches.subcommand_matches("off") {
        tplink_smart_home::off(device_addr);
    } else if let Some(matches) = matches.subcommand_matches("hsv") {
        let h = matches.value_of("HUE").unwrap().parse().unwrap();
        let s = matches.value_of("SATURATION").unwrap().parse().unwrap();
        let v = matches.value_of("VALUE").unwrap().parse().unwrap();

        tplink_smart_home::hsv(device_addr, h, s, v);
    } else if let Some(_matches) = matches.subcommand_matches("sysinfo") {
        tplink_smart_home::get_sysinfo(device_addr);
    } else {
        tplink_smart_home::get_details(device_addr);
    }
}

