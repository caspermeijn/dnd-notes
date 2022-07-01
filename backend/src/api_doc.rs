use crate::AdditionalNames;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(utoipa::OpenApi)]
#[openapi(
    handlers(crate::hello, crate::hello_name,),
    components(AdditionalNames,)
)]
struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(SwaggerUi::new("/api-doc/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}
