use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    pub point_count: u16,
    pub freq_x: u8,
    pub freq_y: u8,
    pub mod_freq_x: u8,
    pub mod_freq_y: u8,
    pub line_weight: u8,
    pub line_color: u8,
    pub line_alpha: u8,
    pub phi: u8,
    pub connection_radius: u8,
    pub connection_ramp: u8,
    pub points: Vec<(u16,u16)>
}

impl Default for Model {
    fn default() -> Self {
        let point_count = 200;
        Model {
            point_count,
            freq_x: 4,
            freq_y: 7,
            mod_freq_x: 3,
            mod_freq_y: 2,
            line_weight: 1,
            line_color: 255,
            line_alpha: 50,
            phi: 15,
            connection_radius: 140,
            connection_ramp: 6,
            points: Vec::with_capacity(point_count as usize),
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model::default()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.point_count {
        let angle = map_range(i, 0, model.point_count, 0, PI * 2);
        let x = 

    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}