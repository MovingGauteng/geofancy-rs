
use tonic::{Request, Response, Status};


use crate::geofancy::*;
use crate::tile38_client::*;

#[derive(Clone)]
pub struct GeofancyImpl {}

#[tonic::async_trait]
impl geofancy_service_server::GeofancyService for GeofancyImpl {
    async fn create_web_hook(
        &self,
        request: Request<GeoFence>,
    ) -> Result<Response<CommandResult>, Status> {
        set_webhook(request.into_inner())
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn delete_web_hook(
        &self,
        request: Request<SearchString>,
    ) -> Result<Response<CommandResult>, Status> {
        delete_webhook(request.into_inner())
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn set_document(
        &self,
        request: Request<Document>,
    ) -> Result<Response<CommandResult>, Status> {
        let doc = request.into_inner();
        let id = &doc.id;
        let collection = &doc.collection;

        match doc.geo.unwrap() {
            document::Geo::Bounds(_bounds) => {
                unimplemented!()
            }
            document::Geo::Geojson(_geo_json) => {
                unimplemented!()
            }
            document::Geo::Line(_line) => {
                unimplemented!()
            }
            document::Geo::Point(point) => set_point(collection, id, point)
                .await
                .map(Response::new)
                .map_err(|e| Status::internal(e.to_string())),
        }
    }

    async fn delete_document(
        &self,
        request: Request<Document>,
    ) -> Result<Response<CommandResult>, Status> {
        let doc = request.into_inner();
        delete_document(&doc.collection, &doc.id)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn delete_collection(
        &self,
        request: Request<Document>,
    ) -> Result<Response<CommandResult>, Status> {
        let doc = request.into_inner();
        delete_collection(&doc.collection)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(e.to_string()))
    }
}
