extern crate edge;
extern crate rusqlite;
extern crate time;

mod db;

use std::io;
use edge::{Container, Request, Response};

struct MyApp;
impl MyApp {
    fn hello(&self, req: &mut Request, mut res: Response) -> io::Result<()> {
        res.content_type("text/plain");
        res.send("Hello, world!").unwrap();
        Ok(())
    }
}

fn main() {
    let d = db::DB::open();

    let article = db::NewArticle {
        url: "https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying".to_string(),
        time_created: time::get_time()
    };
    d.add_article(article);

    for article in d.get_articles() {
        println!("Found article {:?}", article);
    }

    let mut cter = Container::new(MyApp);
    cter.get("/", MyApp::hello);
    cter.start("0.0.0.0:3000").unwrap();
}
