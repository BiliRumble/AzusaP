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

// 热门歌单分类
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/hot").route(web::get().to(playlist_hot)));
}

// 入参
define_request_struct!(PlaylistHot, {
    id: String,
    desc: String,
});


impl PlaylistHot {
    async fn requests(req: HttpRequest, query: Query<PlaylistHot>) -> Result<Response, Value> {
        create_request(
            "/api/playlist/hottags",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(playlist_hot, PlaylistHot);


// // 热门歌单分类
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/playlist/hottags`, {}, createOption(query, 'weapi'))
// }