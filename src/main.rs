extern crate futures;
extern crate tokio_core;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::net::SocketAddr;

use futures::{Stream, Sink};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

use std::io;
use std::option::Option;

use tokio_core::net::UdpCodec;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    #[serde(rename = "smartlife.iot.smartbulb.lightingservice")]
    LightingServiceMsg(LightingServiceMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LightingServiceMsg {
    #[serde(rename = "transition_light_state")]
    TransitionLightState(TransitionLightState),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightState {
    on_off: u8,
    mode: String,
    hue: u16,
    saturation: u8,
    color_temp: u8,
    brightness: u8,
    err_code: u8,
}

pub struct TplinkSmartHomeCodec;

impl UdpCodec for TplinkSmartHomeCodec {
    type In = (SocketAddr, Option<Message>);
    type Out = (SocketAddr, Option<Message>);

    // TODO: Should probably make a custom Reader/Writer to avoid allocation.
    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        let mut decoded = Vec::with_capacity(buf.len());
        let mut last: u8 = 0xAB; // protocol defined 'key'
        for b in buf {
            decoded.push(b ^ last);
            last = b ^ last;
        }
        match serde_json::from_slice(&decoded) {
            Ok(msg) => Ok((*addr, Some(msg))),
            Err(_) => Ok((*addr, None)),
        }
    }

    fn encode(&mut self, (addr, mmsg): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        if let Some(msg) = mmsg {
            let string = serde_json::to_string(&msg).unwrap();
            println!("Sending:\n{}", string);
            let bytes = serde_json::to_vec(&msg).unwrap();
            println!("Bytes: {:?}", bytes);
            let mut encoded = Vec::with_capacity(bytes.len());
            let mut last: u8 = 0xAB; // protocol defined 'key'
            for b in bytes {
                encoded.push(b ^ last);
                last = b ^ last;
            }
            println!("Encoded: {:?}", encoded);
            into.extend(encoded);
        };

        addr
    }
}

fn main() {
    drop(env_logger::init());

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let src: SocketAddr = "0.0.0.0:8675".parse().unwrap();
    let dest: SocketAddr = "192.168.1.145:9999".parse().unwrap();

    let sock = UdpSocket::bind(&src, &handle).unwrap();

    let (mut sink, stream) = sock.framed(TplinkSmartHomeCodec).split();

    let request =
        Message::LightingServiceMsg(
            LightingServiceMsg::TransitionLightState(
                TransitionLightState{
                    on_off: 0,
                    mode: "normal".into(),
                    hue: 300,
                    saturation: 100,
                    color_temp: 0,
                    brightness: 6,
                    err_code: 0,
                }
            )
        );

    sink.start_send((dest,Some(request))).unwrap();

    let stream = stream.take(1).map(|(addr, request)| {
        println!("--> {:?}", request);

        (addr, None)
    });

    let sock = sink.send_all(stream);
    drop(core.run(sock));
}
