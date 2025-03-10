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

// 获取音乐人任务
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/musician/tasks/new").route(web::get().to(musician_tasks_new)));
}

// 入参
define_request_struct!(MusicianTasksNew, {});

impl MusicianTasksNew {
    async fn requests(req: HttpRequest, query: Query<MusicianTasksNew>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/nmusician/workbench/mission/stage/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(musician_tasks_new, MusicianTasksNew);


// // 获取音乐人任务
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/nmusician/workbench/mission/stage/list `,
//     data,
//     createOption(query, 'weapi'),
//   )
// }