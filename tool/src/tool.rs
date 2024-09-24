use std::sync::Mutex;
use std::time::Instant;

use libdsr::prelude::*;
use hudhook::tracing::{debug, error};
use hudhook::ImguiRenderLoop;
use imgui::{Condition, WindowFlags};
use practice_tool_core::widgets::Widget;
use tracing_subscriber::prelude::*;

use crate::config::{Config, Settings};
use crate::util;

enum UiState {
    MenuOpen,
    Closed,
    Hidden,
}

pub(crate) struct Tool {
    settings: Settings,
    pointers: PointerChains,
    ui_state: UiState,
    widgets: Vec<Box<dyn Widget>>,
    framecount: u32,
}

impl Tool {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
        log_panics::init();

        fn load_config() -> Result<Config, String> {
            let config_path = util::get_dll_path()
                .map(|mut path| {
                    path.pop();
                    path.push("dark_souls_remastered_tool.toml");
                    path
                })
                .ok_or_else(|| "Couldn't find config file".to_string())?;
            let config_content = std::fs::read_to_string(config_path)
                .map_err(|e| format!("Couldn't read config file: {:?}", e))?;
            println!("{}", config_content);
            Config::parse(&config_content).map_err(String::from)
        }

        let (config, config_err) = match load_config() {
            Ok(config) => (config, None),
            Err(e) => (Config::default(), Some(e)),
        };

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
                    .with(config.settings.log_level.inner())
                    .with(file_layer)
                    .with(stdout_layer)
                    .init();
            }
            e => {
                tracing_subscriber::fmt()
                    .with_max_level(config.settings.log_level.inner())
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

        if let Some(err) = config_err {
            debug!("{:?}", err);
        }

        let pointers = PointerChains::new();
        let settings = config.settings.clone();
        let widgets = config.make_commands(&pointers);

        Tool {
            settings,
            pointers,
            widgets,
            ui_state: UiState::MenuOpen,
            framecount: 0,
        }
    }

    fn render_visible(&mut self, ui: &imgui::Ui) {
        ui.window("##tool_window")
            .position([16., 16.], Condition::Always)
            .bg_alpha(0.8)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(|| {
                if !(ui.io().want_capture_keyboard && ui.is_any_item_active()) {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }

                for w in self.widgets.iter_mut() {
                    w.render(ui);
                }

                ui.text("An example");

                if ui.button_with_size("Close", [320.0, 0.0]) {
                    self.ui_state = UiState::Closed;
                    // self.pointers.cursor_show.set(false);
                }
            });
    }

    fn render_closed(&mut self, ui: &imgui::Ui) {
        ui.window("##msg_window")
            .position([16., ui.io().display_size[1] * 0.14], Condition::Always)
            .bg_alpha(0.0)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(|| {
                ui.text("fruizt's Dark Souls Remastered Practice Tool");

                if ui.small_button("Open") {
                    self.ui_state = UiState::MenuOpen;
                }

                ui.same_line();

                if ui.small_button("Indicators") {
                    ui.open_popup("##indicators_window");
                }

                ui.same_line();

                if ui.small_button("Help") {
                    ui.open_popup("##help_window");
                }

                ui.modal_popup_config("##help_window")
                    .resizable(false)
                    .movable(false)
                    .title_bar(false)
                    .build(|| {
                        // self.pointers.cursor_show.set(true);
                        ui.text(format!("Dark Souls Remaster Practice Tool",));
                        if ui.button_with_size("Close", [320.0, 0.0]) {
                            ui.close_current_popup();
                            // self.pointers.cursor_show.set(false);
                        }
                    });
            });
    }

    fn render_hidden(&mut self, ui: &imgui::Ui) {
        // for w in self.widgets.iter_mut() {
        //     w.interact(ui);
        // }
    }
}

impl ImguiRenderLoop for Tool {
    fn render(&mut self, ui: &mut imgui::Ui) {
        let display = self.settings.display.is_pressed(ui);
        let hide = self
            .settings
            .hide
            .map(|k| k.is_pressed(ui))
            .unwrap_or(false);

        self.framecount += 1;

        if !ui.io().want_capture_keyboard && (display || hide) {
            self.ui_state = match (&self.ui_state, hide) {
                (UiState::Hidden, _) => UiState::Closed,
                (_, true) => UiState::Hidden,
                (UiState::MenuOpen, _) => UiState::Closed,
                (UiState::Closed, _) => UiState::MenuOpen,
            };

            match &self.ui_state {
                UiState::MenuOpen => {}
                UiState::Closed => { /*self.pointers.cursor_show.set(false)*/ }
                UiState::Hidden => { /*self.pointers.cursor_show.set(false)*/ }
            }
        }

        match &self.ui_state {
            UiState::MenuOpen => {
                // self.pointers.cursor_show.set(true);
                self.render_visible(ui);
            }
            UiState::Closed => {
                self.render_closed(ui);
            }
            UiState::Hidden => {
                self.render_hidden(ui);
            }
        }

        let now = Instant::now();

        // self.render_visible(ui)
    }
}
