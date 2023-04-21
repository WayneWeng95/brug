use std::vec;

fn running(datasize: i32) {
    let mut vec = vec::Vec::new();
    for n in 0..datasize {
        vec.push(n);
        println!("{}", n);
    }
}

use arrow::ipc::Bool;
use arrow::{array, record_batch};
use std::sync;
fn arrow_functional() {
    //A simple arrow test to testify functionality
    println!("Arrow test");
    let col_1 = sync::Arc::new(array::Int32Array::from_iter([0; 100])) as _;
    let col_2 = sync::Arc::new(array::Int32Array::from_iter([0; 100])) as _;

    let batch =
        record_batch::RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
    println!("{:?}", batch);
}

use brug;

use arrow::record_batch::*;
use datafusion::datasource::file_format::file_type::FileCompressionType;
use datafusion::error::Result;
use datafusion::prelude::*;
async fn data_fusion_example() -> Result<()> {
    let ctx = SessionContext::new();

    let testdata = datafusion::test_util::arrow_test_data();

    // register csv file with the execution context
    ctx.register_csv(
        "aggregate_test_100",
        &format!("{testdata}/csv/aggregate_test_100.csv"),
        CsvReadOptions::new(),
    )
    .await?;

    // execute the query
    let df = ctx
        .sql(
            "SELECT c1, MIN(c12), MAX(c12) \
        FROM aggregate_test_100 \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
        )
        .await?;

    // print the results
    df.show().await?;

    // query compressed CSV with specific options
    let csv_options = CsvReadOptions::default()
        .has_header(true)
        .file_compression_type(FileCompressionType::GZIP)
        .file_extension("csv.gz");
    let df = ctx
        .read_csv(
            &format!("{testdata}/csv/aggregate_test_100.csv.gz"),
            csv_options,
        )
        .await?;
    let df = df
        .filter(col("c1").eq(lit("a")))?
        .select_columns(&["c2", "c3"])?;

    df.show().await?;

    Ok(())
}

fn main() {
    println!("I'm using the library:");

    let datasize = 10000;

    let allocator = brug::Allocatormode::_JEMALLOC_;
    // let allocator = brug_allocator::Allocatormode::_MIMALLOC_;
    // let allocator = brug_allocator::Allocatormode::_MMAP_;
    // let allocator = brug_allocator::Allocatormode::_BrugTemplate_;
    // let allocator = brug_allocator::Allocatormode::_BrugAutoOpt_;

    unsafe {
        brug::BrugStruct::set_mode(allocator);
        running(datasize);
        brug::BrugStruct::end_set();
    }
}
