///// REGARDER RASTERIZATION

mod rendering {
    pub mod camera;
    pub mod scene;
    pub mod light;
}
use rendering::camera::Camera;
use rendering::scene::Scene;
use rendering::light::Light;

mod vectorial {
    pub mod point;
}
use vectorial::point::Point;

mod texture {
    //pub mod color;
    pub mod surface;
    //pub mod image;
}
//use texture::color::Rgba;
use texture::surface::Surface;
//use texture::image::make_image;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbaImage};

mod geometry {
    pub mod plan;
    pub mod sphere;
}
use geometry::plan::Plan;
use geometry::sphere::Sphere;

mod connection {
    pub mod udp;
}
use connection::udp::read_server;



// SDL2
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
//use sdl2::Sdl;
//use sdl2::VideoSubsystem;
use sdl2::render::Canvas;
use sdl2::video::Window;
//use sdl2::render::RenderTarget;

// CHAN
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

// THREAD
use std::thread;
use std::thread::JoinHandle;

// TIME
use std::time::Duration;
use std::time::Instant;

// UDP
use std::net::UdpSocket;

// FUNCTION

/*fn test() {
    //let mut camera: Camera = Camera::new(1920.0/8.0, 1080.0/8.0, 90.0, 50.0);
    let mut camera: Camera = Camera::new(252.0, 144.0, 90.0, 50.0);
    let mut scene: Scene = Scene::new();

    scene.add_sphere(Sphere::new(Point::new(10.0, 0.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(255, 255, 255)));
    scene.add_sphere(Sphere::new(Point::new(-10.0, 0.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(0, 0, 0)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 0.0, 10.0, 1.0), 1.0, Surface::Opaque, Color::RGB(0, 0, 255)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 0.0, -10.0, 1.0), 1.0, Surface::Opaque, Color::RGB(255, 0, 0)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 10.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(255, 255, 0)));
    scene.add_sphere(Sphere::new(Point::new(0.0, -10.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(0, 255, 255)));
    
    // scene.add_sphere(Sphere::new(Point::new(3.0, 0.0, 0.5, 1.0), 2.0, Surface::Opaque, Rgba::new(255, 0, 0, 255)));
    // scene.add_sphere(Sphere::new(Point::new(4.0, 0.5, 0.0, 1.0), 3.0, Surface::Opaque, Rgba::new(0, 0, 255, 255)));

    camera.rotation_xy(180.0);

    //camera.get_image(&scene);
    //camera.save_image("image1.png"); // blanche
    
    camera.rotation_xy(-85.0);


    //camera.get_image(&scene);
    //camera.save_image("image2.png"); // rouge

    camera.rotation_xy(-95.0);    

    //camera.get_image(&scene);
    //camera.save_image("image3.png"); // rouge + vert
    /*
    for (x, y, pixel) in camera.image.enumerate_pixels_mut() {
        // *pixel = Rgba::new(255, 0, 0, 1);
        println!("{:?}", pixel);
    }
    */
    
}*/

