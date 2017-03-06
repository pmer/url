extern crate rusqlite;
extern crate time;

use time::Timespec;
use rusqlite::Connection;

#[derive(Debug)]
struct Article {
    id: i32,
    url: String,
    time_created: Timespec
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE article (
                  id              INTEGER PRIMARY KEY,
                  url             TEXT NOT NULL,
                  time_created    TEXT NOT NULL
                  )", &[]).unwrap();

    let logs = Article {
        id: 0,
        url: "https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying".to_string(),
        time_created: time::get_time()
    };

    conn.execute("INSERT INTO article (url, time_created)
                  VALUES (?1, ?2)",
                 &[&logs.url, &logs.time_created]).unwrap();

    let mut stmt = conn.prepare("SELECT id, url, time_created FROM article").unwrap();
    let article_iter = stmt.query_map(&[], |row| {
        Article {
            id: row.get(0),
            url: row.get(1),
            time_created: row.get(2)
        }
    }).unwrap();

    for article in article_iter {
        println!("Found article {:?}", article.unwrap());
    }
}
