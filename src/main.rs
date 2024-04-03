use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use polars::prelude::*;
use serde::Deserialize;
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
        .filter(
            col(&select)
                .neq(-1)
                .and(col("latino_race").neq(-1))
                .and(col("SEX").neq(-1))
                .and(col("veteran").neq(-1)),
        )
        .group_by([col(&select), col("SEX"), col("latino_race"), col("veteran")])
        .agg([col("PERWT").sum().alias("count")])
        .collect()
        .unwrap();
    println!("Data {:?} \n\n Query {:?}", computed, query);
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut data_file = std::fs::File::open("data/processed_2022.parquet").unwrap();
    //
    // let data = web::Data::new(AppState {
    //     data: ParquetReader::new(&mut data_file).finish().unwrap(),
    // });
    //
    // .app_data(data.clone())

    HttpServer::new(move || App::new().service(get_data))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
