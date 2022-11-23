//#![windows_subsystem = "windows"]

mod obj_loader;

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::f32::consts::PI;
#[macro_use]
extern crate glium;
extern crate core;

use core::default::Default;
use glium::glutin::dpi::LogicalPosition;
use glium::glutin::window::CursorGrabMode;
use glium::uniforms::UniformValue;
use std::fs;
use std::time::{Instant, SystemTime};

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_maximized(true);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    display.gl_window().window().set_title("Physics Simulation");

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-1.0, -1.0],
    };
    let vertex2 = Vertex {
        position: [1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [1.0, -1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex1 = Vertex {
        position: [1.0, 1.0],
    };
    let vertex2 = Vertex {
        position: [-1.0, -1.0],
    };
    let vertex3 = Vertex {
        position: [-1.0, 1.0],
    };
    let shape2 = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();

    let vertex_shader_src = &fs::read_to_string("shaders/vertex_shader.vert").unwrap();

    let fragment_shader_src = &fs::read_to_string("shaders/fragment_shader.frag").unwrap();

    let mut program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    #[allow(unused_assignments)]
    let mut delta_time = 0.0;

    let device_state = DeviceState::new();

    let mut position = glam::vec3(0.0, 0.0, 20.0);
    let _creation_time = SystemTime::now();

    let mut last_mouse_position: glam::IVec2 = device_state.get_mouse().coords.into();
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let transform = glam::vec3(0.0, 0.0, -1.0);
    let mut start_time = SystemTime::now();

    let mut target_frame_time = 0.0166666;

    //let mut view = glam::mat4(1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0);
    /*
    let cube = obj_loader::load_obj(
        std::path::Path::new("assets/models/triangle.obj"),
        [1.0, 0.0, 1.0, 1.0],
    );*/
    let sphere_0 = PhysicsSphere {
        radius: 5.0,
        mass: 1.0,
        drag_c: 0.0,
        position: glam::vec3(10.0, 30.0, 0.0),
        velocity: glam::vec3(0.0, 0.0, 0.0),
        force: glam::vec3(0.0, 0.0, 0.0),
        no_gravity: false,
        color: [-1.0, -1.0, -1.0, -1.0],
        metal: false,
    };
    let sphere_1 = PhysicsSphere {
        radius: 10000.0,
        mass: 10000000.0,
        drag_c: 0.0,
        position: glam::vec3(0.0, -10000.0, 0.0),
        velocity: glam::vec3(0.0, 0.0, 0.0), //glam::vec3(2.0, 9.81, 0.0) * 5.0,
        force: glam::vec3(0.0, 0.0, 0.0),
        no_gravity: true,
        color: [0.2, 0.5, 0.5, 1.0],
        metal: false,
    };
    let sphere_2 = PhysicsSphere {
        radius: 1.0,
        mass: 1.0,
        drag_c: 0.0,
        position: glam::vec3(-4.0, 23.0, 2.0),
        velocity: Default::default(), //glam::vec3(0.0, 9.81, 0.0) * 4.0,
        force: Default::default(),
        no_gravity: false,
        color: [1.0, 1.0, 1.0, 1.0],
        metal: true,
    };
    let sphere_3 = PhysicsSphere {
        radius: 2.0,
        mass: 1.0,
        drag_c: 0.0,
        position: glam::vec3(2.0, 25.0, 4.0),
        velocity: Default::default(), //glam::vec3(0.0, 9.81, 0.0) * 4.0,
        force: Default::default(),
        no_gravity: false,
        color: [0.1, 0.1, 0.1, 1.0],
        metal: true,
    };

    let mut world = World {
        spheres: vec![sphere_0, sphere_1, sphere_2, sphere_3],
        gravity: glam::vec3(0.0, -9.81, 0.0),
    };
    let mut frames = 0;
    let mut total_time = 0.0;
    let app_start_time = SystemTime::now();

    let mut paused = true;
    let mut hard_shadows = true;
    let mut mouse_locked = true;
    let mut window_focused = true;

    let mut last_frame_keys = vec![];
    event_loop.run(move |event, _, control_flow| {
        //println!("frame");

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    println!(
                        "simulated Time: {}s; system Time {}s",
                        total_time,
                        app_start_time.elapsed().unwrap().as_secs_f32()
                    );
                    println!(
                        "Average FPS: {}",
                        frames as f32 / app_start_time.elapsed().unwrap().as_secs_f32()
                    );

                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Focused(in_focus) => {
                    window_focused = in_focus;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },

            _ => return,
        }
        //let next_frame_time = Instant::now() + std::time::Duration::from_secs_f64(0.01666);
        let next_frame_time =
            Instant::now() + std::time::Duration::from_secs_f64(target_frame_time);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        delta_time = start_time.elapsed().unwrap().as_secs_f32();
        start_time = SystemTime::now();
        total_time += delta_time;
        frames += 1;
        let aspect_ratio = display.get_framebuffer_dimensions().0 as f32
            / display.get_framebuffer_dimensions().1 as f32;

        let speed = 50.0;
        //let mouse = device_state.get_mouse();
        let mut keys = vec![];
        let mut just_pressed_keys = vec![];

        if mouse_locked && window_focused {
            // Mouse Movement
            let relative_x = last_mouse_position.x - device_state.get_mouse().coords.0;
            let relative_y = last_mouse_position.y - device_state.get_mouse().coords.1;
            //position.x += relative_x as f32 * 0.01;
            //position.y -= relative_y as f32 * 0.01;
            yaw -= relative_x as f32 * 0.005;
            pitch += relative_y as f32 * 0.005;
            if pitch > PI * 0.5 {
                pitch = PI * 0.5
            }
            if pitch < -PI * 0.5 {
                pitch = -PI * 0.5
            }

            display.gl_window().window().set_cursor_visible(false);

            display
                .gl_window()
                .window()
                .set_cursor_grab(CursorGrabMode::Locked)
                .or_else(|_e| {
                    display
                        .gl_window()
                        .window()
                        .set_cursor_position(LogicalPosition::new(
                            display.get_framebuffer_dimensions().0 as f32 / 2.0,
                            display.get_framebuffer_dimensions().1 as f32 / 2.0,
                        ))
                    //.set_cursor_grab(CursorGrabMode::Confined)
                })
                .unwrap();
        } else {
            display.gl_window().window().set_cursor_visible(true);
            display
                .gl_window()
                .window()
                .set_cursor_grab(CursorGrabMode::None)
                .unwrap();
        }

        if window_focused {
            keys = device_state.get_keys();

            for key in keys.iter() {
                if !last_frame_keys.contains(key) {
                    just_pressed_keys.push(*key);
                }
            }
        }

        if world.spheres.len() < 100 && frames % 10 == 0 && !paused {
            world.spheres.push(PhysicsSphere {
                radius: 2.0,
                mass: 1.0,
                drag_c: 0.0,
                position: glam::vec3(
                    rand::random::<f32>() - 0.5,
                    rand::random::<f32>() - 0.5,
                    rand::random::<f32>() - 0.5,
                ) + glam::vec3(0.0, 20.0, 0.0),
                velocity: Default::default(), //glam::vec3(0.0, 9.81, 0.0) * 4.0,
                force: Default::default(),
                no_gravity: false,
                color: [
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    1.0,
                ],
                metal: rand::random(),
            })
        }

        if just_pressed_keys.contains(&Keycode::C) {
            if target_frame_time > 0.00001 {
                target_frame_time = 0.0;
            } else {
                target_frame_time = 0.0166666;
            }
        }

        let transform = rotate_y(transform, yaw);
        //let mut direction = glam::vec3(0.0, 0.0, 0.0);
        if mouse_locked {
            if just_pressed_keys.contains(&Keycode::X) {
                world.spheres = vec![sphere_0, sphere_1, sphere_2, sphere_3];
            }
            if keys.contains(&Keycode::A) {
                position -= rotate_y(transform, PI * 0.5) * speed * delta_time;
                //direction = transform * delta_time * speed;
                //position.x -= speed * delta_time;
            }
            if keys.contains(&Keycode::D) {
                position += rotate_y(transform, PI * 0.5) * speed * delta_time;
                //position.x += speed * delta_time;
            }
            if keys.contains(&Keycode::Space) {
                position.y += speed * delta_time;
            }
            if keys.contains(&Keycode::LShift) {
                position.y -= speed * delta_time;
            }
            if keys.contains(&Keycode::W) {
                position += transform * speed * delta_time;
                //position.z -= speed * delta_time;
            }
            if keys.contains(&Keycode::S) {
                position -= transform * speed * delta_time;
                // position.z += speed * delta_time;}
            }
        }
        if keys.contains(&Keycode::LControl) {
            println!("{:?}", world.spheres[0]);
        }
        if keys.contains(&Keycode::F) {
            println!("{} FPS", 1.0 / delta_time);
        }
        if keys.contains(&Keycode::Left) && paused {
            world.tick(-delta_time);
        }
        if keys.contains(&Keycode::Right) && paused {
            world.tick(delta_time);
        }
        if just_pressed_keys.contains(&Keycode::Escape) {
            mouse_locked = !mouse_locked;
        }
        if just_pressed_keys.contains(&Keycode::P) {
            paused = !paused;
        }
        if just_pressed_keys.contains(&Keycode::H) {
            hard_shadows = !hard_shadows;
        }
        if just_pressed_keys.contains(&Keycode::R) {
            let t = Instant::now();

            let vertex_shader_src = &fs::read_to_string("shaders/vertex_shader.vert").unwrap();

            let fragment_shader_src = &fs::read_to_string("shaders/fragment_shader.frag").unwrap();

            let p =
                glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None);

            if p.is_ok() {
                program = p.unwrap();
            } else {
                println!("Failed to recompile shaders: {}", p.err().unwrap());
            }

            println!("Shaders recompiled in {}s", t.elapsed().as_secs_f32());
        }

        last_frame_keys = keys;

        last_mouse_position = glam::IVec2::from(device_state.get_mouse().coords);

        //let sphere_y = std::time::SystemTime::now().duration_since(creation_time).unwrap().as_secs_f32().sin();
        if !paused {
            world.tick(delta_time);
        }

        let my_uniforms = MyUniforms {
            aspect_ratio,
            camera_position: [position.x, position.y, position.z],
            light_dir: [0.5, 1.0, -2.0f32],
            yaw,
            pitch,
            ambient_light: 0.005,
            hard_shadows,
            spheres: world.get_as_uniform(),
            time: total_time,
        };

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &my_uniforms,
                &Default::default(),
            )
            .unwrap();
        target
            .draw(
                &vertex_buffer2,
                &indices,
                &program,
                &my_uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
        //thread::sleep(Duration::from_secs_f64(0.0166667) - start_time.elapsed().unwrap());
        //*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        //println!("{} FPS", delta_time);

        //println!("{} FPS", 1.0 / start_time.elapsed().unwrap().as_secs_f32());
    });
}
#[allow(unused)]
fn mat4_to_uniform(mat: glam::Mat4) -> [[f32; 4]; 4] {
    [
        [mat.x_axis[0], mat.y_axis[0], mat.z_axis[0], mat.w_axis[0]],
        [mat.x_axis[1], mat.y_axis[1], mat.z_axis[1], mat.w_axis[1]],
        [mat.x_axis[2], mat.y_axis[2], mat.z_axis[2], mat.w_axis[2]],
        [mat.x_axis[3], mat.y_axis[3], mat.z_axis[3], mat.w_axis[3]],
    ]
}

