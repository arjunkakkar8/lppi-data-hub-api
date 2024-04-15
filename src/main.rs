use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use jemallocator::Jemalloc;
use polars::{
    lazy::dsl::{concat_str, Expr},
    prelude::*,
};
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum UorV {
    U(f32),
    V(Vec<f32>),
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

fn construct_filter(
    select: &String,
    where_clause: &HashMap<String, UorV>,
    group_by_clause: &Vec<String>,
) -> Expr {
    let mut expr: Expr = col(&select).neq(-1);
    for (key, value) in where_clause.iter() {
        match value {
            UorV::U(val) => {
                expr = expr.and(col(key).eq(*val));
            }
            UorV::V(vals) => {
                let mut local_expr: Expr = col(key).eq(vals[0]);
                for val in vals.iter().skip(1) {
                    local_expr = local_expr.or(col(key).eq(*val));
                }
                expr = expr.and(local_expr);
            }
        }
    }

    for group_by in group_by_clause.iter() {
        expr = expr.and(col(group_by).neq(-1));
    }

    expr
}

fn construct_group_by(select: &String, group_by: &Vec<String>) -> Vec<Expr> {
    let mut exprs: Vec<Expr> = Vec::new();
    exprs.push(col(&select));

    for group_by in group_by.iter() {
        exprs.push(col(group_by));
    }

    exprs
}

// Consider renaming the original data columns to avoid this step
fn reformat_group_by(group_by: &Vec<String>) -> Vec<String> {
    let mut new_group_by: Vec<String> = Vec::new();
    for group_by in group_by.iter() {
        match group_by.as_str() {
            "sex" => {
                new_group_by.push("SEX".to_string());
            }
            _ => {
                new_group_by.push(group_by.to_string());
            }
        }
    }

    new_group_by
}

#[get("/")]
async fn get_data(query: web::Json<Query>) -> impl Responder {
    let args = ScanArgsParquet::default();
    let path = Path::new("data/processed_2022.parquet");
    let lf = LazyFrame::scan_parquet(&path, args).unwrap();
    let select_string = query.select.first().unwrap();
    let group_by = reformat_group_by(&query.group_by);
    let computed = lf
        .filter(construct_filter(&select_string, &query.r#where, &group_by))
        .cast_all(DataType::Int32, true)
        .group_by(&construct_group_by(&select_string, &group_by))
        .agg(&[col("PERWT").sum().alias("count")])
        .with_columns([concat_str(
            &[lit((*select_string).as_str()), col(&select_string)],
            "_",
            false,
        )
        .alias(select_string)])
        .collect()
        .unwrap();

    println!("Data {:?} \n\n Query {:?}", computed, query);

    let pivoted = pivot::pivot_stable(
        &computed,
        group_by,
        [select_string],
        Some(["count"]),
        false,
        None,
        None,
    )
    .unwrap();

    let mut buffer = Vec::new();

    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut pivoted.clone())
        .unwrap();

    let json_string = String::from_utf8(buffer).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(get_data))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
