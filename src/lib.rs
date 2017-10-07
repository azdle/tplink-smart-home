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

fn make_request_wait_for_response(request: Message, device_addr: SocketAddr) -> Message {
    drop(env_logger::init());

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let src: SocketAddr = "0.0.0.0:0".parse().unwrap();

    let sock = UdpSocket::bind(&src, &handle).unwrap();
    sock.set_broadcast(true).unwrap();

    let (mut sink, stream) = sock.framed(TplinkSmartHomeCodec).split();

    println!("<<<<<<<<<<<\n{}", serde_json::to_string(&request).unwrap());
    sink.start_send((device_addr,Some(request))).unwrap();

    let mut response = None;

    {
        let stream = stream.take(1).map(|(addr, msg)| {
            println!(">>>>>>>>>>>>\n{}", serde_json::to_string(&msg).unwrap());

            response = Some(msg.unwrap());

            (addr, None)
        });

        let sock = sink.send_all(stream);
        drop(core.run(sock));
    }

    response.unwrap()
}

pub fn get_sysinfo(device_addr: SocketAddr) {
    let request =
        Message::SystemMsg(
            SystemMsg::GetSysinfo(None)
        );

    make_request_wait_for_response(request, device_addr);
}

pub fn get_details(device_addr: SocketAddr) {
    let request =
        Message::LightingServiceMsg(
            LightingServiceMsg::GetLightDetails(None)
        );

    make_request_wait_for_response(request, device_addr);
}

pub fn on(device_addr: SocketAddr) {
    let request =
        Message::LightingServiceMsg(
            LightingServiceMsg::TransitionLightState(
                TransitionLightState::TransitionLightOnOff(
                    TransitionLightOnOff{
                        on_off: 1,
                    }
                )
            )
        );

    make_request_wait_for_response(request, device_addr);
}

pub fn off(device_addr: SocketAddr) {
    let request =
        Message::LightingServiceMsg(
            LightingServiceMsg::TransitionLightState(
                TransitionLightState::TransitionLightOnOff(
                    TransitionLightOnOff{
                        on_off: 0,
                    }
                )
            )
        );

    make_request_wait_for_response(request, device_addr);
}
