use std::ops::Rem;
use polars::prelude::*;
use polars::df;
use polars::lazy::dsl::col;
use std::io::{self, BufRead};
use polars::datatypes::DataType::UInt8;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if input == "q" {
            return;
        }
        let mylist: Vec<String> = input.split(',').into_vec();
        
        let df = df![ "ints"  => mylist ].unwrap();
        let frame = df.lazy()
            .with_column(col("ints").cast(UInt8))
            .group_by([col("ints")])
            .agg([col("ints").count().alias("count")])
            .with_column(col("count").rem(lit(2)))
            .filter(col("count").eq(lit(1)))
            .collect().unwrap();
        let answer = frame.column("ints").unwrap().get(0).unwrap();
        
        println!("{answer}");
    }
}
