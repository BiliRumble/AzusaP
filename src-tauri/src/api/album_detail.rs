use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;


// 数字专辑详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/detail").route(web::get().to(album_detail)));
}

// 入参
define_request_struct!(AlbumDetail, {
    id: String,
});


impl AlbumDetail {
    async fn requests(req: HttpRequest, query: Query<AlbumDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
        });

        create_request(
            "/api/vipmall/albumproduct/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}





// 使用宏生成缓存处理函数
cache_handler!(album_detail, AlbumDetail);