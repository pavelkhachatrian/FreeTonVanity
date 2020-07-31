// use std::sync::{Arc, Mutex};
// use postgres::{Connection, TlsMode};
// use postgres::types::{ToSql, INT4, VARCHAR, Type};
// use postgres_binary_copy::BinaryCopyReader;
// use streaming_iterator::StreamingIterator;
use csv::Writer;
use std::fs::File;
use crate::addr_gen::Account;

pub mod backend_types {
    pub const CSV: &str = "csv";
    pub const REDIS: &str = "redis";
    pub const POSTGRESQL: &str = "postgresql";
}

pub struct FileBackend {
    path: String,
    backend_type: String,
}

impl FileBackend {
    pub fn from_path(path: &str) -> Self {
        let backend_type;
        if path.starts_with("redis") {
            backend_type = backend_types::REDIS;
            // } else if path.starts_with("postgresql") {
            //     backend_type = backend_types::POSTGRESQL;
        } else {
            backend_type = backend_types::CSV;
        }
        Self {
            path: path.to_string(),
            backend_type: backend_type.to_string(),
        }
    }

    pub fn get_connector(&self) -> Box<dyn Connector> {
        match self.backend_type.as_ref() {
            backend_types::CSV => Box::new(CsvConnector::from_path(&self.path).unwrap()),
            backend_types::REDIS => Box::new(CsvConnector::from_path(&self.path).unwrap()),
            // backend_types::POSTGRESQL => Box::new(PostgreSQLConnector::from_connection_string(&self.path).unwrap()),
            _ => Box::new(CsvConnector::from_path(&self.path).unwrap()),
        }
    }
}


pub trait Connector: Send + Sync {
    fn save(&mut self) {}
    fn push(&mut self, item: Account, rule: u8) {}
}

pub struct RedisConnector {
    connection: String,
    data_buffer: Vec<Vec<String>>,
}

pub struct CsvConnector {
    writer: Writer<File>,
    data_buffer: Vec<Vec<String>>,
    max_data_buffer: usize
}

impl CsvConnector {
    pub fn from_path(path: &str) -> Result<Self, String> {
        let writer = Writer::from_path(path)
            .map_err(|e| format!("unable open csv file: {}", e))?;
        Ok(Self { writer, data_buffer: vec![] , max_data_buffer: 1000})
    }
}

impl Connector for CsvConnector {
    fn save(&mut self) {
        for row in &mut self.data_buffer {
            self.writer.write_record(row).unwrap();
        }
        self.writer.flush().unwrap();
        self.data_buffer.clear();
    }

    fn push(&mut self, item: Account, rule: u8) {
        if self.data_buffer.len() > self.max_data_buffer{
            self.save();
        }
        let keys = format!("{}|{}", item.public_as_string(), item.secret_as_string());
        self.data_buffer.push(vec![
            item.account_id,
            keys,
            item.seed,
            format!("{}", item.tvc),
            format!("{}", rule),
        ])
    }
}


// pub struct PostgreSQLConnector {
//     connection: Connection,
//     data_buffer: Vec<Box<dyn ToSql + Send + 'static>>,
//     max_buffer_size: usize,
// }
//
// impl PostgreSQLConnector {
//     const COPY_QUERY: &'static str = "COPY vanity_beauty_test (id, keys, seed, tvc, rule) FROM STDIN (FORMAT binary)";
//
//
//     pub fn from_connection_string(string: &str) -> Result<Self, String> {
//         let connection = Connection::connect(string, TlsMode::None)
//             .map_err(|e| format!("unable to connect to PostgreSQL: {}", e))?;
//         connection.execute("create table vanity_beauty_test
//                                 (
//                                     id   varchar,
//                                     keys varchar,
//                                     keys varchar,
//                                     tvc  integer,
//                                     rule integer
//                                 );", &[]).map_err(|e| format!("unable to create table: {}", e))?;
//         connection.execute("create index vanity_beauty_test_id_index on vanity_beauty_test (id);", &[])
//             .map_err(|e| format!("unable to create index: {}", e))?;
//         Ok(Self {connection, data_buffer: vec![], max_buffer_size: 10 })
//     }
// }
//
// impl Connector for PostgreSQLConnector {
//     fn save(&mut self) {
//         let types = &[VARCHAR, VARCHAR, VARCHAR, INT4, INT4];
//         let mut data_buffer: Vec<Box<ToSql>> = vec![];
//         std::mem::swap(&mut data_buffer, &mut self.data_buffer);
//         let data = streaming_iterator::convert(data_buffer.into_iter()).map_ref(|v| &**v);
//         let mut reader = BinaryCopyReader::new(types, data);
//         let stmt = self.connection.prepare(Self::COPY_QUERY).unwrap();
//         stmt.copy_in(&[], &mut reader).unwrap();
//     }
//
//     fn push(&mut self, item: Account, rule: u8) {
//         if self.data_buffer.len() >= self.max_buffer_size {
//             self.save();
//         }
//         let keys = format!("{}|{}", item.public_as_string(), item.secret_as_string());
//         let mut account_data: Vec<Box<ToSql>> = vec![
//             Box::new(item.account_id),
//             Box::new(keys),
//             Box::new(item.seed),
//             Box::new(item.tvc as i32),
//             Box::new(rule as i32)
//         ];
//         self.data_buffer.append(&mut account_data);
//     }
// }
