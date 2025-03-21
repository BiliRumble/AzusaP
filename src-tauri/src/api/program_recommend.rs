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

// 推荐节目
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/program/recommend").route(web::get().to(program_recommend)));
}

// 入参
define_request_struct!(ProgramRecommend, {
    r#type: String,
    limit: Option<u32>,
    offset: Option<u32>,
});

impl ProgramRecommend {
    async fn requests(req: HttpRequest, query: Query<ProgramRecommend>) -> Result<Response, Value> {
        let data = json!({
            "cateId": query.r#type,
            "limit": query.limit.unwrap_or(10),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/program/recommend/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}
cache_handler!(program_recommend, ProgramRecommend);

// // 推荐节目
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cateId: query.type,
//     limit: query.limit || 10,
//     offset: query.offset || 0,
//   }
//   return request(
//     `/api/program/recommend/v1`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