pub fn main() -> Result<(), String> {
    //test();
    //imagetest();
    let width = 1920;
    let height = 1080;

    let sdl_context = sdl2::init()?; //.unwrap();
    let video_subsystem = sdl_context.video()?; //.unwrap();
    let window = video_subsystem.window("SDL2 Example", width, height).position_centered().build().map_err(|e| e.to_string())?; //.unwrap();
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut running = true;


    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let mut camera: Camera = Camera::new(width as f64, height as f64, 90.0, 1000.0);
    let mut scene: Scene = Scene::new();
    scene.add_light(Light::new(Point::new(120.0, 120.0, 0.0, 1.0), 100.0, Color::RGB(255, 255, 255)));
    //scene.add_light(Light::new(Point::new(-20.0, -50.0, 30.0, 1.0), 100.0, Color::RGB(255, 255, 255)));
    //scene.add_light(Light::new(Point::new(-200.0, -200.0, 0.0, 1.0), 100.0, Color::RGB(255, 255, 255)));
    
    //scene.add_sphere(Sphere::new(Point::new(1.0, 1.0, 1.0, 1.0), 0.1, Surface::Opaque, Color::RGB(0, 255, 0)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 0.0, 0.0, 1.0), 0.2, Surface::Opaque, Color::RGB(0, 255, 0)));
    scene.add_sphere(Sphere::new(Point::new(10.0, 0.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(255, 255, 255)));
    scene.add_sphere(Sphere::new(Point::new(-10.0, 0.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(60, 90, 90)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 0.0, 10.0, 1.0), 1.0, Surface::Opaque, Color::RGB(0, 0, 255)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 0.0, -10.0, 1.0), 1.0, Surface::Opaque, Color::RGB(255, 0, 0)));
    scene.add_sphere(Sphere::new(Point::new(0.0, 10.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(255, 255, 0)));
    scene.add_sphere(Sphere::new(Point::new(0.0, -10.0, 0.0, 1.0), 1.0, Surface::Opaque, Color::RGB(0, 255, 255)));

    //scene.add_plan(Plan::new(Point::new(30.0, 0.0, 0.0, 0.0), 10.0, 10.0, 0.0, 0.0, Surface::Opaque, Color::RGB(30, 210, 150)));
    //scene.add_plan(Plan::new(Point::new(25.0, -5.0, 0.0, 0.0), 10.0, 10.0, 0.0, 90.0, Surface::Opaque, Color::RGB(0, 200, 140)));
    
    //Plan::low_texture(&mut scene.plan);
    
    Plan::make_ligne_on_xz(&mut scene.plan, Point::new(10.0, 0.0, -5.0, 0.0), 10.0, 45.0, 135.0, 45.0);
    Plan::make_ligne_on_xy(&mut scene.plan, Point::new(10.0, 1.0, -5.0, 0.0), 5.0, 90.0, 90.0, 0.0);

    Plan::make_tunnel_xz(&mut scene.plan, Point::new(10.0, 0.0, 10.0, 0.0), 1.0, 0.0);
    Plan::make_tunnel_xz(&mut scene.plan, Point::new(7.0, 0.0, 13.0, 0.0), 1.0, 45.0);
    Plan::make_tunnel_xz(&mut scene.plan, Point::new(3.0, 0.0, 15.0, 0.0), 1.0, 90.0);

    Plan::make_cube_xz(&mut scene.plan, Point::new(0.0, 38.0, 0.0, 0.0), 80.0, 0.0);

    //camera.rotation_xz(180.0);
    //camera.get_image(&scene);
    //camera.save_image("image.png");

    //println!("{:?}", camera.image);
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width + camera.echelle as u32, height + camera.echelle as u32).map_err(|e| e.to_string())?;

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    //// MULTITHREAD VARIABLES ////

    /////////////////// CHAN //////////////////////////////////////////////////////////////////////////////////
    let (sender_rendu, receiver_rendu): (Sender<(i32, Vec<Vec<Color>>)>, Receiver<(i32, Vec<Vec<Color>>)>) = mpsc::channel();
    
    
    let core_number = num_cpus::get() as i32;
    println!("core_number: {}", core_number);
    let mut thread_number: i32 = 0;
    let mut thread_list = Vec::new();
    let mut chanlist_rendu: Vec<Sender<(Point, f64, Vec<Vec<Point>>, Scene)>> = Vec::new();
    for _ in 0..core_number/2 {
        let copie_sender_rendu = sender_rendu.clone();
        let (sender_rendering, receiver_rendering): (Sender<(Point, f64, Vec<Vec<Point>>, Scene)>, Receiver<(Point, f64, Vec<Vec<Point>>, Scene)>) = mpsc::channel(); // ajouter les valeurs des events avant de renvoyer camera la dedans
        chanlist_rendu.push(sender_rendering);
        //let copie_receiver_rendering = receiver_rendering.clone();
        let r = thread::spawn(move || {
            renderer(receiver_rendering, copie_sender_rendu, thread_number);
        });
        thread_list.push(r);
        thread_number += 1;
    }
    

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    let mut rendu_start = true;
    let mut rendering = false;
    let mut now = Instant::now();
    while running {
        let elapsed_time = now.elapsed();
        if elapsed_time > Duration::from_millis(32) {
            //now = Instant::now();
            // SPEED TEST
            now += Duration::from_millis(32);
            let start = Instant::now();


            // THREAD RENDU
            //let mut thread_number = 0;
            //if rendering {
            let raygrid = camera.ray_grid_vec_cuter(thread_number);
            let mut i_sender = 0;
            for sender in &chanlist_rendu {
                let mut i_grid = 0;
                for grid in &raygrid {
                    if i_grid == i_sender {
                        sender.send((camera.position, camera.distance_de_vue, grid.clone(), scene.clone())).unwrap();
                        break;
                    }
                    i_grid += 1;
                }
                i_sender += 1;
            }       
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // EVENT PUMP
            for event in sdl_context.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        running = false;
                        break;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false;
                        break;
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                        camera.avance();
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => { 
                        camera.recule();
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => { 
                        camera.droite();
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => { 
                        camera.gauche();
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => {
                        camera.rotation_xz(5.0);
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } => { 
                        camera.rotation_xz(-5.0);
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Z), .. } => { 
                        camera.rotation_xy(5.0);
                    }
                    Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => { 
                        camera.rotation_xy(-5.0);
                    }
                    _ => {}
                }
            }
            //canvas.set_pixel(10, 10, Color::RGB(255, 0, 0, 255)).unwrap(); // Set pixel at 10, 10 to red

            // THREAD RAY GRID
            camera.param_grid();

            // RESULT THREAD RENDUd
            let mut partiel_rendu: Vec<(i32, Vec<Vec<Color>>)> = Vec::<(i32, Vec<Vec<Color>>)>::new();
            for _ in 0..thread_number {
                let r = receiver_rendu.recv().unwrap();
                partiel_rendu.push(r);
            }
            let mut rendu = Vec::<Vec<Color>>::new();
            let mut thread_counter: i32 = 0;
            for thread_counter in 0..thread_number {
                for partiel in &partiel_rendu {
                    if partiel.0 == thread_counter {
                        for ligne in &partiel.1 {
                            let mut copie_ligne = Vec::<Color>::new();
                            for pix in ligne {
                                copie_ligne.push(*pix);
                            }
                            rendu.push(copie_ligne);
                        }
                    }
                }
            }
            /*for partiel in partiel_rendu {
                if partiel.0 == thread_counter {
                    for ligne in &partiel.1 {
                        let mut copie_ligne = Vec::<Color>::new();
                        for pix in ligne {
                            copie_ligne.push(*pix);
                        }
                        rendu.push(copie_ligne);
                    }
                }
                thread_counter += 1;
            }*/
            // SCREEN
            texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let start = Instant::now();
                let mut y: u32 = 0;
                for vec in &rendu { //&camera.image {
                    let mut x: u32 = 0;
                    for pixel in vec {
                        //println!("{}",camera.echelle);
                        for i in 0..camera.echelle as u32 {
                            for j in 0..camera.echelle as u32 {
                                //println!("{} {},  {} {}", y, x, y+i, x+j);
                                let offset = (j + (camera.echelle as u32 *y)) * pitch as u32 + (i + (camera.echelle as u32 *x)) * 3;
                                buffer[offset as usize] = pixel.r;
                                buffer[(offset + 1) as usize] = pixel.g;
                                buffer[(offset + 2) as usize] = pixel.b;
                            }
                        }
                        //println!("{}, {}, {}", pixel.r, pixel.g, pixel.b);
                        /*let offset = y * pitch as u32 + x * 3;
                        buffer[offset as usize] = pixel.r;
                        buffer[(offset + 1) as usize] = pixel.g;
                        buffer[(offset + 2) as usize] = pixel.b;*/
                        x += 1;
                    }
                    y += 1;
                }
                let duration = start.elapsed();        
                println!("screen buffer duration: {:?}", duration);
            })?;
            canvas.clear();
            canvas.copy(&texture, None, Some(Rect::new(0, 0, camera.width as u32, camera.height as u32)))?;
            canvas.present();

            // SPEED TEST
            let duration = start.elapsed();        
            println!("total screen thread execution time: {:?}", duration);
           //println!("screen thread execution time: {:?}, {}, {}", duration, camera.ray_grid.len(), rendu.len());
        }
    }
    Ok(())
}

