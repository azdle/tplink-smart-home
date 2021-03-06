extern crate tplink_smart_home;
extern crate clap;

fn main() {
    use std::io::{self, Write};
    use std::process::exit;

    match app() {
        Ok(()) => (),
        Err(e) => {
            writeln!(io::stderr(), "error: {}", e).unwrap();
            exit(1);
        }
    }
}

fn app() -> Result<(), std::io::Error> {
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
        .subcommand(SubCommand::with_name("temp")
                    .about("set bulb color using color temperature")
                    .arg(Arg::with_name("TEMPERATURE")
                         .required(true))
                    .arg(Arg::with_name("BRIGHTNESS")
                         .required(true)))
        .subcommand(SubCommand::with_name("circadian")
                    .about("set bulb to circadian mode"))
        .get_matches();

    let device_addr_str = matches.value_of("DEVICE").unwrap();
    let device_port_str = matches.value_of("port").unwrap_or("9999");

    let device_addr = SocketAddr::new(device_addr_str.parse().unwrap(), device_port_str.parse().unwrap());

    if let Some(_matches) = matches.subcommand_matches("on") {
        tplink_smart_home::on(device_addr)?;
    } else if let Some(_matches) = matches.subcommand_matches("off") {
        tplink_smart_home::off(device_addr)?;
    } else if let Some(matches) = matches.subcommand_matches("hsv") {
        let h = matches.value_of("HUE").unwrap().parse().unwrap();
        let s = matches.value_of("SATURATION").unwrap().parse().unwrap();
        let v = matches.value_of("VALUE").unwrap().parse().unwrap();

        tplink_smart_home::hsv(device_addr, h, s, v)?;
    } else if let Some(matches) = matches.subcommand_matches("temp") {
        let t = matches.value_of("TEMPERATURE").unwrap().parse().unwrap();
        let b = matches.value_of("BRIGHTNESS").unwrap().parse().unwrap();

        tplink_smart_home::temp(device_addr, t, b)?;
    } else if let Some(_matches) = matches.subcommand_matches("circadian") {
        tplink_smart_home::circadian(device_addr)?;
    } else if let Some(_matches) = matches.subcommand_matches("sysinfo") {
        tplink_smart_home::get_sysinfo(device_addr)?;
    } else {
        tplink_smart_home::get_details(device_addr)?;
    }

    Ok(())
}

