mod cli;
mod commands;

use cli::Cli;
use glint::Config;

fn main() {
    // if cfg!(debug_assertions) {
    //     use std::io::Write;

    //     let target = Box::new(std::fs::File::create("log.txt").expect("Can't create file"));

    //     env_logger::Builder::new()
    //         .target(env_logger::Target::Pipe(target))
    //         .filter(None, log::LevelFilter::Debug)
    //         .format(|buf, record| {
    //             writeln!(
    //                 buf,
    //                 "[{} {} {}:{}] {}",
    //                 chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
    //                 record.level(),
    //                 record.file().unwrap_or("unknown"),
    //                 record.line().unwrap_or(0),
    //                 record.args()
    //             )
    //         })
    //         .init();
    // }

    let command = cli::parse();
    let config = Config::default();

    match command {
        Cli::Commit(params) => {
            commands::commit(params, config);
        }
        Cli::Log(params) => {
            commands::log(params, config);
        }
    }
}
