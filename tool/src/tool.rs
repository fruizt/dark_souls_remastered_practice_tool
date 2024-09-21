use hudhook::ImguiRenderLoop;
use imgui::Condition;

pub struct Tool(String);

impl Tool {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
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
