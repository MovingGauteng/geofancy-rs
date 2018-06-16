# geofancy-rs

Geofancy is a (hopefully) simple [gRPC](http://grpc.io) service that implements a subset of [Tile38](http://tile38.com).

This repository contains the Rust implementation.

There is also another repository ([geofancy-java](https://github.com/MovingGauteng/geofancy-java)), which contains the Java/Kotlin implementation of the server.

## Why Geofancy?

Tile38 is a great program, that amongst others, allows one to set geofences between moving and static geospatial objects; with the ability to trigger webhooks when conditions are met.

While we have tried out Tile38, we didn't want to implement all the logic that we wanted within some of our services, so we decided to create a separate service instead.

Geofancy is a stand-alone microservice that connects with Tile38, and exposes a subset of its API via a gRPC service.

### Implemented Methods

You can view the `geofancy.proto` , which lists the RPCs that are supported.

At the time of writing (imagine a blog post with no date having this line) ... The API is still unstable, and can evolve.
We have

```proto
service GeofancyService {
    rpc CreateWebHook (GeoFence) returns (CommandResult) {}
    rpc DeleteWebHook (SearchString) returns (CommandResult) {}
    rpc SetDocument (Document) returns (CommandResult) {}
    rpc DeleteDocument (Document) returns (CommandResult) {}
    rpc DeleteCollection (Document) returns (CommandResult) {}
}
```

Notice that we return a very simple `CommandResult` message. Contributions are welcome, if anyone would like to return something more solid as a result.