extern crate futures;
extern crate tokio_core;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod codec;
pub mod message;

use codec::TplinkSmartHomeCodec;
use message::*;

use std::net::SocketAddr;

use futures::{Stream, Sink};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

pub fn request() {
    drop(env_logger::init());

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let src: SocketAddr = "0.0.0.0:8675".parse().unwrap();
    let dest: SocketAddr = "192.168.1.145:9999".parse().unwrap();

    let sock = UdpSocket::bind(&src, &handle).unwrap();

    let (mut sink, stream) = sock.framed(TplinkSmartHomeCodec).split();

    let _request =
        Message::LightingServiceMsg(
            LightingServiceMsg::TransitionLightState(
                TransitionLightState{
                    on_off: 1,
                }
            )
        );
    let request =
        Message::LightingServiceMsg(
            LightingServiceMsg::GetLightDetails(None)
        );

    println!("<<<<<<<<<<<\n{}", serde_json::to_string(&request).unwrap());
    sink.start_send((dest,Some(request))).unwrap();

    let stream = stream.take(1).map(|(addr, request)| {
        println!(">>>>>>>>>>>>\n{}", serde_json::to_string(&request).unwrap());

        (addr, None)
    });

    let sock = sink.send_all(stream);
    drop(core.run(sock));
}
