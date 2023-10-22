use anyhow::Result;
use parquet::arrow::arrow_reader::*;

#[allow(dead_code)]
pub struct TableDescriptor {
    name: String,
    parquet_reader: ParquetRecordBatchReader,
}

pub struct Context {
    tables: Vec<TableDescriptor>,
}

impl Context {
    pub fn new() -> Self {
        Context { tables: vec![] }
    }

    pub fn regiter_parquet_table(&mut self, name: &str, table: &str) -> Result<()> {
        use std::fs::File;

        let file = File::open(table)?;

        let parquet_reader = ParquetRecordBatchReaderBuilder::try_new(file)?
            .with_batch_size(8192)
            .build()?;

        let desc = TableDescriptor {
            name: name.to_string(),
            parquet_reader,
        };

        self.tables.push(desc);

        Ok(())
    }
}
