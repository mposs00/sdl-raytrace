mod render;
mod vec3;
mod scene;
mod object;
mod light;
mod threadpool;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
 
use vec3::Vec3f;
use object::*;
use scene::Scene;
use light::Light;
use render::Render;

pub fn main() {
    let mut scene = Scene {
        objects: vec![],
        lights: vec![],
        bg_color: Vec3f {
            x: 0.6,
            y: 0.6,
            z: 0.8
        }
    };

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., -10., 0.),
        normal: Vec3f::new(0., 1., 0.),
        material: Material {
            diffuse_color: Vec3f::new(1., 0., 0.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., 10., 0.),
        normal: Vec3f::new(0., -1., 0.),
        material: Material {
            diffuse_color: Vec3f::new(0., 1., 0.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., 0., -20.),
        normal: Vec3f::new(0., 0., 1.),
        material: Material {
            diffuse_color: Vec3f::new(1., 1., 0.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., 0., 10.),
        normal: Vec3f::new(0., 0., -1.),
        material: Material {
            diffuse_color: Vec3f::new(1., 0.5, 0.5),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(10., 0., 0.),
        normal: Vec3f::new(-1., 0., 0.),
        material: Material {
            diffuse_color: Vec3f::new(1., 0., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(-10., 0., 0.),
        normal: Vec3f::new(1., 0., 0.),
        material: Material {
            diffuse_color: Vec3f::new(0., 1., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Sphere {
        center: Vec3f::new(-5., -7., -16.),
        radius: 3.,
        material: Material {
            diffuse_color: Vec3f::new(0., 0., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 50.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Sphere {
        center: Vec3f::new(-2., -8.5, -12.),
        radius: 2.5,
        material: Material {
            diffuse_color: Vec3f::new(1., 1., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 50.,
            reflective: true
        }
    }));

    scene.objects.push(Box::new(Sphere {
        center: Vec3f::new(5., -5., -15.),
        radius: 4.,
        material: Material {
            diffuse_color: Vec3f::new(1., 1., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 50.,
            reflective: true
        }
    }));

    scene.lights.push(Light {
        position: Vec3f::new(-9., 9., -19.),
        intensity: 0.5
    });

    scene.lights.push(Light {
        position: Vec3f::zero(),
        intensity: 0.25
    });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 360, 240)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let (width, height) = canvas.window().size();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut ic = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        ic = (ic + 1) % 65535;
        let shift = (15. * ((ic as f32 * 3.).to_radians().sin() + 1. )) + 60.;
        let zcoord = ((shift / 90.) * -20.) + 20.;

        for j in 0..height {
            for i in 0..width {
                let float_color = Render::render(&scene, i as i32, j as i32, width as usize, height as usize, 65., Vec3f::new(0., 0., zcoord));
                scene.lights[1].position = Vec3f::new(0., 0., zcoord);
                scene.lights[0].position = Vec3f::new((ic as f32 * 0.05).sin() * 10., 9., -19.);
                canvas.set_draw_color(Color::RGB(float_color.x as u8, float_color.y as u8, float_color.z as u8));
                canvas.draw_point(Point::new(i as i32, j as i32));
                
            }
        }
        canvas.present();
        
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}