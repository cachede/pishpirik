use std::{collections::HashMap, net::UdpSocket};

use crate::ecs;

type Entities = HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>;

pub struct Player {
    pub address: std::net::SocketAddr,
    pub input_buffer: HashMap<u8, bool>
}

pub struct GameServer {
    pub socket: UdpSocket,
    pub players: Vec<Player>
}

impl GameServer {
    pub fn new(bind_addr: &'static str) -> std::io::Result<Self> {
        println!("starting server");
        let socket = UdpSocket::bind(bind_addr)?;
        socket.set_nonblocking(true)?;
        return Ok(GameServer {
            socket: socket,
            players: Vec::new()
        });
    }

    pub fn add_player(&mut self, address: std::net::SocketAddr) -> std::io::Result<()> {
        let mut input_buffer = HashMap::new();
        for key in 1..=4 {
            input_buffer.insert(key, false);
        }

        self.players.push(Player {
            address: address,
            input_buffer: input_buffer
        });

        let player_id = (self.players.len() - 1) as u8;
        self.socket.send_to(&[player_id], address)?;
        Ok(())
    }

    // entities werden als json enkodierung an den client gesendet
    pub fn send_entities(&mut self, entities: &Entities) -> std::io::Result<()> {
        let serialized = serde_json::to_vec(entities)?;

        for player in &self.players {
            self.socket.send_to(&serialized, player.address)?;
        }

        Ok(())
    }

    // einzelner byte wird über udp vom client an den server gesendet dieser byte
    // wenn ein process loop startet sollte der server den buffer des aktiven spielers für den process verwenden und alle anderen leeren
    pub fn poll(&mut self) -> std::io::Result<()> {
        let mut buf = [0; 1];

        match self.socket.recv_from(&mut buf) {
            Ok((_, src_addr)) => {
                // Handle received data
                if let Some(player) = self.players.iter_mut().find(|p| p.address == src_addr) {
                    let input_byte = buf[0];
                    if (1..=4).contains(&input_byte) {
                        if let Some(value) = player.input_buffer.get_mut(&input_byte) {
                            *value = true;
                        }
                    } else {
                        println!("received illegal input");
                    }
                } else {
                    if self.players.len() < 4 {
                        self.add_player(src_addr)?;
                    } else {
                        println!("received message from new client, but already have 4 players so it is ignored");
                    }
                }
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available - not an actual error
                Ok(())
            }
            Err(e) => Err(e) // Other errors are propagated
        }
    }
}
