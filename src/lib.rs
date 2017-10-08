extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod codec;
pub mod message;

use codec::TplinkSmartHomeCodec;
use message::*;

use std::time::Duration;

use std::net::SocketAddr;

use futures::{Future, Stream, Sink};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;
use tokio_timer::Timer;

fn make_request_wait_for_response(request: Message, device_addr: SocketAddr, wait: Duration, count: u64) -> Message {
    drop(env_logger::init());

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let src: SocketAddr = "0.0.0.0:0".parse().unwrap();

    let sock = UdpSocket::bind(&src, &handle).unwrap();
    sock.set_broadcast(true).unwrap();

    let (mut sink, stream) = sock.framed(TplinkSmartHomeCodec).split();

    let timer = Timer::default();

    let timeout = timer.sleep(wait);

    println!("<<<<<<<<<<<\n{}", serde_json::to_string(&request).unwrap());
    sink.start_send((device_addr,Some(request))).unwrap();

    let mut response = None;

    {
        let stream = stream.take(count).map(|(addr, msg)| {
            println!(">>>>>>>>>>>>\n{}", serde_json::to_string(&msg).unwrap());

            response = Some(msg.unwrap());

            (addr, None)
        });

        let sock = sink.send_all(stream);

        let sock = sock.select2(timeout);

        drop(core.run(sock));
    }

    response.unwrap()
}

pub fn get_sysinfo(device_addr: SocketAddr) {
    let request =
        Message::SystemMsg(
            SystemMsg::GetSysinfo(None)
        );

    make_request_wait_for_response(request, device_addr, Duration::from_secs(3), 1);
}

pub fn get_details(device_addr: SocketAddr) {
    let request =
        Message::LightingServiceMsg(
            LightingServiceMsg::GetLightDetails(None)
        );

    make_request_wait_for_response(request, device_addr, Duration::from_secs(3), 1);
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

    make_request_wait_for_response(request, device_addr, Duration::from_secs(3), 1);
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

    make_request_wait_for_response(request, device_addr, Duration::from_secs(3), 1);
}
