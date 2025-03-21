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

// 会员下载歌曲记录
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/downlist").route(web::get().to(song_downlist)));
}

// 入参
define_request_struct!(SongDownlist, {
    limit: Option<String>,
    offset: Option<String>,
});

impl SongDownlist {
    async fn requests(req: HttpRequest, query: Query<SongDownlist>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or_else(|| "20".to_string()),
            "offset": query.offset.clone().unwrap_or_else(|| "0".to_string()),
            "total": "true",
        });
        create_request(
            "/api/member/song/downlist",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        )
        .await
    }
}

cache_handler!(song_downlist, SongDownlist);

// // 会员下载歌曲记录
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || '20',
//     offset: query.offset || '0',
//     total: 'true',
//   }
//   return request(`/api/member/song/downlist`, data, createOption(query))
// }
