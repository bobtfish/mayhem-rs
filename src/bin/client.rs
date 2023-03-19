#![feature(custom_inner_attributes)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::too_many_lines, clippy::type_complexity, clippy::too_many_arguments, clippy::missing_const_for_fn, clippy::unused_self, clippy::needless_pass_by_value, clippy::module_name_repetitions, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_truncation)]

pub use bevy::window::PresentMode;
use mayhem_rs::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let server_addr: SocketAddr = format!("127.0.0.1:{}", args[1]).parse().unwrap();
    let username = Username(args[2].clone());
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mayhem!".to_string(),
                        resolution: bevy::window::WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                },
            )
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {level: bevy::log::Level::DEBUG, ..default()})
        )
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(create_renet_client(server_addr, username))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_system(bevy::window::close_on_esc)
        .add_system(client_ping)
        .add_system(handle_messages)
        .run();
}

fn create_renet_client(server_addr: SocketAddr, username: Username) -> RenetClient {
    let current_time = get_current_time();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let client_id = current_time.as_millis() as u64;
    let connection_config = RenetConnectionConfig::default();
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: Some(username.to_netcode_user_data()),
        protocol_id: PROTOCOL_ID,
    };
    let client = RenetClient::new(
        current_time,
        socket,
        connection_config,
        authentication,
    );
    client.unwrap()
}

fn handle_messages(
    mut client: ResMut<RenetClient>,
) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    while let Some(message) = client.receive_message(reliable_channel_id) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::Pong => {
                info!("Got pong");
            }
        }
    }
}

fn client_ping(
    mut client: ResMut<RenetClient>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        ClientMessage::Ping.send(client.as_mut());
        info!("Sent ping!");
    }
}