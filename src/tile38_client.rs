extern crate redis;

use std::env;

use redis::*;

use geofancy;
use geofancy::*;

fn connect() -> Result<redis::Connection, redis::RedisError> {
    let host = env::var("TILE38_CONNECTION").unwrap();
    redis::Client::open(host.as_str())?.get_connection()
}

pub fn set_point(collection: &str, id: &str, point: Point) -> Result<CommandResult, RedisError> {
    let connection = connect().unwrap();

    let coordinates = point.coord.unwrap();

    let a : () = redis::cmd("SET").arg(collection).arg(id).arg("POINT")
        .arg(coordinates.lat)
        .arg(coordinates.lng).query(&connection)?;

    println!("{:?}", a);

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };
    
    Ok(success)
}

pub fn set_webhook(geofence: GeoFence) -> Result<CommandResult, RedisError> {
    let connection = connect().unwrap();

    let mut q = redis::cmd("SETHOOK");
    q.arg(geofence.id.as_str()).arg(geofence.endpoint.as_str());

    match geofence.query.clone().unwrap() {
        geofancy::geo_fence::Query::Nearby(x) => {
            q.arg("NEARBY").arg(x.collection);
        }
        _ => {
            unimplemented!()
        }
    }


    q.arg("MATCH").arg(geofence.match_.as_str())
        .arg("FENCE");

    // detect
    let detect: Vec<&str> = geofence.detect.clone().into_iter().map(|d| {
        match d {
            0 => "INSIDE",
            1 => "OUTSIDE",
            2 => "ENTER",
            3 => "EXIT",
            4 => "CROSS",
            _ => ""
        }
    }).collect();

    if detect.len() > 0 {
        let mut detect_str: Vec<&str> = Vec::new();

        for x in detect {
            detect_str.push(x);
        }
        q.arg("DETECT").arg(detect_str.as_slice().join(","));
    }

    // commands
    let commands: Vec<&str> = geofence.commands.clone().into_iter().map(|c| {
        match c {
            0 => "SET",
            1 => "DEL",
            2 => "DROP",
            _ => ""
        }
    }).collect();

    if commands.len() > 0 {
        let mut commands_str: Vec<&str> = Vec::new();

        commands_str.push("");

        for x in commands {
            commands_str.push(x);
        }
        q.arg("COMMANDS").arg(commands_str.as_slice().join(","));
    }

    let point = geofence.point.unwrap();
    let coordinate = &point.coord.unwrap();

    q.arg("POINT")
        .arg(coordinate.lat)
        .arg(coordinate.lng)
        .arg(geofence.distance);

    let a : () = try!(q.query(&connection));

    println!("{:?}", a);

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };

    Ok(success)
}

pub fn delete_webhook(search_string: SearchString) -> Result<CommandResult, redis::RedisError> {
    let connection = connect().unwrap();
    
    let a : () = redis::cmd("PDELHOOK").arg(search_string.value).query(&connection)?;

    println!("{:?}", a);

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };
    
    Ok(success)
}

pub fn delete_document(collection: &str, id: &str) -> Result<CommandResult, redis::RedisError> {
    let connection = connect().unwrap();
    
    let a : () = try!(redis::cmd("DEL").arg(collection).arg(id).query(&connection));

    println!("{:?}", a);

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };
    
    Ok(success)
}

pub fn delete_collection(collection: &str) -> Result<CommandResult, redis::RedisError> {
    let connection = connect().unwrap();
    
    let a : () = try!(redis::cmd("DROP").arg(collection).query(&connection));

    println!("{:?}", a);

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };
    
    Ok(success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_point_test() {
        env::set_var("TILE38_CONNECTION", "redis://tile38-tile38:9851/0");
        let point = Point {
            coord: Some(Coordinate {
                lat: 12.355,
                lng: -26.215
            })
        };
        let result = set_point("test-collection", "test-id", point);
        // println!("{:?}", point);
        match result {
            Ok(result) => {
                assert!(result.status == 0);
            },
            Err(_e) => {}
        }
    }
}