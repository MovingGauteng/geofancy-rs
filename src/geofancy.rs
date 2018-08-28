#[derive(Clone, PartialEq, Message)]
pub struct CommandResult {
    #[prost(enumeration="command_result::CommandStatus", tag="1")]
    pub status: i32,
    #[prost(string, tag="2")]
    pub message: String,
}
pub mod command_result {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum CommandStatus {
        CommandOk = 0,
        CommandFailure = 1,
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct Document {
    #[prost(string, tag="1")]
    pub collection: String,
    #[prost(string, tag="2")]
    pub id: String,
    /// EX
    #[prost(uint64, tag="10")]
    pub expiry: u64,
    /// NX
    #[prost(bool, tag="11")]
    pub not_exist: bool,
    /// XX
    #[prost(bool, tag="12")]
    pub already_exist: bool,
    /// TODO add FIELD data as a map/set
    #[prost(oneof="document::Geo", tags="4, 5, 6, 7")]
    pub geo: ::std::option::Option<document::Geo>,
}
pub mod document {
    /// TODO add FIELD data as a map/set
    #[derive(Clone, Oneof, PartialEq)]
    pub enum Geo {
        #[prost(message, tag="4")]
        Point(super::Point),
        #[prost(message, tag="5")]
        Line(super::LineString),
        #[prost(message, tag="6")]
        Bounds(super::Bounds),
        #[prost(string, tag="7")]
        Geojson(String),
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct Bounds {
    #[prost(message, optional, tag="1")]
    pub south_west: ::std::option::Option<Coordinate>,
    #[prost(message, optional, tag="2")]
    pub north_east: ::std::option::Option<Coordinate>,
}
#[derive(Clone, PartialEq, Message)]
pub struct GeoFence {
    #[prost(string, tag="1")]
    pub id: String,
    #[prost(string, tag="2")]
    pub endpoint: String,
    #[prost(string, tag="6")]
    pub match_: String,
    #[prost(enumeration="geo_fence::Detect", repeated, tag="7")]
    pub detect: ::std::vec::Vec<i32>,
    #[prost(enumeration="geo_fence::Commands", repeated, tag="8")]
    pub commands: ::std::vec::Vec<i32>,
    /// fence
    /// TODO support more types per spec if we need them
    #[prost(message, optional, tag="9")]
    pub point: ::std::option::Option<Point>,
    #[prost(uint64, tag="10")]
    pub distance: u64,
    #[prost(oneof="geo_fence::Query", tags="3, 4, 5")]
    pub query: ::std::option::Option<geo_fence::Query>,
}
pub mod geo_fence {
    #[derive(Clone, PartialEq, Message)]
    pub struct QueryNearby {
        #[prost(string, tag="1")]
        pub collection: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct QueryWithin {
        #[prost(string, tag="1")]
        pub collection: String,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct QueryIntersects {
        #[prost(string, tag="1")]
        pub collection: String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum Detect {
        Inside = 0,
        Outside = 1,
        Enter = 2,
        Exit = 3,
        Cross = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum Commands {
        Set = 0,
        Del = 1,
        Drop = 2,
    }
    #[derive(Clone, Oneof, PartialEq)]
    pub enum Query {
        #[prost(message, tag="3")]
        Nearby(QueryNearby),
        #[prost(message, tag="4")]
        Within(QueryWithin),
        #[prost(message, tag="5")]
        Intersects(QueryIntersects),
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct Point {
    #[prost(message, optional, tag="1")]
    pub coord: ::std::option::Option<Coordinate>,
}
#[derive(Clone, PartialEq, Message)]
pub struct LineString {
    #[prost(message, repeated, tag="1")]
    pub coords: ::std::vec::Vec<Coordinate>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Coordinate {
    #[prost(double, tag="1")]
    pub lat: f64,
    #[prost(double, tag="2")]
    pub lng: f64,
}
#[derive(Clone, PartialEq, Message)]
pub struct SearchString {
    #[prost(string, tag="1")]
    pub value: String,
}
pub mod client {
    use ::tower_grpc::codegen::client::*;
    use super::{GeoFence, CommandResult, SearchString, Document};

    #[derive(Debug)]
    pub struct GeofancyService<T> {
        inner: grpc::Grpc<T>,
    }

    impl<T> GeofancyService<T>
    where T: tower_h2::HttpService,
    {
        pub fn new(inner: T) -> Self {
            let inner = grpc::Grpc::new(inner);
            Self { inner }
        }

        pub fn poll_ready(&mut self) -> futures::Poll<(), grpc::Error<T::Error>> {
            self.inner.poll_ready()
        }

        pub fn create_web_hook(&mut self, request: grpc::Request<GeoFence>) -> grpc::unary::ResponseFuture<CommandResult, T::Future, T::ResponseBody>
        where grpc::unary::Once<GeoFence>: grpc::Encodable<T::RequestBody>,
        {
            let path = http::PathAndQuery::from_static("/geofancy.GeofancyService/CreateWebHook");
            self.inner.unary(request, path)
        }

        pub fn delete_web_hook(&mut self, request: grpc::Request<SearchString>) -> grpc::unary::ResponseFuture<CommandResult, T::Future, T::ResponseBody>
        where grpc::unary::Once<SearchString>: grpc::Encodable<T::RequestBody>,
        {
            let path = http::PathAndQuery::from_static("/geofancy.GeofancyService/DeleteWebHook");
            self.inner.unary(request, path)
        }

        pub fn set_document(&mut self, request: grpc::Request<Document>) -> grpc::unary::ResponseFuture<CommandResult, T::Future, T::ResponseBody>
        where grpc::unary::Once<Document>: grpc::Encodable<T::RequestBody>,
        {
            let path = http::PathAndQuery::from_static("/geofancy.GeofancyService/SetDocument");
            self.inner.unary(request, path)
        }

        pub fn delete_document(&mut self, request: grpc::Request<Document>) -> grpc::unary::ResponseFuture<CommandResult, T::Future, T::ResponseBody>
        where grpc::unary::Once<Document>: grpc::Encodable<T::RequestBody>,
        {
            let path = http::PathAndQuery::from_static("/geofancy.GeofancyService/DeleteDocument");
            self.inner.unary(request, path)
        }

        pub fn delete_collection(&mut self, request: grpc::Request<Document>) -> grpc::unary::ResponseFuture<CommandResult, T::Future, T::ResponseBody>
        where grpc::unary::Once<Document>: grpc::Encodable<T::RequestBody>,
        {
            let path = http::PathAndQuery::from_static("/geofancy.GeofancyService/DeleteCollection");
            self.inner.unary(request, path)
        }
    }
}

pub mod server {
    use ::tower_grpc::codegen::server::*;
    use super::{GeoFence, CommandResult, SearchString, Document};

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => (match $e {
            Ok(futures::Async::Ready(t)) => t,
            Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
            Err(e) => return Err(From::from(e)),
        })
    }

    pub trait GeofancyService: Clone {
        type CreateWebHookFuture: futures::Future<Item = grpc::Response<CommandResult>, Error = grpc::Error>;
        type DeleteWebHookFuture: futures::Future<Item = grpc::Response<CommandResult>, Error = grpc::Error>;
        type SetDocumentFuture: futures::Future<Item = grpc::Response<CommandResult>, Error = grpc::Error>;
        type DeleteDocumentFuture: futures::Future<Item = grpc::Response<CommandResult>, Error = grpc::Error>;
        type DeleteCollectionFuture: futures::Future<Item = grpc::Response<CommandResult>, Error = grpc::Error>;

        fn create_web_hook(&mut self, request: grpc::Request<GeoFence>) -> Self::CreateWebHookFuture;

        fn delete_web_hook(&mut self, request: grpc::Request<SearchString>) -> Self::DeleteWebHookFuture;

        fn set_document(&mut self, request: grpc::Request<Document>) -> Self::SetDocumentFuture;

        fn delete_document(&mut self, request: grpc::Request<Document>) -> Self::DeleteDocumentFuture;

        fn delete_collection(&mut self, request: grpc::Request<Document>) -> Self::DeleteCollectionFuture;
    }

    #[derive(Debug, Clone)]
    pub struct GeofancyServiceServer<T> {
        geofancy_service: T,
    }

    impl<T> GeofancyServiceServer<T>
    where T: GeofancyService,
    {
        pub fn new(geofancy_service: T) -> Self {
            Self { geofancy_service }
        }
    }

    impl<T> tower::Service for GeofancyServiceServer<T>
    where T: GeofancyService,
    {
        type Request = http::Request<tower_h2::RecvBody>;
        type Response = http::Response<geofancy_service::ResponseBody<T>>;
        type Error = h2::Error;
        type Future = geofancy_service::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: Self::Request) -> Self::Future {
            use self::geofancy_service::Kind::*;

            match request.uri().path() {
                "/geofancy.GeofancyService/CreateWebHook" => {
                    let service = geofancy_service::methods::CreateWebHook(self.geofancy_service.clone());
                    let response = grpc::Grpc::unary(service, request);
                    geofancy_service::ResponseFuture { kind: Ok(CreateWebHook(response)) }
                }
                "/geofancy.GeofancyService/DeleteWebHook" => {
                    let service = geofancy_service::methods::DeleteWebHook(self.geofancy_service.clone());
                    let response = grpc::Grpc::unary(service, request);
                    geofancy_service::ResponseFuture { kind: Ok(DeleteWebHook(response)) }
                }
                "/geofancy.GeofancyService/SetDocument" => {
                    let service = geofancy_service::methods::SetDocument(self.geofancy_service.clone());
                    let response = grpc::Grpc::unary(service, request);
                    geofancy_service::ResponseFuture { kind: Ok(SetDocument(response)) }
                }
                "/geofancy.GeofancyService/DeleteDocument" => {
                    let service = geofancy_service::methods::DeleteDocument(self.geofancy_service.clone());
                    let response = grpc::Grpc::unary(service, request);
                    geofancy_service::ResponseFuture { kind: Ok(DeleteDocument(response)) }
                }
                "/geofancy.GeofancyService/DeleteCollection" => {
                    let service = geofancy_service::methods::DeleteCollection(self.geofancy_service.clone());
                    let response = grpc::Grpc::unary(service, request);
                    geofancy_service::ResponseFuture { kind: Ok(DeleteCollection(response)) }
                }
                _ => {
                    geofancy_service::ResponseFuture { kind: Err(grpc::Status::UNIMPLEMENTED) }
                }
            }
        }
    }

    impl<T> tower::NewService for GeofancyServiceServer<T>
    where T: GeofancyService,
    {
        type Request = http::Request<tower_h2::RecvBody>;
        type Response = http::Response<geofancy_service::ResponseBody<T>>;
        type Error = h2::Error;
        type Service = Self;
        type InitError = h2::Error;
        type Future = futures::FutureResult<Self::Service, Self::Error>;

        fn new_service(&self) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    pub mod geofancy_service {
        use ::tower_grpc::codegen::server::*;
        use super::GeofancyService;

        pub struct ResponseFuture<T>
        where T: GeofancyService,
        {
            pub(super) kind: Result<Kind<
                grpc::unary::ResponseFuture<methods::CreateWebHook<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::DeleteWebHook<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::SetDocument<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::DeleteDocument<T>, tower_h2::RecvBody>,
                grpc::unary::ResponseFuture<methods::DeleteCollection<T>, tower_h2::RecvBody>,
            >, grpc::Status>,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where T: GeofancyService,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = h2::Error;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateWebHook(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(CreateWebHook(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(DeleteWebHook(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(DeleteWebHook(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(SetDocument(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(SetDocument(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(DeleteDocument(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(DeleteDocument(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Ok(DeleteCollection(ref mut fut)) => {
                        let response = try_ready!(fut.poll());
                        let (head, body) = response.into_parts();
                        let body = ResponseBody { kind: Ok(DeleteCollection(body)) };
                        let response = http::Response::from_parts(head, body);
                        Ok(response.into())
                    }
                    Err(ref status) => {
                        let body = ResponseBody { kind: Err(status.clone()) };
                        Ok(grpc::Response::new(body).into_http().into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where T: GeofancyService,
        {
            pub(super) kind: Result<Kind<
                grpc::Encode<grpc::unary::Once<<methods::CreateWebHook<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::DeleteWebHook<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::SetDocument<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::DeleteDocument<T> as grpc::UnaryService>::Response>>,
                grpc::Encode<grpc::unary::Once<<methods::DeleteCollection<T> as grpc::UnaryService>::Response>>,
            >, grpc::Status>,
        }

        impl<T> tower_h2::Body for ResponseBody<T>
        where T: GeofancyService,
        {
            type Data = bytes::Bytes;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateWebHook(ref v)) => v.is_end_stream(),
                    Ok(DeleteWebHook(ref v)) => v.is_end_stream(),
                    Ok(SetDocument(ref v)) => v.is_end_stream(),
                    Ok(DeleteDocument(ref v)) => v.is_end_stream(),
                    Ok(DeleteCollection(ref v)) => v.is_end_stream(),
                    Err(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, h2::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateWebHook(ref mut v)) => v.poll_data(),
                    Ok(DeleteWebHook(ref mut v)) => v.poll_data(),
                    Ok(SetDocument(ref mut v)) => v.poll_data(),
                    Ok(DeleteDocument(ref mut v)) => v.poll_data(),
                    Ok(DeleteCollection(ref mut v)) => v.poll_data(),
                    Err(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, h2::Error> {
                use self::Kind::*;

                match self.kind {
                    Ok(CreateWebHook(ref mut v)) => v.poll_trailers(),
                    Ok(DeleteWebHook(ref mut v)) => v.poll_trailers(),
                    Ok(SetDocument(ref mut v)) => v.poll_trailers(),
                    Ok(DeleteDocument(ref mut v)) => v.poll_trailers(),
                    Ok(DeleteCollection(ref mut v)) => v.poll_trailers(),
                    Err(ref status) => {
                        let mut map = http::HeaderMap::new();
                        map.insert("grpc-status", status.to_header_value());
                        Ok(Some(map).into())
                    }
                }
            }
        }

        #[derive(Debug, Clone)]
        pub(super) enum Kind<CreateWebHook, DeleteWebHook, SetDocument, DeleteDocument, DeleteCollection> {
            CreateWebHook(CreateWebHook),
            DeleteWebHook(DeleteWebHook),
            SetDocument(SetDocument),
            DeleteDocument(DeleteDocument),
            DeleteCollection(DeleteCollection),
        }

        pub mod methods {
            use ::tower_grpc::codegen::server::*;
            use super::super::{GeofancyService, GeoFence, CommandResult, SearchString, Document};

            pub struct CreateWebHook<T>(pub T);

            impl<T> tower::Service for CreateWebHook<T>
            where T: GeofancyService,
            {
                type Request = grpc::Request<GeoFence>;
                type Response = grpc::Response<CommandResult>;
                type Error = grpc::Error;
                type Future = T::CreateWebHookFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.create_web_hook(request)
                }
            }

            pub struct DeleteWebHook<T>(pub T);

            impl<T> tower::Service for DeleteWebHook<T>
            where T: GeofancyService,
            {
                type Request = grpc::Request<SearchString>;
                type Response = grpc::Response<CommandResult>;
                type Error = grpc::Error;
                type Future = T::DeleteWebHookFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.delete_web_hook(request)
                }
            }

            pub struct SetDocument<T>(pub T);

            impl<T> tower::Service for SetDocument<T>
            where T: GeofancyService,
            {
                type Request = grpc::Request<Document>;
                type Response = grpc::Response<CommandResult>;
                type Error = grpc::Error;
                type Future = T::SetDocumentFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.set_document(request)
                }
            }

            pub struct DeleteDocument<T>(pub T);

            impl<T> tower::Service for DeleteDocument<T>
            where T: GeofancyService,
            {
                type Request = grpc::Request<Document>;
                type Response = grpc::Response<CommandResult>;
                type Error = grpc::Error;
                type Future = T::DeleteDocumentFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.delete_document(request)
                }
            }

            pub struct DeleteCollection<T>(pub T);

            impl<T> tower::Service for DeleteCollection<T>
            where T: GeofancyService,
            {
                type Request = grpc::Request<Document>;
                type Response = grpc::Response<CommandResult>;
                type Error = grpc::Error;
                type Future = T::DeleteCollectionFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: Self::Request) -> Self::Future {
                    self.0.delete_collection(request)
                }
            }
        }
    }
}
