use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use css_inline::inline;
use log::error;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use serde::{Deserialize, Serialize};
use actix_web::post;

const DEFAULT_TRUE: bool = true;
const DEFAULT_FALSE: bool = false;


fn get_default_true() -> bool {
    DEFAULT_TRUE
}

fn get_default_false() -> bool {
    DEFAULT_FALSE
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{


    #[derive(OpenApi)]
    #[openapi(
    paths(get_email),
    components(
    schemas(EmailRequest)),
    tags(
    (name = "email", description = "Convert an email with classes to an email with inlined css")
    ),
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(get_email)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    }).workers(4)
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailRequest{
    html: String,
    #[serde(default = "get_default_false")]
    load_remote_stylesheets: bool,
    #[serde(default = "get_default_true")]
    remove_style_tags: bool,
}

#[utoipa::path(
responses(
(status = 200, description = "Inline the css of an email", body = String)
),
request_body = EmailRequest,
tag="email"
)]
#[post("/email")]
pub async fn get_email(data: web::Json<EmailRequest>) ->impl Responder{
    let inliner = css_inline::CSSInliner::options()
        .load_remote_stylesheets(data.load_remote_stylesheets)
        .remove_style_tags(data.remove_style_tags)
        .build();
    let inlined_html = inliner.inline(&data.html);

    return match inlined_html {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => {
            error!("Error inlining css: {:?}", e);
            HttpResponse::InternalServerError().body("Ein Fehler ist aufgetreten")
        }
    }
}

