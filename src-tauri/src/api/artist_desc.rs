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

// // 歌手介绍
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/artist/desc").route(web::get().to(artist_desc)));
}

// 入参
define_request_struct!(ArtistDesc, {
    id: String,
});


impl ArtistDesc {
    async fn requests(req: HttpRequest, query: Query<ArtistDesc>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
        });
        create_request(
            "/api/artist/introduction",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(artist_desc, ArtistDesc);

// // 歌手介绍
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/artist/introduction`, data, createOption(query, 'weapi'))
// }