use three::*;
use three::controls::{Key};
use mint::Point3;
use std::f32::consts::{PI};
use glutin::dpi::{LogicalPosition, LogicalSize};

fn mk_ball(window: &mut Window) -> Mesh {
    let mut vertices = vec![] as Vec<Point3<f32>>;

    let max = 100;
    for i in 1..max {
        for j in 1..max {
            vertices.push(Point3::from_slice(&[
                (i as f32 / max as f32).cos() * max as f32,
                (i as f32 / max as f32).sin() * max as f32,
                (j as f32 / max as f32).cos() * max as f32,
            ]));
        }
    }

    let g = Geometry::with_vertices(vertices);
    let m = material::Basic { color: color::RED, map: None };
    window.factory.mesh(g, m)
}

fn main() {
    let mut window = Window::new("Hello, Three");

    let mut window_origin = LogicalPosition::new(0.0, 0.0);

    let gl_win = window.glutin_window();
    //gl_win.hide_cursor(true);
    if let Err(err) = gl_win.grab_cursor(true) {
        println!("Could not grab cursor: {}", err);
    }
    if let Err(err) = gl_win.set_cursor_position(window_origin) {
        println!("Could not set cursor position: {}", err);
    }
    if let Some(LogicalSize { width, height }) = gl_win.get_inner_size() {
        window_origin.x = width * 0.5;
        window_origin.y = height * 0.5;
    }

    window.scene.background = Background::Color(0x1177EE);

    let geometry = Geometry::cuboid(100.0, 100.0, 100.0);
    let material = material::Basic {
        color: color::RED,
        .. Default::default()
    };
    let red_cube = window.factory.mesh(geometry, material);
    red_cube.set_position([100.0, 100.0, 100.0]);
    window.scene.add(&red_cube);

    const ORIGIN: [f32; 3] = [0.0, 0.0, 0.0];
    let camera = window.factory.perspective_camera(60.0, 1.0 .. 1000.0);
    let mut look_at: [f32; 3] = [0.0, 0.0, 0.0];
    let mut camera_pos: [f32; 3] = [0.0, 0.0, 0.0];

    let font = window.factory.load_font_karla();
    let mut debug_text = window.factory.ui_text(&font, "Cam pos: 0.0 x 0.0");

    // Azimuth, zenith
    let mut angle: [f32; 2] = [0.0, 0.0];

    while window.update() {
        let dt = window.input.delta_time();
        let keys = window.input.keys_hit();

        let mouse_pos = window.input.mouse_pos_ndc();
        if mouse_pos.y != 0.0 {
            angle[0] = (angle[0] + mouse_pos.y * 2.0 * PI).rem_euclid(2.0 * PI);
        }
        if mouse_pos.x != 0.0 {
            angle[1] = (angle[1] + mouse_pos.x * 2.0 * PI).rem_euclid(2.0 * PI);
        }

        let forward = window.input.hit(Key::W) || window.input.hit(Key::Up);
        let backward = window.input.hit(Key::S) || window.input.hit(Key::Down);
        let leftward = window.input.hit(Key::A) || window.input.hit(Key::Left);
        let rightward = window.input.hit(Key::D) || window.input.hit(Key::Right);
        let upward = window.input.hit(Key::Space);

        let speed = dt * 100.0;

        if forward {
            camera_pos[0] += angle[0].sin() * angle[1].cos() * speed;
            camera_pos[1] += angle[0].sin() * angle[1].sin() * speed;
            camera_pos[2] += angle[0].cos() * speed;
        }
        if backward {
            camera_pos[0] -= angle[0].sin() * angle[1].cos() * speed;
            camera_pos[1] -= angle[0].sin() * angle[1].sin() * speed;
            camera_pos[2] -= angle[0].cos() * speed;
        }
        if leftward {
            camera_pos[0] += (angle[1] + PI * 0.5).cos() * speed;
            camera_pos[1] += (angle[1] + PI * 0.5).sin() * speed;
        }
        if rightward {
            camera_pos[0] += (angle[1] - PI * 0.5).cos() * speed;
            camera_pos[1] += (angle[1] - PI * 0.5).sin() * speed;
        }
        // TODO FIXME: doth not worketh
        if upward {
            camera_pos[0] += (angle[0] - PI * 0.5).sin() * speed;
            camera_pos[1] += (angle[0] - PI * 0.5).sin() * speed;
            camera_pos[2] += (angle[0] - PI * 0.5).sin() * speed;
        }

        look_at[0] = camera_pos[0] + angle[0].sin() * angle[1].cos();
        look_at[1] = camera_pos[1] + angle[0].sin() * angle[1].sin();
        look_at[2] = camera_pos[2] + angle[0].cos();

        camera.look_at(camera_pos, look_at, None);

        debug_text.set_text(
            format!("Azimuth/zenith: {} x {}", angle[0], angle[1])
        );

        if let Err(err) = window.glutin_window().set_cursor_position(window_origin) {
            println!("Could not set cursor position: {}", err);
        }

        window.render(&camera);
    }
}
