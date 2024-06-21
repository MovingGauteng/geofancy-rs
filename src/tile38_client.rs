use std::env;

use redis::RedisError;

use crate::geofancy::{self, *};

async fn connect() -> Result<redis::aio::MultiplexedConnection, RedisError> {
    let host = env::var("TILE38_CONNECTION").unwrap();
    redis::Client::open(host.as_str())?
        .get_multiplexed_async_connection_with_timeouts(
            std::time::Duration::from_secs(5),
            std::time::Duration::from_secs(5),
        )
        .await
}

pub async fn set_point(
    collection: &str,
    id: &str,
    point: Point,
) -> Result<CommandResult, RedisError> {
    let mut connection = connect().await?;

    let coordinates = point.coord.unwrap();

    redis::cmd("SET")
        .arg(collection)
        .arg(id)
        .arg("POINT")
        .arg(coordinates.lat)
        .arg(coordinates.lng)
        .query_async(&mut connection)
        .await?;

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };

    Ok(success)
}

pub async fn set_webhook(geofence: GeoFence) -> Result<CommandResult, RedisError> {
    let mut connection = connect().await?;

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

    q.arg("MATCH").arg(geofence.r#match.as_str()).arg("FENCE");

    // detect
    let detect: Vec<&str> = geofence
        .detect
        .clone()
        .into_iter()
        .map(|d| match d {
            0 => "INSIDE",
            1 => "OUTSIDE",
            2 => "ENTER",
            3 => "EXIT",
            4 => "CROSS",
            _ => "",
        })
        .collect();

    if !detect.is_empty() {
        let mut detect_str: Vec<&str> = Vec::new();

        for x in detect {
            detect_str.push(x);
        }
        q.arg("DETECT").arg(detect_str.as_slice().join(","));
    }

    // commands
    let commands: Vec<&str> = geofence
        .commands
        .clone()
        .into_iter()
        .map(|c| match c {
            0 => "SET",
            1 => "DEL",
            2 => "DROP",
            _ => "",
        })
        .collect();

    if !commands.is_empty() {
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

    q.query_async(&mut connection).await?;

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };

    Ok(success)
}

pub async fn delete_webhook(search_string: SearchString) -> Result<CommandResult, RedisError> {
    let mut connection = connect().await?;

    redis::cmd("PDELHOOK")
        .arg(search_string.value)
        .query_async(&mut connection)
        .await?;

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };

    Ok(success)
}

pub async fn delete_document(collection: &str, id: &str) -> Result<CommandResult, RedisError> {
    let mut connection = connect().await?;

    redis::cmd("DEL")
        .arg(collection)
        .arg(id)
        .query_async(&mut connection)
        .await?;

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };

    Ok(success)
}

pub async fn delete_collection(collection: &str) -> Result<CommandResult, RedisError> {
    let mut connection = connect().await?;

    redis::cmd("DROP")
        .arg(collection)
        .query_async(&mut connection)
        .await?;

    let success = CommandResult {
        status: 0,
        ..Default::default()
    };

    Ok(success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_point_test() {
        env::set_var("TILE38_CONNECTION", "redis://tile38-tile38:9851/0");
        let point = Point {
            coord: Some(Coordinate {
                lat: 12.355,
                lng: -26.215,
            }),
        };
        let result = set_point("test-collection", "test-id", point).await;
        // println!("{:?}", point);
        match result {
            Ok(result) => {
                assert!(result.status == 0);
            }
            Err(_e) => {}
        }
    }
}
