use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    control_window: WindowId,
    display_window: WindowId,
    acceleration: Vector2,
    velocity: Vector2,
    position: Vector2,
}

fn model(app: &App) -> Model {
    let control_window = app
        .new_window()
        .with_title("Control Window")
        .with_dimensions(500, 500)
        .event(control_event)
        .build()
        .unwrap();
    let display_window = app
        .new_window()
        .with_dimensions(500, 500)
        .with_title("Display Window")
        .build()
        .unwrap();
    let acceleration = Vector2::zero();
    let velocity = Vector2::zero();
    let position = Vector2::zero();
    Model {
        control_window,
        display_window,
        acceleration,
        velocity,
        position,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.velocity += model.acceleration;
    model.position += model.velocity;
    model.acceleration = Vector2::zero();
}

fn control_event(_app: &App, model: &mut Model, event: WindowEvent) {
    if let WindowEvent::MouseMoved(mouse_pos) = event {
        let distance_from_center = ((mouse_pos.x).powi(2) + (mouse_pos.y).powi(2)).sqrt();
        if 50.0 < distance_from_center && distance_from_center < 250.0 {
            model.acceleration = Vector2::new((mouse_pos.x) * 0.0002, (mouse_pos.y) * 0.0002);
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw_for_window(frame.window_id()).unwrap();
    match frame.window_id() {
        id if id == model.control_window => {
            draw.background().color(BLACK);
            draw.ellipse()
            .color(INDIANRED)
            .radius(250.0);
            draw.ellipse()
            .color(CORNFLOWERBLUE)
            .radius(50.0);
        }
        id if id == model.display_window => {
            draw.background().color(BLACK);
            draw.ellipse()
            .color(CORNFLOWERBLUE)
            .x_y(model.position.x%500.0, model.position.y%500.0)
            .radius(10.0);
        }
        _ => (),
    }
    draw.to_frame(app, frame).unwrap();
}
