use etherparse::InternetSlice::{Ipv4, Ipv6};
use etherparse::{Icmpv4Slice, SlicedPacket, TcpHeader, TcpHeaderSlice};
use etherparse::TransportSlice::{Icmpv4, Icmpv6, Tcp, Udp, Unknown};
use pcap::{Activated, Device, Capture, PacketHeader};

fn main() {
    let main_device = Device::list().unwrap();
    let device = main_device.get(0).unwrap().clone();
    let mut cap = Capture::from_device(device).unwrap()
        .promisc(true)
        .snaplen(5000)
        .open().unwrap();

    // while let Ok(packet) = cap.next() {
    //     println!("received packet! {:?}", packet);
    // }
    read_packets(cap)
}

fn read_packets<T: Activated>(mut capture: Capture<T>) {
    while let Ok(packet) = capture.next() {
        // let header = packet.header.clone();
        match SlicedPacket::from_ethernet(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                // println!("link: {:?}", value.link);
                // println!("vlan: {:?}", value.vlan);
                // println!("ip: {:?}", value.ip);
                // println!("transport: {:?}", value.transport);

                //per operazioni su livello 3
                // match value.ip {
                //     Some(val) => {
                //         let copy_val = val.clone();
                //         match copy_val {
                //             Ipv4(header, extension) =>
                //                 println!("{:?} and {:?}", String::from("IPV4"), header),
                //             Ipv6(header, ..) =>
                //                 println!("{:?}", String::from("IPV6")),
                //         }
                //
                //         // match TcpHeader::from_slice(&packet) {
                //         //     Err(value) => {},
                //         //     Ok(result) => {
                //         //         println!("Tcp: {:?}/n Data: {:?}", result.0, result.1)
                //         //     }
                //         // }
                //     }
                //     _ => {}
                // }

                //per operazioni su livello 4
                match value.transport {
                    Some(val) => {
                        let copy_val = val.clone();
                        match copy_val {
                            Udp(header_slice) => println!("{:?} and {:?}", String::from("UDP"), header_slice.to_header()),
                            Tcp(header_slice) => println!("{:?} and {:?}", String::from("TCP"), header_slice.to_header()),
                            Icmpv4(slice) => println!("{:?} and {:?}", String::from("Icmpv4"), slice.header()),
                            Icmpv6(slice) => println!("{:?} and {:?}", String::from("Icmpv6"), slice.header()),
                            Unknown(val) => println!("{:?}", String::from("Unknown"))
                        }
                    }
                    _ => {}
                }
            }
        }

        // let result = match header {
        //     PacketHeader::Ipv4(_) => String::from("Ipv4"),
        //     PacketHeader::Ipv6(_) => String::from("Ipv6")
        // };
    }
}

// impl ToString for PacketHeader {
//     fn to_string(&self) -> String {
//         match self {
//             PacketHeader::Ipv4(_) => String::from("Ipv4"),
//             PacketHeader::Ipv6(_) => String::from("Ipv6")
//         }
//     }
// }

// pub fn parse_ipv4(
//     content: &[u8],
//     parsed_packet: &mut ParsedPacket,
// ) -> Result<(), String> {
//     match ipv4::parse_ipv4_header(content) {
//         Ok((content, IPv4Header)) => {
//             self.parse_transport_layer(&IPv4Header.protocol, content, parsed_packet)?;
//             parsed_packet.headers.push(PacketHeader::Ipv4(IPv4Header));
//             Ok(())
//         }
//         Err(err) => {
//             parsed_packet.remaining = content.to_owned();
//             Err("Error".to_string())
//         }
//     }
// }