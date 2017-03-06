extern crate rusqlite;
extern crate time;

use rusqlite::Connection;
use time::Timespec;

pub struct NewArticle {
    pub url: String,
    pub time_created: Timespec
}

#[derive(Clone, Debug)]
pub struct Article {
    pub id: i32,
    pub url: String,
    pub time_created: Timespec
}

pub struct DB {
    conn: Connection
}

impl DB {
    pub fn open() -> DB {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute("CREATE TABLE article (
                  id              INTEGER PRIMARY KEY,
                  url             TEXT NOT NULL,
                  time_created    TEXT NOT NULL
                  )", &[]).unwrap();

        DB {
            conn: conn
        }
    }

    pub fn add_article(&self, article: NewArticle) {
        self.conn.execute("INSERT INTO article (url, time_created)
                           VALUES (?1, ?2)",
                          &[&article.url, &article.time_created]).unwrap();
    }

    pub fn get_articles(&self) -> Vec<Article> {
        let mut stmt = self.conn.prepare("SELECT id, url, time_created FROM article").unwrap();
        let article_iter = stmt.query_map(&[], |row| {
            Article {
                id: row.get(0),
                url: row.get(1),
                time_created: row.get(2)
            }
        }).unwrap();
        let v: Vec<Article> = article_iter.map(|art| art.unwrap()).collect();
        v.clone()
    }
}
