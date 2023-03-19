use std::{time::{SystemTime, Duration}, fmt};

pub use std::net::{SocketAddr, UdpSocket};
pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{Deserialize, Serialize};
pub use bevy::log::LogPlugin;

mod constants;
pub use constants::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
  Ping,
}

impl ClientMessage {
  pub fn send(&self, client: &mut RenetClient) {
    let message = bincode::serialize(self).unwrap();
    match self {
      ClientMessage::Ping
      // ClientMessage::Other
      => {
        let reliable_channel_id = ReliableChannelConfig::default().channel_id;
        if client.can_send_message(reliable_channel_id) {
          client.send_message(reliable_channel_id, message);
        } else {
          error!("Cannot send message! {:?}", self);
        }
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
  Pong,
}

impl ServerMessage {
  pub fn send(&self, client_id: u64, server: &mut RenetServer) {
    let message = bincode::serialize(self).unwrap();
    match self {
      ServerMessage::Pong
      // ServerMessage::Other
      => {
        let reliable_channel_id = ReliableChannelConfig::default().channel_id;
        if server.can_send_message(client_id, reliable_channel_id) {
          server.send_message(client_id, reliable_channel_id, message);
        } else {
          error!("Cannot send message to client! {:?} client id {}", self, client_id);
        }
      }
    }
  }
}

pub const PROTOCOL_ID: u64 = 1000;
pub const PORT: u16 = 42096;

pub fn get_current_time() -> Duration {
  SystemTime::now()
  .duration_since(SystemTime::UNIX_EPOCH)
  .unwrap()
}

// Helper struct to pass an username in the user data
pub struct Username(pub String);

impl Username {
    pub fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
            panic!("Username is too big");
        }
        user_data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
        user_data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());

        user_data
    }

    pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&user_data[0..8]);
        let mut len = u64::from_le_bytes(buffer) as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 8);
        let data = user_data[8..len + 8].to_vec();
        let username = String::from_utf8(data).unwrap();
        Self(username)
    }
}
impl fmt::Display for Username {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}