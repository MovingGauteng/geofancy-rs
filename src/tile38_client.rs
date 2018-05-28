extern crate redis;

use std::env;

use protobuf::ProtobufEnum;
use redis::*;

use geofancy::*;

fn connect() -> Result<redis::Connection, redis::RedisError> {
    let host = env::var("TILE38_CONNECTION").unwrap();
    redis::Client::open(host.as_str())?.get_connection()
}

pub fn set_point(collection: &str, id: &str, point: &Point) -> Result<CommandResult, RedisError> {
    let connection = connect().unwrap();

    let a : () = try!(redis::cmd("SET").arg(collection).arg(id).arg("POINT")
        .arg(point.get_coord().get_lat())
        .arg(point.get_coord().get_lng()).query(&connection));

    println!("{:?}", a);

    let mut success = CommandResult::new();

    success.set_status(CommandResult_CommandStatus::COMMAND_OK);

    Ok(success)
}

pub fn set_webhook(geofence: &GeoFence) -> Result<CommandResult, RedisError> {
    let connection = connect().unwrap();

    let mut q = redis::cmd("SETHOOK");
    q.arg(geofence.get_id()).arg(geofence.get_endpoint());

    if geofence.has_nearby() {
        // TODO in geofancy, I get the NEARBY from the message's name
        q.arg("NEARBY").arg(geofence.get_nearby().get_collection());
    } else {
        unimplemented!()
    }

    q.arg("MATCH").arg(geofence.get_field_match())
        .arg("FENCE");

    // detect
    let detect = geofence.get_detect();

    if detect.len() > 0 {
        let mut detect_str: Vec<&str> = Vec::new();

        for x in detect {
            detect_str.push(x.descriptor().name());
        }
        q.arg("DETECT").arg(detect_str.as_slice().join(","));
    }

    // commands
    let commands = geofence.get_commands();

    if commands.len() > 0 {
        let mut commands_str: Vec<&str> = Vec::new();

        commands_str.push("");

        for x in commands {
            commands_str.push(x.descriptor().name());
        }
        q.arg("COMMANDS").arg(commands_str.as_slice().join(","));
    }

    q.arg("POINT")
        .arg(geofence.get_point().get_coord().get_lat())
        .arg(geofence.get_point().get_coord().get_lng())
        .arg(geofence.get_distance());

    let a : () = try!(q.query(&connection));

    println!("{:?}", a);

    let mut success = CommandResult::new();

    success.set_status(CommandResult_CommandStatus::COMMAND_OK);

    Ok(success)
}

pub fn delete_webhook(search_string: &SearchString) -> Result<CommandResult, redis::RedisError> {
    let connection = connect().unwrap();
    
    let a : () = try!(redis::cmd("PDELHOOK").arg(search_string.get_value()).query(&connection));

    println!("{:?}", a);

    let mut success = CommandResult::new();

    success.set_status(CommandResult_CommandStatus::COMMAND_OK);

    Ok(success)
}

pub fn delete_document(collection: &str, id: &str) -> Result<CommandResult, redis::RedisError> {
    let connection = connect().unwrap();
    
    let a : () = try!(redis::cmd("DEL").arg(collection).arg(id).query(&connection));

    println!("{:?}", a);

    let mut success = CommandResult::new();

    success.set_status(CommandResult_CommandStatus::COMMAND_OK);

    Ok(success)
}

pub fn delete_collection(collection: &str) -> Result<CommandResult, redis::RedisError> {
    let connection = connect().unwrap();
    
    let a : () = try!(redis::cmd("DROP").arg(collection).query(&connection));

    println!("{:?}", a);

    let mut success = CommandResult::new();

    success.set_status(CommandResult_CommandStatus::COMMAND_OK);

    Ok(success)
}