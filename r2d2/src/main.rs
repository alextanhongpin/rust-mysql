extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;

use mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::sync::Arc;
use std::thread;

fn main() {
    let db_url = "mysql://admin:123456@127.0.0.1:3306/test";
    let opts = Opts::from_url(db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

    let mut tasks = vec![];

    for i in 0..3 {
        let pool = pool.clone();
        let th = thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            conn.query("select user()").unwrap();
            println!("thread {} end!", i);
        });
        tasks.push(th);
    }

    for th in tasks {
        let _ = th.join();
    }
}
