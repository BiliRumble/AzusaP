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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/topic/detail").route(web::get().to(topic_detail)));
}

// 入参
define_request_struct!(TopicDetail, {
    actid: String,
});

impl TopicDetail {
    async fn requests(req: HttpRequest, query: Query<TopicDetail>) -> Result<Response, Value> {
        let data = json!({
            "actid": query.actid,
        });
        create_request(
            "/api/act/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(topic_detail, TopicDetail);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     actid: query.actid,
//   }
//   return request(`/api/act/detail`, data, createOption(query, 'weapi'))
// }