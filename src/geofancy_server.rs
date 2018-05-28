#![allow(unused_imports)]

use grpc::*;

use geofancy_grpc::*;
use geofancy::*;
use tile38_client::*;

pub struct GeofancyImpl;

impl GeofancyService for GeofancyImpl {
    fn create_web_hook(&self, _o: RequestOptions, request: GeoFence) -> SingleResponse<CommandResult> {
        println!("Creating webhook");
        println!("{:?}", request);
        let res = set_webhook(&request);
        match res {
            Err(e) => {
                println!("Redis error {:?}", e);
                SingleResponse::err(Error::Other(""))
            },
            Ok(r) => {
                SingleResponse::completed(r)
            }
        }
    }

    fn delete_web_hook(&self, _o: RequestOptions, request: SearchString) -> SingleResponse<CommandResult> {
        let res = delete_webhook(&request);
        match res {
            Err(_e) => {
                SingleResponse::err(Error::Other(""))
            },
            Ok(r) => {
                SingleResponse::completed(r)
            }
        }
    }

    fn set_document(&self, _o: RequestOptions, request: Document) -> SingleResponse<CommandResult> {
        let id = &request.get_id();
        let collection = &request.get_collection();

        if request.has_point() {
            let point = request.get_point();
            let res = set_point(&collection, &id, &point);
            match res {
                Err(_e) => {
                    SingleResponse::err(Error::Other(""))
                },
                Ok(r) => {
                    SingleResponse::completed(r)
                }
            }
        }
        else if request.has_line() {
            unimplemented!()
        }
        else if request.has_bounds() {
            unimplemented!()
        }
        else if request.has_geojson() {
            unimplemented!()
        } else {
            unimplemented!()
        }
    }

    fn delete_document(&self, _o: RequestOptions, request: Document) -> SingleResponse<CommandResult> {
        let res = delete_document(request.get_collection(), request.get_id());
        match res {
            Err(_e) => {
                SingleResponse::err(Error::Other(""))
            },
            Ok(r) => {
                SingleResponse::completed(r)
            }
        }
    }

    fn delete_collection(&self, _o: RequestOptions, request: Document) -> SingleResponse<CommandResult> {
        let res = delete_collection(request.get_collection());
        match res {
            Err(_e) => {
                SingleResponse::err(Error::Other(""))
            },
            Ok(r) => {
                SingleResponse::completed(r)
            }
        }
    }
}