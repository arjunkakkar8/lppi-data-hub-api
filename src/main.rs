use actix_web::{get, web, web::ServiceConfig, HttpResponse, Responder};
use polars::prelude::*;
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum UorV {
    U(u8),
    V(Vec<u8>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Query {
    group_by: Vec<String>,
    select: Vec<String>,
    r#where: HashMap<String, UorV>,
    #[serde(default)]
    age_min: u8,
    #[serde(default)]
    age_max: u8,
}

struct AppState {
    data: DataFrame,
}

#[get("/")]
async fn get_data(query: web::Json<Query>) -> impl Responder {
    println!("Query Recieved");
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("data/processed_2022.parquet", args).unwrap();
    let select = query.select.first().unwrap();
    let computed = lf
        .clone()
        .lazy()
        .filter(col(&select).neq(-1))
        .group_by([col(&select)])
        .agg([col("PERWT").sum().alias("sum")])
        .collect()
        .unwrap();
    println!("Data {:?} \n\n Query {:?}", computed, query);
    HttpResponse::Ok().body("Hello world!")
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(get_data);
    };

    Ok(config.into())
}
