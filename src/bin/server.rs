
use std::net::{SocketAddr, UdpSocket};

use bevy::prelude::*;
use bevy::log::LogPlugin;
use mayhem_rs::*;

fn main() {
    App::new()
    .add_plugins(
        MinimalPlugins
    )
    .add_plugin(LogPlugin {level: bevy::log::Level::DEBUG, ..default()})
    .add_plugin(RenetServerPlugin::default())
    .insert_resource(create_renet_server())
    .add_system(server_events)
    .add_system(server_ping)
    .run()
}

fn create_renet_server() -> RenetServer {
    let current_time = get_current_time();
    let server_addr = SocketAddr::new("127.0.0.1".parse().unwrap(), PORT);
    info!("Creating server: {:?}", server_addr);
    let server_config = ServerConfig::new(8, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let connection_config = RenetConnectionConfig::default();
    RenetServer::new(
        current_time,
        server_config,
        connection_config,
        UdpSocket::bind(server_addr).unwrap(),
    ).unwrap()
}

fn server_events(
    mut events: EventReader<ServerEvent>,
) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                let username = Username::from_user_data(user_data);
                info!("Client connected: {} {}", id, username);
            },
            ServerEvent::ClientDisconnected(id) => info!("Client disconnected: {}", id),
        }
    }
}

fn server_ping(
    mut server: ResMut<RenetServer>,
) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, reliable_channel_id) {
            let client_message: ClientMessage = bincode::deserialize(&message).unwrap();
            match client_message {
                ClientMessage::Ping => {
                    info!("Got ping!");
                    ServerMessage::Pong.send(client_id, server.as_mut());
                }
            }
        }
    }
}