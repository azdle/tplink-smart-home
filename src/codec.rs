use message::*;

use std::net::SocketAddr;

use std::io;
use std::option::Option;

use tokio_core::net::UdpCodec;

use serde_json;

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
            last = *b;
        }

        match serde_json::from_slice(&decoded) {
            Ok(msg) => Ok((*addr, Some(msg))),
            Err(_) => Ok((*addr, None)),
        }
    }

    fn encode(&mut self, (addr, mmsg): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        if let Some(msg) = mmsg {

            let bytes = serde_json::to_vec(&msg).unwrap();

            let mut encoded = Vec::with_capacity(bytes.len());
            let mut last: u8 = 0xAB; // protocol defined 'key'
            for b in bytes {
                encoded.push(b ^ last);
                last = b ^ last;
            }

            into.extend(encoded);
        };

        addr
    }
}

