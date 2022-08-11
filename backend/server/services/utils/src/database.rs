extern crate dotenv;
extern crate dotenv_codegen;
use dotenv::dotenv;
use std::env::var;

use mysql::prelude::*;
use mysql::*;

fn create() -> Option<PooledConn> {
    dotenv().ok();
    let db_user_var = var("DB_USER");
    let db_pwd_var = var("DB_PASSWORD");
    let db_name_var = var("DB_NAME");
    let db_port_var = var("DB_PORT");
    let db_host_var = var("DB_HOST");
    let opts: Opts;
    match (
        db_user_var,
        db_pwd_var,
        db_name_var,
        db_port_var,
        db_host_var,
    ) {
        (Ok(u), Ok(p), Ok(n), Ok(port), Ok(host)) => {
            opts = Opts::from_url(&format!(
                "mysql://{name}:{pwd}@{host}:{port}/{db}",
                name = u,
                pwd = p,
                port = port,
                db = n,
                host = host
            ))
            .expect("Error initializing opts");
        }
        _ => return None,
    };
    let pool: Pool = Pool::new(opts).unwrap();
    let conn: PooledConn = pool.get_conn().unwrap();
    return Some(conn);
}

pub fn send(query: &str) -> bool {
    let mut conn: PooledConn = match create() {
        Some(x) => x,
        None => return false,
    };
    return match conn.query_iter(query) {
        Ok(_) => true,
        Err(_) => false,
    };
}

pub fn get(query: &str) -> Option<Vec<Result<Row>>> {
    let mut conn: PooledConn = match create() {
        Some(x) => x,
        None => return None,
    };
    let mut res: Vec<Result<Row>> = vec![];
    return match conn.query_iter(query) {
        Ok(x) => {
            x.for_each(|row| {
                res.push(row);
            });
            Some(res)
        }
        Err(_) => None
    };
}
