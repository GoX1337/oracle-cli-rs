mod db;
mod config;

fn main() {

    match config::Config::read_config("config.json") {
        Ok(config) => db::exec(config.database, config.username, config.password, config.sql),
        Err(e) => panic!("{}", e)
    };
}