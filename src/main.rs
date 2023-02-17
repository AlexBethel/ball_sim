mod world;

use std::time::Duration;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};
use world::World;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Hello World!", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut world = World::new();
    loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return;
                }
                _ => {}
            }
        }

        world.tick();
        world.draw(&mut canvas);

        // draw_circle(
        //     &mut canvas,
        //     Color::RGB(255, 0, 0),
        //     Point::new(100, 100),
        //     20.0,
        // );

        canvas.present();
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn draw_circle<T>(canvas: &mut Canvas<T>, color: Color, center: Point, radius: f64)
where
    T: RenderTarget,
{
    let radius_i = radius.ceil() as i32;
    let mut pts = Vec::new();
    for dy in (-radius_i)..radius_i {
        for dx in (-radius_i)..radius_i {
            if dy * dy + dx * dx < (radius * radius) as i32 {
                pts.push(Point::new(center.x() + dx, center.y() + dy));
            }
        }
    }

    canvas.set_draw_color(color);
    canvas.draw_points(&pts[..]).unwrap();
}
