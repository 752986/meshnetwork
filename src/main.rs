use std::{
    collections::HashMap,
    io::{read_to_string, BufRead, BufReader, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

fn main() {
    let listener: TcpListener =
        TcpListener::bind("0.0.0.0:42069").expect("unable to bind listener");

    loop {
        match listener.accept() {
            Ok((mut client, ip)) => {
                println!("accepted incoming connection from {ip}");
                client.write_all(b"hello from declan!").unwrap();
                let mut reader = BufReader::new(&client);
                loop {
                    let mut response = String::new();
                    match reader.read_line(&mut response) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => print!("{}", &response),
                    }
                }
                println!("disconnected from {ip}");
            }
            Err(ip) => {
                println!("unable to connect to client at {ip}");
            }
        }
    }
}

type Id = u32;

struct Peer {
    stream: TcpStream,
}

impl Peer {
    fn send(&mut self, data: PacketData) {
        // turn the given packetdata into bytes and send it to the peer
        todo!()
        // self.stream.write_all(data);
    }
}

struct Packet<'a> {
    from: Peer,
    data: PacketData<'a>,
}

enum PacketData<'a> {
    /// A notification that a new piece of data is available
    Available { id: Id },
    /// A request for data with the given id
    Request { id: Id },
    /// A piece of data
    Data { id: Id, data: &'a [u8] },
}

struct Server {
    seen_data: HashMap<Id, Vec<u8>>,
    connections: Vec<Peer>,
}

impl Server {
    fn handleIncomingPacket(&mut self, mut packet: Packet) {
        match packet.data {
            PacketData::Request { id } => {
                if self.seen_data.contains_key(&id) {
                    packet.from.send(PacketData::Data {
                        id,
                        data: &self.seen_data[&id],
                    })
                }
            }
            PacketData::Available { id } => {
                if !(self.seen_data.contains_key(&id)) {
                    packet.from.send(PacketData::Request { id })
                }
            }
            PacketData::Data { id, data } => {
                self.seen_data.insert(id, data.to_vec());
            }
        }
    }
}
