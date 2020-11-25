// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use std::sync::{Arc, Mutex};

use crate::contexts::FuseQueryContext;
use crate::datablocks::DataBlock;
use crate::datasources::{IDataSource, MemoryTable, Partition};
use crate::datavalues::{DataField, DataSchema, DataSchemaRef, DataType, Int64Array};
use crate::transforms::SourceTransform;

pub struct MemoryTestData {
    db: &'static str,
    table: &'static str,
    column: &'static str,
}

impl MemoryTestData {
    pub fn create() -> Self {
        MemoryTestData {
            db: "default",
            table: "t1",
            column: "c6",
        }
    }

    pub fn memory_table_schema_for_test(&self) -> DataSchemaRef {
        Arc::new(DataSchema::new(vec![DataField::new(
            self.column,
            DataType::Int64,
            false,
        )]))
    }

    pub fn memory_table_partitions_for_test(&self, datas: Vec<Vec<i64>>) -> Vec<Partition> {
        let mut partitions = Vec::with_capacity(datas.len());
        for (i, _) in datas.into_iter().enumerate() {
            let part_name = format!("part-{}", i);
            partitions.push(Partition {
                name: part_name.clone(),
                version: 0,
            });
        }
        partitions
    }

    pub fn memory_table_datasource_for_test(
        &self,
        datas: Vec<Vec<i64>>,
    ) -> Arc<Mutex<dyn IDataSource>> {
        let mut table = MemoryTable::create(self.table, self.memory_table_schema_for_test());

        for (i, data) in datas.into_iter().enumerate() {
            let block = DataBlock::create(
                Arc::new(DataSchema::new(vec![DataField::new(
                    self.column,
                    DataType::Int64,
                    false,
                )])),
                vec![Arc::new(Int64Array::from(data))],
            );
            let part_name = format!("part-{}", i);
            table.add_partition(part_name.as_str(), block).unwrap();
        }
        let datasource = crate::datasources::get_datasource("memory://").unwrap();
        datasource.lock().unwrap().add_database(self.db).unwrap();
        datasource
            .lock()
            .unwrap()
            .add_table(self.db, Arc::new(table))
            .unwrap();
        datasource
    }

    pub fn memory_table_source_transform_for_test(&self, datas: Vec<Vec<i64>>) -> SourceTransform {
        let ctx =
            FuseQueryContext::create_ctx(0, self.memory_table_datasource_for_test(datas.clone()));
        SourceTransform::create(
            ctx,
            self.db,
            self.table,
            self.memory_table_partitions_for_test(datas),
        )
    }
}