fn rotate_y(vec: glam::Vec3, angle: f32) -> glam::Vec3 {
    glam::vec3(
        vec.x * angle.cos() - vec.z * angle.sin(),
        vec.y,
        vec.z * angle.cos() + vec.x * angle.sin(),
    )
}
#[allow(unused)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Triangle {
    color: [f32; 4],
    vertex0: [f32; 3],
    vertex1: [f32; 3],
    vertex2: [f32; 3],
}
#[derive(Copy, Clone, Default)]
struct Sphere {
    radius: f32,
    position: [f32; 3],
    color: [f32; 4],
    metal: bool,
}
const MAX_OBJECTS: usize = 128;
struct MyUniforms {
    pub aspect_ratio: f32,
    pub camera_position: [f32; 3],
    pub light_dir: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub ambient_light: f32,
    pub hard_shadows: bool,
    pub spheres: [Sphere; MAX_OBJECTS],
    pub time: f32,
}
impl glium::uniforms::Uniforms for MyUniforms {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("aspect_ratio", UniformValue::Float(self.aspect_ratio));
        f("camera_position", UniformValue::Vec3(self.camera_position));
        f("light_dir", UniformValue::Vec3(self.light_dir));
        f("yaw", UniformValue::Float(self.yaw));
        f("pitch", UniformValue::Float(self.pitch));
        f("ambient_light", UniformValue::Float(self.ambient_light));
        f("hard_shadows", UniformValue::Bool(self.hard_shadows));
        f("time", UniformValue::Float(self.time));

