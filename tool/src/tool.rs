use std::fmt::Write;
use std::sync::Mutex;
use std::time::Instant;

use hudhook::tracing::{debug, error, info};
use hudhook::ImguiRenderLoop;
use imgui::{Condition, StyleVar, Ui, WindowFlags};
use libdsr::prelude::*;
use practice_tool_core::crossbeam_channel::{self, Receiver, Sender};
use practice_tool_core::widgets::Widget;
use tracing_subscriber::prelude::*;

use crate::config::{Config, IndicatorType, Settings};
use crate::util;

enum UiState {
    MenuOpen,
    Closed,
    Hidden,
}

pub(crate) struct Tool {
    settings: Settings,
    pointers: PointerChains,
    version_label: String,
    widgets: Vec<Box<dyn Widget>>,

    log: Vec<(Instant, String)>,
    log_rx: Receiver<String>,
    log_tx: Sender<String>,
    ui_state: UiState,

    framecount: u32,
    framecount_buf: String,
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
        info!("pointers {:?}", pointers);
        let version_label = {
            let (maj, min, patch) = (*VERSION).into();
            format!("Game Ver {}.{:02}.{}", maj, min, patch)
        };
        let settings = config.settings.clone();
        let widgets = config.make_commands(&pointers);

        let (log_tx, log_rx) = crossbeam_channel::unbounded();
        info!("Initialized");

        Tool {
            settings,
            pointers,
            version_label,
            widgets,
            log: Vec::new(),
            log_tx,
            log_rx,
            ui_state: UiState::Closed,
            framecount: 0,
            framecount_buf: Default::default(),
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

                if ui.button_with_size("Close", [320.0, 0.0]) {
                    self.ui_state = UiState::Closed;
                    // self.pointers.cursor_show.set(false);
                }
            });
    }

    fn render_closed(&mut self, ui: &imgui::Ui) {
        let stack_tokens = [
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];
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

                ui.modal_popup_config("##indicators_window")
                    .resizable(false)
                    .movable(false)
                    .title_bar(false)
                    .build(|| {
                        let style = ui.clone_style();

                        ui.text(
                            "You can toggle indicators here, as\nwell as reset the frame \
                             counter.\n\nKeep in mind that the available\nindicators and order of \
                             them depend\non your config file.",
                        );
                        ui.separator();

                        for indicator in &mut self.settings.indicators {
                            let label = match indicator.indicator {
                                IndicatorType::GameVersion => "Game Version",
                                IndicatorType::Position => "Player Position",
                                IndicatorType::PositionChange => "Player Velocity",
                                IndicatorType::Igt => "IGT Timer",
                                IndicatorType::Fps => "FPS",
                                IndicatorType::FrameCount => "Frame Counter",
                                IndicatorType::ImguiDebug => "ImGui Debug Info",
                                IndicatorType::Animation => "Animation",
                            };

                            let mut state = indicator.enabled;

                            if ui.checkbox(label, &mut state) {
                                indicator.enabled = state;
                            }
                            if let IndicatorType::FrameCount = indicator.indicator {
                                ui.same_line();

                                let btn_reset_label = "Reset";
                                let btn_reset_width = ui.calc_text_size(btn_reset_label)[0]
                                    + style.frame_padding[0] * 2.0;

                                ui.set_cursor_pos([
                                    ui.content_region_max()[0] - btn_reset_width,
                                    ui.cursor_pos()[1],
                                ]);

                                if ui.button("Reset") {
                                    self.framecount = 0;
                                }
                            }
                        }

                        ui.separator();

                        let btn_close_width =
                            ui.content_region_max()[0] - style.frame_padding[0] * 2.0;

                        if ui.button_with_size("Close", [btn_close_width, 0.0]) {
                            ui.close_current_popup();
                            // self.pointers.cursor_show.set(false);
                        }
                    });

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

                ui.new_line();

                for indicator in &self.settings.indicators {
                    if !indicator.enabled {
                        continue;
                    }

                    match indicator.indicator {
                        IndicatorType::GameVersion => {
                            ui.text(&self.version_label);
                        }
                        IndicatorType::FrameCount => {
                            self.framecount_buf.clear();
                            write!(self.framecount_buf, "Frame count {0}", self.framecount,).ok();
                            ui.text(&self.framecount_buf);
                        }
                        IndicatorType::ImguiDebug => {
                            imgui_debug(ui);
                        }
                        _ => {}
                    }
                }

                for w in self.widgets.iter_mut() {
                    w.render_closed(ui);
                }

                for w in self.widgets.iter_mut() {
                    w.interact(ui);
                }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn render_hidden(&mut self, ui: &imgui::Ui) {
        for w in self.widgets.iter_mut() {
            w.interact(ui);
        }
    }

    fn render_logs(&mut self, ui: &imgui::Ui) {
        let io = ui.io();

        let [dw, dh] = io.display_size;
        let [ww, wh] = [dw * 0.3, 14.0 * 6.];

        let stack_tokens = vec![
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];

        ui.window("##logs")
            .position_pivot([1., 1.])
            .position([dw * 0.95, dh * 0.8], Condition::Always)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
                    | WindowFlags::NO_INPUTS
            })
            .size([ww, wh], Condition::Always)
            .bg_alpha(0.0)
            .build(|| {
                for _ in 0..5 {
                    ui.text("");
                }
                for l in self.log.iter().rev().take(3).rev() {
                    ui.text(&l.1);
                }
                ui.set_scroll_here_y();
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
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

        for w in &mut self.widgets {
            w.log(self.log_tx.clone());
        }

        let now = Instant::now();
        self.log.extend(
            self.log_rx
                .try_iter()
                .inspect(|log| info!("{}", log))
                .map(|l| (now, l)),
        );
        self.log
            .retain(|(tm, _)| tm.elapsed() < std::time::Duration::from_secs(5));

        self.render_logs(ui);

        // self.render_visible(ui)
    }
}

// Display some imgui debug information. Very expensive.
fn imgui_debug(ui: &Ui) {
    let io = ui.io();
    ui.text(format!("Mouse position     {:?}", io.mouse_pos));
    ui.text(format!("Mouse down         {:?}", io.mouse_down));
    ui.text(format!("Want capture mouse {:?}", io.want_capture_mouse));
    ui.text(format!("Want capture kbd   {:?}", io.want_capture_keyboard));
    ui.text(format!("Want text input    {:?}", io.want_text_input));
    ui.text(format!("Want set mouse pos {:?}", io.want_set_mouse_pos));
    ui.text(format!("Any item active    {:?}", ui.is_any_item_active()));
    ui.text(format!("Any item hovered   {:?}", ui.is_any_item_hovered()));
    ui.text(format!("Any item focused   {:?}", ui.is_any_item_focused()));
    ui.text(format!("Any mouse down     {:?}", ui.is_any_mouse_down()));
}
