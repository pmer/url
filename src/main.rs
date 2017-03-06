extern crate rusqlite;
extern crate time;

mod db;

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
}
