use crate::Point;
use crate::Scene;
//use crate::Rgba;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbaImage};
//use crate::Color;
//use crate::Canvas;
//use crate::Window;
use sdl2::pixels::Color;
use sdl2::render::Texture;
//use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
// TIME
use std::time::Duration;
use std::time::Instant;

//use crate::plan::Plan;
//use crate::surface::Surface;
//use crate::sphere::Sphere;
//use crate::light::Light;



#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
    pub position: Point,
    pub distance_de_vue: f64,
    pub direction: Point, // Point as Vec3
    pub rotation_verticale: f64, // angle de vue haut bas, pivot x
    pub rotation_horizontale: f64, // angle de vue gauche droite, pivot x
    pub width: f64, // ray number on width, plan xz
    pub height: f64, // ray number on height, plan xy
    pub fov: f64, // angle d'ouverture de la camera 80° < fov < 120°, default 90°
    //const ray_number_width = 240.0;
    //const ray_number_height = 135.0;
    pub ray_grid: Vec<Vec<Point>>,
    pub echelle: f64,
    //pub image: Vec<Vec<Color>>,
    //pub image: Color,
}
impl Camera {
    fn ray_number_width() -> f64 {
        800.0 // 512.0 // 256.0 // 1920.0 //  64.0 // 96.0 //
    }
    fn ray_number_height() -> f64 {
        450.0 // 288.0 // 144.0 // 1080.0 // 36.0 // 54.0 //
    }
    pub fn new(width: f64, height: f64, angle_ouverture: f64, distance_de_vue: f64) -> Camera {
        let position = Point::new(0.0, 0.0, 0.0, 0.0);
        let direction = Point::new(1.0, 0.0, 0.0, angle_ouverture/Self::ray_number_width()); //Point::new(width*(90/angle_ouverture), 0.0, 0.0);
        let rotation_verticale = 0.0;
        let rotation_horizontale = 0.0;
        let fov = angle_ouverture;
        let ray_grid = Vec::<Vec<Point>>::new();
        let echelle = width/Self::ray_number_width();
        //let image = Vec::<Vec<Color>>::new();
        let mut camera: Camera = Camera { position, distance_de_vue, direction, rotation_verticale, rotation_horizontale, width, height, fov, ray_grid, echelle };//, image };
        //camera.param_grid();
        return camera
    }
    // copie dedié à l'utilisation des threads pour le grid et l'image
    pub fn copie(&self) -> Camera { 
        let position = self.position;
        let distance_de_vue = self.distance_de_vue;
        let direction = self.direction;
        let rotation_verticale = self.rotation_verticale;
        let rotation_horizontale = self.rotation_horizontale;
        let width = self.width;
        let height = self.height;
        let fov = self.fov;
        let ray_grid = self.ray_grid.clone(); //Vec::<Vec<Point>>::new();
        let echelle = self.echelle;
        //let image = Vec::<Vec<Color>>::new();
        Camera { position, distance_de_vue, direction, rotation_verticale, rotation_horizontale, width, height, fov, ray_grid, echelle }//, image }
    }

    ////////////////////////// MOVE //////////////////////////////////////////////////////////////////////
    pub fn avance(&mut self) {
        self.position.x += self.rotation_horizontale.to_radians().cos(); //self.direction.x;
        self.position.z += self.rotation_horizontale.to_radians().sin(); //self.direction.z;
    }
    pub fn recule(& mut self) {
        self.position.x -= self.rotation_horizontale.to_radians().cos(); //self.direction.x;
        self.position.z -= self.rotation_horizontale.to_radians().sin(); //self.direction.z;
    }
    pub fn droite(&mut self) {
        self.position.x += self.rotation_horizontale.to_radians().sin(); //self.direction.z;
        self.position.z -= self.rotation_horizontale.to_radians().cos(); //self.direction.x;
    }
    pub fn gauche(&mut self) {
        self.position.x -= self.rotation_horizontale.to_radians().sin(); //self.direction.z;
        self.position.z += self.rotation_horizontale.to_radians().cos(); //self.direction.x;
    }

    pub fn change_fov(&mut self, fov: f64) {
        self.fov = fov;
        self.direction.radius = fov/self.width;
    }

