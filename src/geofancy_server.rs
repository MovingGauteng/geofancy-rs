#![allow(unused_imports)]

use grpc::*;
use rustracing::log;
use rustracing::sampler::AllSampler;
use rustracing::tag::{Tag, TagValue};
use rustracing_jaeger::Tracer;
use rustracing_jaeger::reporter::JaegerCompactReporter;

use geofancy_grpc::*;
use geofancy::*;
use tile38_client::*;

pub struct GeofancyImpl;

impl GeofancyService for GeofancyImpl {

    fn create_web_hook(&self, _o: RequestOptions, request: GeoFence) -> SingleResponse<CommandResult> {

        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult>;

        {
            let mut span = tracer.span("geofancy.GeofancyService/CreateWebHook")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Creating webhook");
            });

            let res = set_webhook(&request);
            match res {
                Err(_e) => {
                    span.log(|log| {
                        log.error().message("Failure setting webhook");
                    });
                    result = Err(Error::Other("Failure setting webhook"));
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
                SingleResponse::completed(r)
            },
            Err(e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                SingleResponse::err(e)
            }
        }
    }

    fn delete_web_hook(&self, _o: RequestOptions, request: SearchString) -> SingleResponse<CommandResult> {

        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult>;

        {
            let mut span = tracer.span("geofancy.GeofancyService/DeleteWebHook")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Deleting webhook");
            });

            let res = delete_webhook(&request);
            match res {
                Err(_e) => {
                    span.log(|log| {
                        log.error().message("Failure deleting webhook");
                    });
                    result = Err(Error::Other("Failure deleting webhook"));
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
                SingleResponse::completed(r)
            },
            Err(e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                SingleResponse::err(e)
            }
        }
    }

    fn set_document(&self, _o: RequestOptions, request: Document) -> SingleResponse<CommandResult> {

        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult>;

        {

            let mut span = tracer.span("geofancy.GeofancyService/SetDocument")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Setting document");
            });

            let id = &request.get_id();
            let collection = &request.get_collection();

            if request.has_point() {
                let point = request.get_point();
                let res = set_point(&collection, &id, &point);
                match res {
                    Err(_e) => {
                        span.log(|log| {
                            log.error().message("Failure setting document");
                        });
                        result = Err(Error::Other("Failure setting document"));
                    },
                    Ok(r) => {
                        span.log(|log| {
                            log.std().message("Successfully created document");
                        });
                        result = Ok(r);
                    }
                }
            } else if request.has_line() {
                span.log(|log| {
                    log.error().message("Unimplemented method");
                });
                result = Err(Error::Other("Unimplemented method for has line"));
            } else if request.has_bounds() {
                span.log(|log| {
                    log.error().message("Unimplemented method");
                });
                result = Err(Error::Other("Unimplemented method for has bounds"));
            } else if request.has_geojson() {
                span.log(|log| {
                    log.error().message("Unimplemented method");
                });
                result = Err(Error::Other("Unimplemented method for has geojson"));
            } else {
                span.log(|log| {
                    log.error().message("Unimplemented method");
                });
                result = Err(Error::Other("Unknown geo option"));
            }
        }

        match result {
            Ok(r) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                SingleResponse::completed(r)
            },
            Err(e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                SingleResponse::err(e)
            }
        }
    }

    fn delete_document(&self, _o: RequestOptions, request: Document) -> SingleResponse<CommandResult> {
        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult>;

        {

            let mut span = tracer.span("geofancy.GeofancyService/DeleteDocument")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Deleting document");
            });

            let res = delete_document(request.get_collection(), request.get_id());
            match res {
                Err(_e) => {
                    span.log(|log| {
                        log.error().message("Failure deleting document");
                    });
                    result = Err(Error::Other("Failure deleting document"));
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
                SingleResponse::completed(r)
            },
            Err(e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                SingleResponse::err(e)
            }
        }
    }

    fn delete_collection(&self, _o: RequestOptions, request: Document) -> SingleResponse<CommandResult> {
        let (tracer, span_rx) = Tracer::new(AllSampler);

        let reporter = get_reporter();

        let result: Result<CommandResult>;

        {

            let mut span = tracer.span("geofancy.GeofancyService/DeleteCollection")
                .tag(Tag::new("request", format!("{:?}", &request)))
                .start();

            span.log(|log| {
                log.std().message("Deleting collection");
            });

            let res = delete_collection(request.get_collection());
            match res {
                Err(_e) => {
                    span.log(|log| {
                        log.error().message("Failure deleting collection");
                    });
                    result = Err(Error::Other("Failure deleting collection"));
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
                SingleResponse::completed(r)
            },
            Err(e) => {
                reporter.report(&span_rx.try_iter().collect::<Vec<_>>()).unwrap();
                SingleResponse::err(e)
            }
        }
    }
}

fn get_reporter() -> JaegerCompactReporter {
    JaegerCompactReporter::new("geofancy-rust").unwrap()
}