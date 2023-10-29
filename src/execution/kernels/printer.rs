use arrow::{
    array::{
        Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, StringArray,
        TimestampNanosecondArray,
    },
    record_batch::RecordBatch,
};

use super::Kernel;

pub fn print_result(kernel: &mut dyn Kernel<RecordBatch>) {
    while let Some(batch) = &kernel.next() {
        for i in 0..batch.num_rows() {
            for v in batch.columns() {
                let v = &(**v);
                match v.data_type() {
                    arrow::datatypes::DataType::Null => todo!(),
                    arrow::datatypes::DataType::Boolean => todo!(),
                    arrow::datatypes::DataType::Int8 => {
                        let v = v.as_any().downcast_ref::<Int8Array>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::Int16 => {
                        let v = v.as_any().downcast_ref::<Int16Array>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::Int32 => {
                        let v = v.as_any().downcast_ref::<Int32Array>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::Int64 => {
                        let v = v.as_any().downcast_ref::<Int64Array>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::UInt8 => todo!(),
                    arrow::datatypes::DataType::UInt16 => todo!(),
                    arrow::datatypes::DataType::UInt32 => todo!(),
                    arrow::datatypes::DataType::UInt64 => todo!(),
                    arrow::datatypes::DataType::Float16 => todo!(),
                    arrow::datatypes::DataType::Float32 => {
                        let v = v.as_any().downcast_ref::<Float32Array>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::Float64 => {
                        let v = v.as_any().downcast_ref::<Float64Array>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::Timestamp(_, _) => {
                        let v = v
                            .as_any()
                            .downcast_ref::<TimestampNanosecondArray>()
                            .unwrap()
                            .value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::Date32 => todo!(),
                    arrow::datatypes::DataType::Date64 => todo!(),
                    arrow::datatypes::DataType::Time32(_) => todo!(),
                    arrow::datatypes::DataType::Time64(_) => todo!(),
                    arrow::datatypes::DataType::Duration(_) => todo!(),
                    arrow::datatypes::DataType::Interval(_) => todo!(),
                    arrow::datatypes::DataType::Binary => todo!(),
                    arrow::datatypes::DataType::FixedSizeBinary(_) => todo!(),
                    arrow::datatypes::DataType::LargeBinary => todo!(),
                    arrow::datatypes::DataType::Utf8 => {
                        let v = v.as_any().downcast_ref::<StringArray>().unwrap().value(i);
                        print!("{v} ");
                    }
                    arrow::datatypes::DataType::LargeUtf8 => todo!(),
                    arrow::datatypes::DataType::List(_) => todo!(),
                    arrow::datatypes::DataType::FixedSizeList(_, _) => todo!(),
                    arrow::datatypes::DataType::LargeList(_) => todo!(),
                    arrow::datatypes::DataType::Struct(_) => todo!(),
                    arrow::datatypes::DataType::Union(_, _) => todo!(),
                    arrow::datatypes::DataType::Dictionary(_, _) => todo!(),
                    arrow::datatypes::DataType::Decimal128(_, _) => todo!(),
                    arrow::datatypes::DataType::Decimal256(_, _) => todo!(),
                    arrow::datatypes::DataType::Map(_, _) => todo!(),
                    arrow::datatypes::DataType::RunEndEncoded(_, _) => todo!(),
                }
            }
            println!();
        }
    }
}