    ////////////////////// TURN //////////////////////////////////////////////////////////////////////////
    // look at up/ down
    pub fn rotation_xy(&mut self, mut angle: f64) {
        /*if !(self.rotation_verticale + angle > -80.0 && self.rotation_verticale + angle < 80.0) {
            return
        }*/
        self.rotation_verticale += angle;
        if self.rotation_verticale > 180.0 {
            self.rotation_verticale -= 360.0;
        } else if self.rotation_verticale < -180.0 {
            self.rotation_verticale += 360.0;
        }
        /*if self.rotation_verticale > 85.0 {
            self.rotation_verticale = 85.0;
        } else if self.rotation_verticale < -85.0 {
            self.rotation_verticale = -85.0;
        }*/
        println!("rotation verticale: {}", self.rotation_verticale);
        angle = self.rotation_verticale.to_radians();
        let mut cos = angle.cos();
        let sin = angle.sin();
        if !(self.rotation_horizontale >= -90.0 && self.rotation_horizontale <= 90.0) {
            cos = -cos;
        }
        self.direction.x = cos;
        self.direction.y = sin;
        //self.param_grid();
        println!("direction: {:?}", self.direction);
    }
    // look at left/ right
    pub fn rotation_xz(&mut self, mut angle: f64) {
        // ratio x 

        self.rotation_horizontale += angle;
        if self.rotation_horizontale > 180.0 {
            self.rotation_horizontale -= 360.0;
        } else if self.rotation_horizontale < -180.0 {
            self.rotation_horizontale += 360.0;
        }
        println!("rotation horizontal: {}", self.rotation_horizontale); 
        angle = self.rotation_horizontale.to_radians();
        let mut cos = angle.cos();
        let sin = angle.sin();
        if !(self.rotation_verticale >= -90.0 && self.rotation_verticale <= 90.0) {
            cos = -cos;
        }
        self.direction.x = cos;
        self.direction.z = sin;
        //self.param_grid();
        println!("direction: {:?}", self.direction);
    }

    //////////////////////// IMAGE /////////////////////////////////////////////////////////
    pub fn get_image(&mut self, scene: &Scene, chan_sender: Sender<Vec<Vec<Color>>>) { //Vec<Vec<Rgba>> {
        let start = Instant::now();
        //self.param_grid();
        let mut img = Vec::<Vec<Color>>::new();
        let mut y: u32 = 0;
        for ray_ligne in &self.ray_grid {
            let mut x: u32 = 0;
            let mut row = Vec::<Color>::new();
            for ray in ray_ligne {
                /*let (tx, rx): (Sender<Color>, Receiver<Color>) = mpsc::channel();
                let mut children = Vec::new();
                for thread_number in 0..4 {
                    let tx = tx.clone();
                    let ray = ray;
                    let pos = self.position;
                    let ddv = self.distance_de_vue;
                    let scn = scene.clone();
                    children.push(thread::spawn(move || {
                        let rgba = ray.ray_tracing(&pos, ddv, &scn);
                        tx.send(rgba).unwrap();
                    }));
                }
                for child in children {
                    match child.join() {
                        Ok(_) => (),// { row.push(rx.receive().unwrap());  },
                        Err(_) => println!("error"),
                    }
                }*/
                let rgba = ray.ray_tracing(&self.position, self.distance_de_vue, scene);
                //self.image[y as usize][x as usize] = rgba;
                row.push(rgba);
                x += 1;
            }
            img.push(row);
            y += 1;
        }
        chan_sender.send(img).unwrap();
        let duration = start.elapsed();        
        println!("get image duration: {:?}", duration);
        //self.image = img;
    }
    
    pub fn save_image(&self, image: Vec<Vec<Color>>,path: &str) {
        let mut img:RgbaImage = ImageBuffer::new(self.width as u32 +1, self.height as u32 +1);
        let mut y: u32 = 0;
        for pix_ligne in &image { //&self.image {
            let mut x: u32 = 0;
            for pix in pix_ligne {
                img.put_pixel(x, y , image::Rgba([pix.r, pix.g, pix.b, pix.a]));
                x += 1;
            }
            y += 1;
        }
        img.save(path).unwrap();
    }

