extern crate hipchat_client;
extern crate hyper;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;
use hipchat_client::Client as HipchatClient;

#[derive(RustcDecodable)]
struct Config {
    token: String,
    origin: String,
    room: String,
    user: u64
}

fn setup() -> (HipchatClient, Config) {
    let contents = &mut String::new();
    File::open("./settings.json")
        .unwrap_or_else(|e| panic!("{}", e))
        .read_to_string(contents)
        .unwrap_or_else(|e| panic!("{}", e));
    let config = json::decode::<Config>(contents)
        .unwrap_or_else(|e| panic!("{}", e));
    (HipchatClient::new(config.origin.clone(), config.token.clone()), config)
}

#[test]
fn integration_get_room() {
    let (client, config) = setup();
    let room = client.get_room(&config.room).unwrap();
    println!("room: {:#?}", room);
    assert_eq!(room.name, config.room);
}

#[test]
fn integration_get_rooms() {
    let (client, config) = setup();
    let rooms = client.get_rooms(None).unwrap();
    println!("rooms: {:#?}", rooms);
    assert!(rooms.items.iter().any(|r| r.name == config.room));
}

#[test]
fn integration_get_user() {
    let (client, config) = setup();
    let user = client.get_user(&config.user.to_string()).unwrap();
    println!("user: {:#?}", user);
    assert_eq!(user.id, config.user);
}

#[test]
fn integration_get_users() {
    let (client, _) = setup();
    let users = client.get_users(None).unwrap();
    println!("users: {:#?}", users);
}

#[test]
fn integration_get_private_messages() {
    let (client, config) = setup();
    let messages = client.get_private_messages(&config.user.to_string(), None).unwrap();
    println!("messages: {:#?}", messages);
}
