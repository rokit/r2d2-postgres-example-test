use std::thread;
use postgres::{NoTls, Client};
use r2d2_postgres::PostgresConnectionManager;

fn main() {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres".parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    for i in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let mut client = pool.get().unwrap();
            client.execute("INSERT INTO foo (bar) VALUES ($1)", &[&i]).unwrap();
        });
    }
}