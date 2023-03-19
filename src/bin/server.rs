
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
                let username = Username::from_user_data(&user_data);
                info!("Client connected: {} {}", id, username);
            },
            ServerEvent::ClientDisconnected(id) => info!("Client disconnected: {}", id),
        }
    }
}