        /*
        for i in 0..MAX_OBJECTS {
            f(
                &format!("u_triangles[{}].color", i)[..],
                UniformValue::Vec4(self.triangles[i].color),
            );
            f(
                &format!("u_triangles[{}].vertex0", i)[..],
                UniformValue::Vec3(self.triangles[i].vertex0),
            );
            f(
                &format!("u_triangles[{}].vertex1", i)[..],
                UniformValue::Vec3(self.triangles[i].vertex1),
            );
            f(
                &format!("u_triangles[{}].vertex2", i)[..],
                UniformValue::Vec3(self.triangles[i].vertex2),
            );
        }*/
        for i in 0..MAX_OBJECTS {
            f(
                &format!("u_spheres[{}].radius", i)[..],
                UniformValue::Float(self.spheres[i].radius),
            );
            f(
                &format!("u_spheres[{}].position", i)[..],
                UniformValue::Vec3(self.spheres[i].position),
            );
            f(
                &format!("u_spheres[{}].color", i)[..],
                UniformValue::Vec4(self.spheres[i].color),
            );
            f(
                &format!("u_spheres[{}].metal", i)[..],
                UniformValue::Bool(self.spheres[i].metal),
            )
        }
    }
}
#[derive(Debug, Clone)]
struct World {
    pub spheres: Vec<PhysicsSphere>,
    pub gravity: glam::Vec3,
}
impl World {
    pub fn tick(&mut self, dt: f32) {
        let spheres = self.spheres.clone();

        for i in 0..self.spheres.len() {
            let obj = &mut self.spheres[i];
            for j in 0..spheres.len() {
                if i == j {
                    continue;
                }
                let col = obj.collide(&spheres[j]);

                if col.is_some() {
                    obj.force -= col.unwrap() * dt * 100.0;
                }
            }
        }
        for i in 0..self.spheres.len() {
            let obj = &mut self.spheres[i];

            if !obj.no_gravity {
                obj.force += self.gravity * obj.mass * dt;
            }
            obj.tick(dt);
        }
    }
    pub fn get_as_uniform(&self) -> [Sphere; MAX_OBJECTS] {
        let mut spheres = [Sphere::default(); MAX_OBJECTS];
        for i in 0..self.spheres.len() {
            spheres[i] = Sphere {
                radius: self.spheres[i].radius,
                position: <[f32; 3]>::from(self.spheres[i].position),
                color: self.spheres[i].color,
                metal: self.spheres[i].metal,
            }
        }

        spheres
    }
}
#[allow(unused)]
#[derive(Debug, Clone, Copy, Default)]
struct PhysicsSphere {
    pub radius: f32,
    pub mass: f32,
    pub drag_c: f32,
    pub position: glam::Vec3,
    pub velocity: glam::Vec3,
    pub force: glam::Vec3,
    pub no_gravity: bool,
    pub color: [f32; 4],
    pub metal: bool,
}

impl PhysicsSphere {
    pub fn tick(&mut self, dt: f32) {
        self.velocity += self.force / self.mass;
        self.position += self.velocity * dt;
        self.force = glam::vec3(0.0, 0.0, 0.0);
    }
    pub fn collide(&self, other: &Self) -> Option<glam::Vec3> {
        if (other.radius + self.radius).powi(2) > (self.position - other.position).length_squared()
        {
            let depth = (other.radius + self.radius) - (self.position - other.position).length();
            return Some((other.position - self.position).normalize() * depth.abs());
        }
        return None;
    }
}
