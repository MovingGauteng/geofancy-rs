#![allow(unused_imports)]

use rustracing::log;
use rustracing::sampler::AllSampler;
use rustracing::tag::{Tag, TagValue};
use rustracing_jaeger::Tracer;
use rustracing_jaeger::reporter::JaegerCompactReporter;

use futures::{future, Future, Stream};
use futures::sync::mpsc;
//use tower_h2::Server;
use tower_grpc::{Request, Response, Streaming, Error};

use geofancy::*;
use tile38_client::*;

#[derive(Clone)]
pub struct GeofancyImpl {}

impl server::GeofancyService for GeofancyImpl {

    type CreateWebHookFuture = Box<Future<Item = Response<CommandResult>, Error = Error>>;

    fn create_web_hook(&mut self, request: Request<GeoFence>) -> Self::CreateWebHookFuture {

        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult, _>;

        {
            let mut span = tracer.span("geofancy.GeofancyService/CreateWebHook")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Creating webhook");
            });

            let res = set_webhook(&request.into_inner());
            match res {
                Err(e) => {
                    span.log(|log| {
                        log.error().message("Failure setting webhook");
                    });
                    result = Err(Error::from(e));
                },
                Ok(r) => {
                    span.log(|log| {
                        log.std().message("Successfully created webhook");
                    });
                    result = Ok(r);
                }
            }
        }

        match result {
            Ok(r) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::ok(Response::new(r)))
            },
            Err(_e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::err(Error::Inner(())))
            }
        }
    }

    type DeleteWebHookFuture = Box<Future<Item = Response<CommandResult>, Error = Error>>;

    fn delete_web_hook(&mut self, request: Request<SearchString>) -> Self::DeleteWebHookFuture {

        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult, _>;

        {
            let mut span = tracer.span("geofancy.GeofancyService/DeleteWebHook")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Deleting webhook");
            });

            let res = delete_webhook(request.into_inner());
            match res {
                Err(e) => {
                    span.log(|log| {
                        log.error().message("Failure deleting webhook");
                    });
                    result = Err(Error::from(e));
                },
                Ok(r) => {
                    span.log(|log| {
                        log.std().message("Successfully deleted webhook");
                    });
                    result = Ok(r);
                }
            }
        }

        match result {
            Ok(r) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::ok(Response::new(r)))
            },
            Err(_e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::err(Error::Inner(())))
            }
        }
    }

    type SetDocumentFuture = Box<Future<Item = Response<CommandResult>, Error = Error>>;

    fn set_document(&mut self, request: Request<Document>) -> Self::SetDocumentFuture {

        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult, _>;

        {

            let mut span = tracer.span("geofancy.GeofancyService/SetDocument")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Setting document");
            });

            let doc = request.into_inner();

            let id = &doc.id;
            let collection = &doc.collection;

            match doc.geo.unwrap() {
                document::Geo::Bounds(_bounds) => {
                    span.log(|log| {
                        log.error().message("Unimplemented method");
                    });
                    unimplemented!()
                },
                document::Geo::Geojson(_geo_json) => {
                    span.log(|log| {
                        log.error().message("Unimplemented method");
                    });
                    unimplemented!()
                },
                document::Geo::Line(_line) => {
                    span.log(|log| {
                        log.error().message("Unimplemented method");
                    });
                    unimplemented!()
                },
                document::Geo::Point(point) => {
                    let res = set_point(&collection, &id, &point);
                    match res {
                        Err(e) => {
                            span.log(|log| {
                                log.error().message("Failure setting document");
                            });
                            result = Err(Error::from(e));
                        },
                        Ok(r) => {
                            span.log(|log| {
                                log.std().message("Successfully created document");
                            });
                            result = Ok(r);
                        }
                    }
                }
            }
        }

        match result {
            Ok(r) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::ok(Response::new(r)))
            },
            Err(_e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::err(Error::Inner(())))
            }
        }
    }

    type DeleteDocumentFuture = Box<Future<Item = Response<CommandResult>, Error = Error>>;

    fn delete_document(&mut self, request: Request<Document>) -> Self::DeleteDocumentFuture {
        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult, _>;

        {

            let mut span = tracer.span("geofancy.GeofancyService/DeleteDocument")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Deleting document");
            });

            let doc = request.into_inner();

            let res = delete_document(&doc.collection, &doc.id);
            match res {
                Err(e) => {
                    span.log(|log| {
                        log.error().message("Failure deleting document");
                    });
                    result = Err(Error::from(e));
                },
                Ok(r) => {
                    span.log(|log| {
                        log.std().message("Successfully deleted document");
                    });
                    result = Ok(r);
                }
            }
        }

        match result {
            Ok(r) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::ok(Response::new(r)))
            },
            Err(_e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::err(Error::Inner(())))
            }
        }
    }

    type DeleteCollectionFuture = Box<Future<Item = Response<CommandResult>, Error = Error>>;

    fn delete_collection(&mut self, request: Request<Document>) -> Self::DeleteCollectionFuture {
        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult, _>;

        {

            let mut span = tracer.span("geofancy.GeofancyService/DeleteCollection")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Deleting collection");
            });

            let doc = request.into_inner();

            let res = delete_collection(&doc.collection);
            match res {
                Err(e) => {
                    span.log(|log| {
                        log.error().message("Failure deleting collection");
                    });
                    result = Err(Error::from(e));
                },
                Ok(r) => {
                    span.log(|log| {
                        log.std().message("Successfully deleted collection");
                    });
                    result = Ok(r);
                }
            }
        }

        match result {
            Ok(r) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::ok(Response::new(r)))
            },
            Err(_e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                Box::new(future::err(Error::Inner(())))
            }
        }
    }
}

fn get_reporter() -> JaegerCompactReporter {
    JaegerCompactReporter::new("geofancy-rust").unwrap()
}