    ///////////////////////// RAY /////////////////////////////////////////////////
    // xz WIDTH, xy HEIGHT
    pub fn create_cam_ray(&self, ray_number_xz: f64, ray_number_xy: f64) -> Point {
        //let mut a_ray: Point = Point::new(0.0, 0.0, 0.0, self.position.radius);
        let mut a_ray: Point = Point::new(self.direction.x, self.direction.y, self.direction.z, self.direction.radius);
        a_ray.rotation_compared_cam_xy(ray_number_xy, self.rotation_horizontale, self.rotation_verticale);
        a_ray.rotation_compared_cam_xz(ray_number_xz, self.rotation_horizontale, self.rotation_verticale);
        return a_ray
    }
    // create and param the grid of ray starting from the camera
    pub fn param_grid(&mut self/*, grid_sender: Sender<Vec<Vec<Point>>>*/) {
        let start = Instant::now();
        let mut grid = Vec::new();
        let centre_xy_height = Self::ray_number_height() / 2.0; //self.height / 2.0;
        let centre_xz_width =  Self::ray_number_width() / 2.0; //self.width / 2.0;
        let mut y = 0.0;
        while y < Self::ray_number_height() + 1.0 {
            let mut ray_ligne = Vec::new();
            // param angle -45 < angle < 45
            let ray_number_height_y = -(y - centre_xy_height);
            let mut x = 0.0;
            while x < Self::ray_number_width() + 1.0 {
                // param angle -45 < angle < 45
                let ray_number_width_x = -(x - centre_xz_width);
                ray_ligne.push(self.create_cam_ray(ray_number_width_x, ray_number_height_y));
                //println!("");
                x += 1.0;
            }
            grid.push(ray_ligne);
            y += 1.0;
        }
        //grid_sender.send(grid).unwrap();
        self.ray_grid = grid;
        let duration = start.elapsed();        
        println!("param grid duration: {:?}", duration);
    }

    ////////////////////////// CHANNEl COPIE ///////////////////////////////////////////////////////////////
    /*pub fn to_image(&mut self, img: Vec<Vec<Color>>) {
        self.image = img;
    }*/
    pub fn to_grid(&mut self, grid: Vec<Vec<Point>>) {
        self.ray_grid = grid;
    }

    pub fn ray_grid_cuter(&self) -> (Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>) {//, Vec<Vec<Point>>, Vec<Vec<Point>>) {
        //let reste = self.ray_grid.len() % 4;
        let cut_1 = self.ray_grid.len() / 6;
        let cut_2 = cut_1 * 2;
        let cut_3 = cut_1 * 3;// + reste;
        let cut_4 = cut_1 * 4;// + reste;
        let cut_5 = cut_1 * 5;// + reste;
        //let cut_6 = cut_1 * 6;// + reste;
        //let cut_7 = cut_1 * 7;// + reste;

        //let mut arr1 = Vec::new();
        //let mut arr2 = Vec::new();
        //let mut arr3 = Vec::new();
        //let mut arr4 = Vec::new();
        //let mut index = 0;
        /*for ray_ligne in &camera.ray_grid {
            if index < cut_1 {
                arr1.push(ray_ligne);
            } else if index < cut_2 {
                arr2.push(ray_ligne);
            } else if index < cut_3 {
                arr3.push(ray_ligne);
            } else {
                arr4.push(ray_ligne);
            }
            index += 1;
        }*/
        //arr1 = camera.ray_grid[0..cut_1].to_vec();
        //arr2 = camera.ray_grid[cut_1..cut_2].to_vec();
        //arr3 = camera.ray_grid[cut_2..cut_3].to_vec();
        //arr4 = camera.ray_grid[cut_3..].to_vec();
        return (self.ray_grid[0..cut_1].to_vec(), self.ray_grid[cut_1..cut_2].to_vec(), self.ray_grid[cut_2..cut_3].to_vec(), self.ray_grid[cut_3..cut_4].to_vec(), self.ray_grid[cut_4..cut_5].to_vec(), self.ray_grid[cut_5../*cut_6*/].to_vec());//, self.ray_grid[cut_6..cut_7].to_vec(), self.ray_grid[cut_7..].to_vec())
    }
    pub fn ray_grid_vec_cuter(&self, thread_number: i32) -> Vec<Vec<Vec<Point>>> {//, Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>, Vec<Vec<Point>>) {//, Vec<Vec<Point>>, Vec<Vec<Point>>) {
        //let reste = self.ray_grid.len() % 4;
        let cut_1 = self.ray_grid.len() / thread_number as usize;
        //return (self.ray_grid[0..cut_1].to_vec(), self.ray_grid[cut_1..cut_2].to_vec(), self.ray_grid[cut_2..cut_3].to_vec(), self.ray_grid[cut_3..cut_4].to_vec(), self.ray_grid[cut_4..cut_5].to_vec(), self.ray_grid[cut_5../*cut_6*/].to_vec());//, self.ray_grid[cut_6..cut_7].to_vec(), self.ray_grid[cut_7..].to_vec())
        let mut arr = Vec::new();
        for i in 0..thread_number {
            if i == thread_number {
                arr.push(self.ray_grid[((i) * cut_1 as i32) as usize..].to_vec());
            } else {
                arr.push(self.ray_grid[(i * cut_1 as i32) as usize..((i + 1) * cut_1 as i32) as usize].to_vec());
            }
        }
        return arr;
    }
}