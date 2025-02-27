use std::future::{Ready, ready};
use ntex::web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Pager {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
}
impl Pager {
  pub fn new(pager: &impl super::query::Pager, total: i64) -> Pager {
    Pager {
      page: pager.get_page(),
      page_size: pager.get_page_size(),
      total,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct ResponseWrapper<'a, T> {
    pub code: i32,
    pub message: &'a str,
    pub data: Option<T>,
    pub pager: Option<Pager>,
    pub list: Option<Vec<T>>,
}
const SUCCESS_CODE: i32 = 200;
const SERVER_ERROR_CODE: i32 = 503;
const CLIENT_ERROR_CODE: i32 = 400;

pub struct Response;

impl Response {
    pub fn ok<T>(data: Option<T>) -> ResponseWrapper<'static, T> {
        ResponseWrapper {
            code: SUCCESS_CODE,
            message: "获取成功",
            data,
            pager: None,
            list: None,
        }
    }

    pub fn ok_list<T>(data: Vec<T>, message: &str) -> ResponseWrapper<T> {
        ResponseWrapper {
            code: SUCCESS_CODE,
            message,
            list: Some(data),
            data: None,
            pager: None,
        }
    }

    pub fn ok_pager<T>(data: Vec<T>, pager: Pager) -> ResponseWrapper<'static, T> {
        ResponseWrapper {
            code: SUCCESS_CODE,
            message: "",
            list: Some(data),
            pager: Some(pager),
            data: None,
        }
    }

    pub fn server_error<T>(message: &str) -> ResponseWrapper<T> {
        ResponseWrapper {
            code: SERVER_ERROR_CODE,
            message,
            list: None,
            data: None,
            pager: None,
        }
    }

    pub fn client_error<T>(message: &str) -> ResponseWrapper<T> {
        ResponseWrapper {
            code: CLIENT_ERROR_CODE,
            message,
            list: None,
            data: None,
            pager: None,
        }
    }
}

impl <'a, T> Responder for ResponseWrapper<'a, T>
where
    T: Serialize,
{
    fn respond_to(self, _req: &HttpRequest) -> Ready<HttpResponse> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::Ok()
            .content_type("application/json")
            .body(body);
        ready(res)
    }
}