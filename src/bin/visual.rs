use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, Select};
use webbrowser;

use coco::app::visual::{local_server, output_static};
use coco::domain::config::CocoConfig;
use coco::infrastructure::file_scanner;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Coco Visual")
        .version(VERSION)
        .author("Inherd Group")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
        .subcommand(
            App::new("export")
                .about("export")
                .version(VERSION)
                .author("Inherd Group")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .help("output path")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("server")
                .about("server")
                .version(VERSION)
                .author("Inherd Group")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .help("http server port")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let project = select_project_prompt();

    if let Some(ref matches) = matches.subcommand_matches("export") {
        let mut path = "coco_static";
        if let Some(input) = matches.value_of("path") {
            path = input
        }

        // todo: make really output
        output_static::run(path);
        return Ok(());
    }

    // todo: add load config
    let _config = CocoConfig::default();

    let port = match matches.value_of("port") {
        Some(input) => input,
        None => "8000",
    };

    return start_local_server(project, port).await;
}

async fn start_local_server(project: String, port: &str) -> std::io::Result<()> {
    let url = format!("http://127.0.0.1:{}", port);
    println!("start server: {}", url);

    open_url(&url);

    println!("project: {}", project);
    return local_server::start(port, project).await;
}

pub fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        println!("failure to open in browser: {}", err);
    }
}

pub fn select_project_prompt() -> String {
    let selections = file_scanner::lookup_projects();
    if selections.len() == 0 {
        panic!("Please run coco first!");
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("pick project")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("1. Windows Users need to run with Windows Shell, such as PowerShell");

    let project = selections[selection].clone();
    project
}
