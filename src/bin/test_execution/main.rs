use labradb::execution::kernels::{parquet_reader::*, print::*};

use anyhow::Result;

fn main() -> Result<()> {
    let mut parquet_reader =
        ParquetReader::try_new("/home/gogov/git/labradb/sample-data/parquet/userdata.parquet")?;
    print_result(&mut parquet_reader);
    Ok(())
}
