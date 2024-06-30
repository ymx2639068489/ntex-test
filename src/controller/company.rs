// use std::collections::HashMap;
// use std::fmt;
// use std::fmt::Write;
use ntex::web;
use ntex::web::{HttpResponse, Responder};
use serde::{
  // Deserialize, Deserializer,
  Deserialize, Serialize
};
// use serde::de::{MapAccess, Visitor};
use crate::models::query::company::QueryCompanyQO;

pub async fn get_page_list(
    pool: crate::Pool,
    pager: web::types::Query<QueryCompanyQO>
) -> impl Responder {
    let res = crate::service::company::get_page_list(pool, pager.into_inner()).await;
    match res {
        Ok(res) => {
            crate::Response::ok_list(res, "获取成功")
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            crate::Response::server_error("error")
        }
    }
}

pub async fn update() -> String {
    "chello".to_owned()
}

pub async fn insert() -> String {
    "chello".to_owned()
}

pub async fn delete() -> String {
    "chello".to_owned()
}



// impl<'de> Deserialize<'de> for ArrayParams {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct ArrayParamsVisitor;

//         impl<'de> Visitor<'de> for ArrayParamsVisitor {
//             type Value = ArrayParams;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a map containing array parameters")
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut a_params: HashMap<usize, String> = HashMap::new();

//                 while let Some((key, value)) = map.next_entry::<String, String>()? {
//                     if key.starts_with("a[") && key.ends_with("]") {
//                         if let Ok(index) = key[2..key.len()-1].parse::<usize>() {
//                             a_params.insert(index, value);
//                         }
//                     }
//                 }

//                 let mut a = vec![String::new(); a_params.len()];
//                 for (index, value) in a_params {
//                     if index < a.len() {
//                         a[index] = value;
//                     }
//                 }

//                 Ok(ArrayParams { a })
//             }
//         }

//         deserializer.deserialize_map(ArrayParamsVisitor)
//     }
// }


#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayParams {
    a: Vec<String>,
}
pub async fn test(query: web::types::Query<ArrayParams>) -> impl Responder {
    println!("{:?}", query);
    HttpResponse::Ok().json(&query.into_inner())
}
