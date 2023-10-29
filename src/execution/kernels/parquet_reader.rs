use super::Kernel;

use anyhow::Result;
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};

pub struct ParquetReader {
    reader: ParquetRecordBatchReader,
}

impl ParquetReader {
    pub fn try_new(filename: &str) -> Result<Self> {
        use std::fs::File;

        let file = File::open(filename)?;
        let reader = ParquetRecordBatchReaderBuilder::try_new(file)?
            .with_batch_size(4096)
            .build()?;

        Ok(ParquetReader { reader })
    }
}

impl Kernel<RecordBatch> for ParquetReader {
    fn next(&mut self) -> Option<RecordBatch> {
        let res = self.reader.next();

        res.map(std::result::Result::unwrap)
    }
}
