use std::sync::Mutex;

use hudhook::tracing::error;
use hudhook::ImguiRenderLoop;
use imgui::Condition;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;

use crate::util;

pub struct Tool(String);

impl Tool {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
        log_panics::init();

        // fn load_config() -> Result<Config, String> {
        //     let config_path = util::get_dll_path()
        //         .map(|mut path| {
        //             path.pop();
        //             path.push("jdsd_dsiii_practice_tool.toml");
        //             path
        //         })
        //         .ok_or_else(|| "Couldn't find config file".to_string())?;
        //     let config_content = std::fs::read_to_string(config_path)
        //         .map_err(|e| format!("Couldn't read config file: {:?}", e))?;
        //     println!("{}", config_content);
        //     Config::parse(&config_content).map_err(String::from)
        // }

        // let (config, config_err) = match load_config() {
        //     Ok(config) => (config, None),
        //     Err(e) => (Config::default(), Some(e)),
        // };

        let log_file = util::get_dll_path()
            .map(|mut path| {
                path.pop();
                path.push("dark_souls_remastered_tool.log");
                path
            })
            .map(std::fs::File::create);

        match log_file {
            Some(Ok(log_file)) => {
                let file_layer = tracing_subscriber::fmt::layer()
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .with_writer(Mutex::new(log_file))
                    .with_ansi(false)
                    .boxed();
                let stdout_layer = tracing_subscriber::fmt::layer()
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .with_ansi(true)
                    .boxed();

                tracing_subscriber::registry()
                    .with(LevelFilter::DEBUG)
                    .with(file_layer)
                    .with(stdout_layer)
                    .init();
            }
            e => {
                tracing_subscriber::fmt()
                    .with_max_level(LevelFilter::DEBUG)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .with_ansi(true)
                    .init();

                match e {
                    None => error!("Could not construct log file path"),
                    Some(Err(e)) => error!("Could not initialize log file: {:?}", e),
                    _ => unreachable!(),
                }
            }
        }

        Tool("OMG".to_string())
    }

    fn render_visible(&mut self, ui: &imgui::Ui) {
        ui.window("Example Window")
            .position([16., 16.], Condition::Always)
            .build(|| {
                ui.text("An example");
            });
    }

    fn render_closed(&mut self, ui: &imgui::Ui) {}
}

impl ImguiRenderLoop for Tool {
    fn render(&mut self, ui: &mut imgui::Ui) {
        self.render_visible(ui)
    }
}