/*pub fn camera_renderer(running: &bool, camera: &mut Camera, scene: &Scene) {
    let mut now = Instant:: now();
    while *running {
        let elapsed_time = now.elapsed();
        if elapsed_time > Duration::from_millis(16) {
            now += Duration::from_millis(16);
            camera.get_image(&scene);
        }
    }
}*/

 //////////////////////// IMAGE /////////////////////////////////////////////////////////
pub fn rendu(position: Point, distance_de_vue: f64, ray_grid: Vec<Vec<Point>>,/* camera: &Camera,*/ scene: &Scene, chan_sender: Sender<(i32, Vec<Vec<Color>>)>, thread_number: i32) { //Vec<Vec<Rgba>> {
    let starter = Instant::now();
    let mut img = Vec::<Vec<Color>>::new();
    //let (tx, rx): (Sender<Color>, Receiver<Color>) = mpsc::channel();
    //let mut children = Vec::new();
    for ray_ligne in &ray_grid { //&camera.ray_grid {//&self.ray_grid {
        //let mut children_row = Vec::new();
        let mut row = Vec::<Color>::new();
        for ray in ray_ligne {
            //for thread_number in x..x+3 {
                //let tx = tx.clone();
                //let rayon = Point::new(ray.x, ray.y, ray.z, ray.radius);
                //let pos = Point::new(position.x, position.y, position.z, position.z);//Point::new(camera.position.x, camera.position.y, camera.position.z, camera.position.radius); //-20.0, 0.0, 0.0, 1.0);
                let ddv: f64 = distance_de_vue;//camera.distance_de_vue;
                //let scn = scene.clone();
                //let a_child = thread::spawn(move || {
                    //let rgba = rayon.ray_tracing(&pos, ddv, &scn);
                    //let rgba = ray.ray_tracing(&pos, ddv, &scn);
                    let rgba = ray.ray_tracing(&position, ddv, scene);
                    row.push(rgba);
                    //tx.send(rgba).unwrap();
                //});
                //children_row.push(a_child);
            //}
        }
        img.push(row);
        //children.push(children_row);
        /*let mut row = Vec::<Color>::new();
        for child in children_row {
            match child.join() {
                Ok(_) => { row.push(rx.recv().unwrap()); },
                Err(_) => { println!("wait"); },
            }
        }
        img.push(row);*/
    }
    /*for children_row in children {
        let mut row = Vec::<Color>::new();
        for child in children_row {
            match child.join() {
                Ok(_) => { row.push(rx.recv().unwrap()); },
                Err(_) => { println!("wait"); },
            }
        }
        img.push(row);
    }*/
    chan_sender.send((thread_number, img)).unwrap();
    let duration = starter.elapsed();        
    println!("get image duration: {:?}", duration);
    //self.image = img;
}
pub fn renderer(environement_receiver: Receiver<(Point, f64, Vec<Vec<Point>>, Scene)>, chan_sender: Sender<(i32, Vec<Vec<Color>>)>, thread_number: i32) { //Vec<Vec<Rgba>> {
    loop {
        let (position, distance_de_vue, ray_grid, scene) = environement_receiver.recv().unwrap();
        //println!("{}, {:?}, {}, {}, {}", thread_number, position, distance_de_vue, ray_grid.len(), scene.plan.len());
        //println!("{}, {:?}", distance_de_vue);
        //println!("{}, {:?}", ray_grid.len());
        //println!("{}, {:?}", scene.plan.len());
        //println!("///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////");
        let starter = Instant::now();

        /*let mut thread_grid: Vec<Vec<Point>> = Vec::<Vec<Point>>::new();
        for i in 0..ray_grid.len() {
            if i == thread_number as usize {
                thread_grid = ray_grid[i].clone();
            }
        }*/


        let mut img = Vec::<Vec<Color>>::new();
        for ray_ligne in &ray_grid { //&thread_grid {
            let mut row = Vec::<Color>::new();
            for ray in ray_ligne {
                let ddv: f64 = distance_de_vue;
                let rgba = ray.ray_tracing(&position, ddv, &scene);
                row.push(rgba);
            }
            img.push(row);
        }
        chan_sender.send((thread_number, img)).unwrap();
        let duration = starter.elapsed();        
        println!("{}, get image duration: {:?}", thread_number, duration);
    }
}

