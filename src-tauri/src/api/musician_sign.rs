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

// 音乐人签到
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/musician/sign").route(web::get().to(musician_sign)));
}

// 入参
define_request_struct!(MusicianSign, {});

impl MusicianSign {
    async fn requests(req: HttpRequest, query: Query<MusicianSign>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/creator/user/access",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(musician_sign, MusicianSign);


// // 音乐人签到
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(`/api/creator/user/access`, data, createOption(query, 'weapi'))
// }