use cgmath::{Euler, InnerSpace, Matrix4, Point3, Quaternion, Rad, Transform};
use input::Input;
use rayon::prelude::*;

use arrayvec::ArrayVec;
use box_shape::BoxShape;
use glutin::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use program::Program;
use ray::Ray;
use shape::Shape;
use triangle_shape::TriangleShape;
use vector::{SteppedVector, Vector};

mod gl;
mod input;
mod program;
mod ray;
mod vector;

// TODO: (Performance, Memory) Improve shape classes
// TODO: Shape base trait
mod box_shape;
mod plane_shape;
mod shape;
mod triangle_shape;

const UP: Vector = Vector::new(0.0, 1.0, 0.0);

const _MISSING_TEXTURE: [u8; 16] = [
    255, 0, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 255, 255,
];

const SCREEN_VERTICES: [f32; 18] = [
    -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0,
];

// const VIEWPORT_SIZE: LogicalSize<usize> = LogicalSize::new(320, 180);
const VIEWPORT_SIZE: LogicalSize<usize> = LogicalSize::new(640, 360);

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Rraayy")
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1280, 720));

    let window = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let window = unsafe { window.make_current().unwrap() };

    let mut dragging: bool = false;
    let mut input: Input = Input::new();

    let mut last_time: std::time::Instant = std::time::Instant::now();

    let mut camera_pitch: f32 = 0.0;
    let mut camera_yaw: f32 = 0.0;
    let mut camera_position: Point3<f32> = Point3::new(0.0, 5.0, 0.0);
    let mut camera_direction: Vector = Vector::new(0.0, 0.0, -1.0);
    let camera_near: f32 = 0.1;
    let camera_far: f32 = 500.0;

    let mut prev_mouse_position = PhysicalPosition::new(0.0, 0.0);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        gl::Enable(gl::TEXTURE_2D);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Viewport(0, 0, 1280, 720);
    }

    let program = Program::new_vert_frag("assets/shaders/default.glsl");

    let mut texture_id = 0;

    // let image = image::open("assets/tile.png").unwrap();
    // let rgba = image.as_rgba8().unwrap();

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexStorage2D(
            gl::TEXTURE_2D,
            1,
            gl::RGBA8,
            VIEWPORT_SIZE.width as i32,
            VIEWPORT_SIZE.height as i32,
        );
        // gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, VIEWPORT_SIZE.width, VIEWPORT_SIZE.height, gl::RGBA, gl::UNSIGNED_BYTE, buffer.as_ptr() as *const _);
    }

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (std::mem::size_of::<f32>() * SCREEN_VERTICES.len()) as isize,
            SCREEN_VERTICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (std::mem::size_of::<f32>() * 3) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    let mut _next_update =
        std::time::Instant::now() + std::time::Duration::from_secs_f32(1.0 / 30.0);

    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::WaitUntil(next_update);
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => unsafe {
                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let mouse_motion_x = (position.x - prev_mouse_position.x) * 0.1;
                    let mouse_motion_y = (position.y - prev_mouse_position.y) * 0.1;
                    prev_mouse_position = position;

                    if dragging {
                        camera_pitch = (camera_pitch + mouse_motion_y as f32).clamp(-89.9, 89.9);
                        camera_yaw = (camera_yaw - mouse_motion_x as f32) % 360.0;
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        dragging = state == ElementState::Pressed;
                    }
                }
                WindowEvent::KeyboardInput {
                    input: key_input, ..
                } => {
                    input.process_event(key_input);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let delta = (now - last_time).as_secs_f32();
                last_time = now;

                let rad_pitch = camera_pitch.to_radians();
                let rad_yaw = camera_yaw.to_radians();
                let xz_len = rad_pitch.cos();
                camera_direction.x = xz_len * rad_yaw.sin();
                camera_direction.y = (-rad_pitch).sin();
                camera_direction.z = xz_len * (-rad_yaw).cos();
                camera_direction = camera_direction.normalize();

                let camera_right =
                    -UP.cross(Vector::new(camera_direction.x, 0.0, camera_direction.z).normalize());

                let camera_speed = 7.5;

                if input.key_pressed(VirtualKeyCode::W) {
                    camera_position += camera_direction * camera_speed * delta;
                }
                if input.key_pressed(VirtualKeyCode::A) {
                    camera_position -= camera_right * camera_speed * delta;
                }
                if input.key_pressed(VirtualKeyCode::S) {
                    camera_position -= camera_direction * camera_speed * delta;
                }
                if input.key_pressed(VirtualKeyCode::D) {
                    camera_position += camera_right * camera_speed * delta;
                }
                if input.key_pressed(VirtualKeyCode::Space) {
                    camera_position.y += camera_speed * delta;
                }
                if input.key_pressed(VirtualKeyCode::LShift) {
                    camera_position.y -= camera_speed * delta;
                }

                let cells_hor: u32 = 4;
                let cells_vert: u32 = 4;
                let cells: u32 = cells_hor * cells_vert;
                let pixels_hor: u32 = (VIEWPORT_SIZE.width as u32) / cells_hor;
                let pixels_vert: u32 = (VIEWPORT_SIZE.height as u32) / cells_vert;
                let pixels: u32 = pixels_hor * pixels_vert;

                let box_shape = BoxShape::new(
                    SteppedVector::new(-5, 0, -30),
                    SteppedVector::new(5, 10, -20),
                );
                let floor_shape = BoxShape::new(
                    SteppedVector::new(-50, 10, -50),
                    SteppedVector::new(350, 11, 350),
                );
                let triangle_shape = TriangleShape::new(
                    Vector::new(15.0, 10.0, -20.0),
                    Vector::new(5.0, 0.0, -20.0),
                    Vector::new(5.0, 10.0, -20.0),
                );

                let mut shapes: Vec<Box<dyn Shape>> = vec![
                    Box::new(box_shape),
                    Box::new(floor_shape),
                    Box::new(triangle_shape),
                ];

                for x in 0..10 {
                    let x_offset = x * 15 + 25;
                    for z in 0..10 {
                        let z_offset = z * 15 + 25;
                        shapes.push(Box::new(BoxShape::new(
                            SteppedVector::new(-5 + x_offset, 0, -5 + z_offset),
                            SteppedVector::new(5 + x_offset, 10, 5 + z_offset),
                        )));
                    }
                }

                let camera_rotation_pitch = Quaternion::from(Euler {
                    x: Rad(rad_pitch),
                    y: Rad(0.0),
                    z: Rad(0.0),
                });

                let camera_rotation_yaw = Quaternion::from(Euler {
                    x: Rad(0.0),
                    y: Rad(rad_yaw),
                    z: Rad(0.0),
                });

                let mut camera_matrix =
                    Matrix4::from_translation(camera_position.to_homogeneous().truncate())
                        .inverse_transform()
                        .unwrap();
                camera_matrix = camera_matrix
                    * Matrix4::from(camera_rotation_yaw)
                    * Matrix4::from(camera_rotation_pitch);

                let cells: Vec<Vec<u8>> = (0..cells)
                    .into_par_iter()
                    .map(|cell_index| {
                        let cell_x = cell_index % cells_hor;
                        let cell_y = cell_index / cells_hor;

                        (0..pixels)
                            .flat_map(|pixel_index| {
                                let pixel_x = (pixel_index % pixels_hor) + (cell_x * pixels_hor);
                                let pixel_y = (pixel_index / pixels_hor) + (cell_y * pixels_vert);

                                let mut depth: f32 = camera_far;

                                let ray = pixel_to_camera(pixel_x, pixel_y, camera_matrix);

                                for shape in shapes.iter() {
                                    let (result, t) = shape.intersects_ray(&ray);

                                    if result {
                                        if t < depth && t > camera_near {
                                            depth = t;
                                        }
                                    }
                                }

                                // let depth: f32 = (depth / camera_near).ln() / (camera_far / camera_near).ln();
                                let value: u8 =
                                    (depth * (255.0 / camera_far)).clamp(0.0, 255.0) as u8;

                                let array_vec: ArrayVec<u8, 4> = [value, value, value, 0xFF].into();

                                return array_vec;
                            })
                            .collect()
                    })
                    .collect();

                unsafe {
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                }
                for y in 0..cells_vert {
                    for x in 0..cells_hor {
                        let index = (y * cells_hor) + x;

                        let cell = &cells[index as usize];

                        unsafe {
                            gl::TexSubImage2D(
                                gl::TEXTURE_2D,
                                0,
                                (x * pixels_hor) as i32,
                                (y * pixels_vert) as i32,
                                pixels_hor as i32,
                                pixels_vert as i32,
                                gl::RGBA,
                                gl::UNSIGNED_BYTE,
                                cell[..].as_ptr() as *const _,
                            );
                        }
                    }
                }

                unsafe {
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                    gl::UseProgram(program.id);
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }

                window.swap_buffers().unwrap();
                input.update_states();
            }
            Event::RedrawEventsCleared => {
                window.window().request_redraw();
                // if std::time::Instant::now() > next_update {
                //     window.window().request_redraw();

                //     next_update =
                //         std::time::Instant::now() + std::time::Duration::from_secs_f32(1.0 / 30.0);
                // }
            }
            _ => {}
        }
    })
}

fn pixel_to_camera(x: u32, y: u32, matrix: Matrix4<f32>) -> Ray {
    let ndc_x = ((x as f32) + 0.5) / (VIEWPORT_SIZE.width as f32);
    let ndc_y = ((y as f32) + 0.5) / (VIEWPORT_SIZE.height as f32);

    let screen_x = 2.0 * ndc_x - 1.0;
    let screen_y = 1.0 - 2.0 * ndc_y; // Flipped vertically

    let aspect = (VIEWPORT_SIZE.width as f32) / (VIEWPORT_SIZE.height as f32);
    let fov = 90.0;
    let angle = (fov / 2.0 * std::f32::consts::PI / 180.0).tan();

    let camera_x = screen_x * angle * aspect;
    let camera_y = screen_y * angle;

    let origin: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    let ray_world_origin = matrix.transform_point(origin).to_homogeneous().truncate();
    let ray_world_direction = matrix.transform_vector(Vector::new(camera_x, camera_y, -1.0));

    Ray::new(ray_world_origin, ray_world_direction.normalize())
}
