// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait GeofancyService {
    fn create_web_hook(&self, o: ::grpc::RequestOptions, p: super::geofancy::GeoFence) -> ::grpc::SingleResponse<super::geofancy::CommandResult>;

    fn delete_web_hook(&self, o: ::grpc::RequestOptions, p: super::geofancy::SearchString) -> ::grpc::SingleResponse<super::geofancy::CommandResult>;

    fn set_document(&self, o: ::grpc::RequestOptions, p: super::geofancy::Document) -> ::grpc::SingleResponse<super::geofancy::CommandResult>;

    fn delete_document(&self, o: ::grpc::RequestOptions, p: super::geofancy::Document) -> ::grpc::SingleResponse<super::geofancy::CommandResult>;

    fn delete_collection(&self, o: ::grpc::RequestOptions, p: super::geofancy::Document) -> ::grpc::SingleResponse<super::geofancy::CommandResult>;
}

// client

pub struct GeofancyServiceClient {
    grpc_client: ::grpc::Client,
    method_CreateWebHook: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::geofancy::GeoFence, super::geofancy::CommandResult>>,
    method_DeleteWebHook: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::geofancy::SearchString, super::geofancy::CommandResult>>,
    method_SetDocument: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::geofancy::Document, super::geofancy::CommandResult>>,
    method_DeleteDocument: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::geofancy::Document, super::geofancy::CommandResult>>,
    method_DeleteCollection: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::geofancy::Document, super::geofancy::CommandResult>>,
}

impl GeofancyServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        GeofancyServiceClient {
            grpc_client: grpc_client,
            method_CreateWebHook: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/geofancy.GeofancyService/CreateWebHook".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteWebHook: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/geofancy.GeofancyService/DeleteWebHook".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetDocument: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/geofancy.GeofancyService/SetDocument".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteDocument: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/geofancy.GeofancyService/DeleteDocument".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteCollection: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/geofancy.GeofancyService/DeleteCollection".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            GeofancyServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            GeofancyServiceClient::with_client(c)
        })
    }
}

impl GeofancyService for GeofancyServiceClient {
    fn create_web_hook(&self, o: ::grpc::RequestOptions, p: super::geofancy::GeoFence) -> ::grpc::SingleResponse<super::geofancy::CommandResult> {
        self.grpc_client.call_unary(o, p, self.method_CreateWebHook.clone())
    }

    fn delete_web_hook(&self, o: ::grpc::RequestOptions, p: super::geofancy::SearchString) -> ::grpc::SingleResponse<super::geofancy::CommandResult> {
        self.grpc_client.call_unary(o, p, self.method_DeleteWebHook.clone())
    }

    fn set_document(&self, o: ::grpc::RequestOptions, p: super::geofancy::Document) -> ::grpc::SingleResponse<super::geofancy::CommandResult> {
        self.grpc_client.call_unary(o, p, self.method_SetDocument.clone())
    }

    fn delete_document(&self, o: ::grpc::RequestOptions, p: super::geofancy::Document) -> ::grpc::SingleResponse<super::geofancy::CommandResult> {
        self.grpc_client.call_unary(o, p, self.method_DeleteDocument.clone())
    }

    fn delete_collection(&self, o: ::grpc::RequestOptions, p: super::geofancy::Document) -> ::grpc::SingleResponse<super::geofancy::CommandResult> {
        self.grpc_client.call_unary(o, p, self.method_DeleteCollection.clone())
    }
}

// server

pub struct GeofancyServiceServer;


impl GeofancyServiceServer {
    pub fn new_service_def<H : GeofancyService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/geofancy.GeofancyService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/geofancy.GeofancyService/CreateWebHook".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_web_hook(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/geofancy.GeofancyService/DeleteWebHook".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_web_hook(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/geofancy.GeofancyService/SetDocument".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.set_document(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/geofancy.GeofancyService/DeleteDocument".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_document(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/geofancy.GeofancyService/DeleteCollection".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_collection(o, p))
                    },
                ),
            ],
        )
    }
}
