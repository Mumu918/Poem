use chrono::Local;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

use crate::utils::file_path_utils::get_file_path;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: u8,
    pub content: String,
    pub md5: String,
    pub create_time: u64,
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        let file_path = get_file_path("records.db");
        let conn = Connection::open(file_path).unwrap();

        Self { conn }
    }

    pub fn init() {
        let home_dir: std::path::PathBuf = dirs::home_dir().unwrap();
        let path = home_dir.join(".poem").join("records.db");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let path = path.display().to_string();
        let conn = Connection::open(path).unwrap();
        let sql = "
        create table if not exists record
        (
            id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            content     TEXT,
            md5         VARCHAR(200) DEFAULT '',
            create_time INTEGER
        );
        ";

        conn.execute(sql, ()).unwrap();
    }

    pub fn insert_record(&self, record: Record) -> Result<i64> {
        let sql: &str = "insert into record (content, md5, create_time) values (?1, ?2, ?3)";
        let md5 = record.md5;
        let create_time = Local::now().timestamp_millis() as u64;
        self.conn
            .execute(sql, (&record.content, md5, create_time))?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn find(&self, key: &str) -> Result<Vec<Record>> {
        let key = format!("%{}%", key);
        let sql =
            "select id, content, md5, create_time from record where content like ?1 order by id desc";
        let mut stmt = self.conn.prepare(sql)?;
        let record_iter = stmt.query_map([(key)], |row| {
            Ok(Record {
                id: row.get(0)?,
                content: row.get(1)?,
                md5: row.get(2)?,
                create_time: row.get(3)?,
            })
        })?;
        let mut res = vec![];
        for record in record_iter {
            res.push(record?);
        }
        Ok(res)
    }

    pub fn find_all(&self) -> Result<Vec<Record>> {
        let sql = "select id, content, md5, create_time from record order by id desc";
        let mut stmt = self.conn.prepare(sql)?;
        let record_iter = stmt.query_map([], |row| {
            Ok(Record {
                id: row.get(0)?,
                content: row.get(1)?,
                md5: row.get(2)?,
                create_time: row.get(3)?,
            })
        })?;
        let mut res = vec![];
        for record in record_iter {
            res.push(record?);
        }
        Ok(res)
    }

    pub fn find_by_limit(&self, limit: usize) -> Result<Vec<Record>> {
        let sql = "select id, content, md5, create_time from record order by id desc limit ?1";
        let mut stmt = self.conn.prepare(sql)?;
        let record_iter = stmt.query_map([limit], |row| {
            Ok(Record {
                id: row.get(0)?,
                content: row.get(1)?,
                md5: row.get(2)?,
                create_time: row.get(3)?,
            })
        })?;
        let mut res = vec![];
        for record in record_iter {
            res.push(record?);
        }
        Ok(res)
    }

    pub fn delete_over_limit(&self, limit: usize) -> Result<usize> {
        let sql =
            "DELETE FROM record WHERE id NOT IN (SELECT id FROM record ORDER BY id DESC LIMIT ?1);";
        self.conn.execute(sql, [limit])
    }
}
