use crate::Scene;
use crate::Sphere;
use crate::Plan;
use crate::Surface;

//use crate::Color;
use sdl2::pixels::Color;



#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
}
impl Point {
    pub fn new(x: f64, y: f64, z: f64, radius: f64) -> Point {
        Point { x: x, y: y, z: z, radius }
    }
    /////////// GEOMETRY UTILITIES ///////////
    pub fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn sub(&mut self, other: &Point) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
    pub fn mul(&mut self, other: &Point) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
    pub fn div(&mut self, other: &Point) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
    pub fn norm_squared(&self) -> f64 { // Hypoténuse au carré
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /////////// VECTOR RAY UTILITIES ///////////
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn x_angle(&self, other: &Point) -> f64 {
        self.x_distance(other) / self.d3_distance(other)
    }
    pub fn y_angle(&self, other: &Point) -> f64 {
        self.y_distance(other) / self.d3_distance(other)
    }
    pub fn z_angle(&self, other: &Point) -> f64 {
        self.z_distance(other) / self.d3_distance(other)
    }
    pub fn x_distance(&self, other: &Point) -> f64 {
        self.x - other.x
    }
    pub fn y_distance(&self, other: &Point) -> f64 {
        self.y - other.y
    }
    pub fn z_distance(&self, other: &Point) -> f64 {
        self.z - other.z
    }
    // 3d pythagore, Hypoténuse
    pub fn d3_distance(&self, other: &Point) -> f64 {
        (self.x_distance(other).powi(2) + self.y_distance(other).powi(2) + self.z_distance(other).powi(2)).sqrt()
    }
    pub fn intersect(&self, other: &Point, radius: f64) -> bool {
        self.d3_distance(other) <= radius
    }

    //////////// CAMERA UTILITIES ////////////
    // xz Width
    // calculer le vecteur d'un rayon en fonction de la direction de la camera
    pub fn rotation_compared_cam_xz(&mut self, ray_number_xz: f64, rotation_horizontale: f64, rotation_verticale: f64) {
        //let mut rotation_xz: f64 = a_ray.radius * ray_number_xz;//((self.fov / 2.0) / (self.width / 2.0)) * ray_number_xz;
        let mut angle: f64 = rotation_horizontale + (self.radius.abs() * ray_number_xz);
        angle = angle.to_radians();
        //println!("angle: {}, cos {}, sin {}", angle_x, angle_x.cos(), angle_x.sin());
        let mut cos = angle.cos();
        let mut sin = angle.sin();
        if self.x < 0.0 {
            cos = -cos;
        }
        if rotation_verticale == 90.0 || rotation_verticale == -90.0 {
            cos = 0.0;
            sin = 0.0;
        }
        /*if !(rotation_verticale > -90.0 && rotation_verticale < 90.0) {
            cos = -cos;
        }*/
        self.x = cos;
        self.z = sin;
    }
    // xy Height
    // calculer le vecteur d'un rayon en fonction de la direction de la camera
    pub fn rotation_compared_cam_xy(&mut self, ray_number_xy: f64, rotation_horizontale: f64, rotation_verticale: f64) {
        //let mut rotation_xy: f64 = a_ray.radius * ray_number_xy;//((self.fov / 2.0) / (self.height / 2.0)) * ray_number_xy;
        let mut angle: f64 = rotation_verticale + (self.radius.abs() * ray_number_xy);
        angle = angle.to_radians();
        //println!("angle: {}", angle_x);
        let mut cos = angle.cos();
        let sin = angle.sin();
        /*if !(rotation_horizontale >= -90.0 && rotation_horizontale <= 90.0) {
            cos = -cos;
        }*/
        self.x = cos;
        self.y = sin;
    }

    ///////// RAY TRACING /////////
    pub fn ray_tracing(&self, origine: &Point, distance: f64, scene: &Scene) -> Color {
        // Vec<(position_contact, distance)>
        let mut all_contact_sphere: Vec<(Point, Sphere)> = Vec::<(Point, Sphere)>::new();
        let mut all_contact_plan: Vec<(Point, Plan)> = Vec::<(Point, Plan)>::new();
        //////////////////////////////////////////////////////////////////////////////////////
        for sphere in &scene.sphere {
            let mut cam_to_sphere: Point = Point::new(origine.x, origine. y, origine. z, origine.radius);
            cam_to_sphere.sub(&sphere.position);

            let a: f64 = self.norm_squared();
            let half_b: f64 = self.dot(&cam_to_sphere);
            let c: f64 = cam_to_sphere.norm_squared() - (sphere.radius * sphere.radius);
            let discriminant: f64 = (half_b * half_b) - (a * c);
    
            if discriminant < 0.0 {
                continue;
            }
            let sqrt_descriminant: f64 = discriminant.sqrt();
            let mut root: f64 = (-half_b - sqrt_descriminant) / a;           
            if root < 0.0 || root > distance {
                root = (-half_b + sqrt_descriminant) / a;
                if root < 0.0 || root > distance {
                    continue;
                }
            }            

            let hit_point: Point = Point::new(origine.x + (root * self.x), origine.y + (root * self.y), origine.z + (root * self.z), origine.radius);
            if (self.x < 0.0 && hit_point.x > origine.x) || (self.x > 0.0 && hit_point.x < origine.x) {
                continue;
            } else if (self.y < 0.0 && hit_point.y > origine.y) || (self.y > 0.0 && hit_point.y < origine.y) {
                continue;
            } else if (self.z < 0.0 && hit_point.z > origine.z) || (self.z > 0.0 && hit_point.z < origine.z) {
                continue;
            } else {
                //println!("{:?}\n  {:?}\n", self, hit_point);
                all_contact_sphere.push((hit_point, Sphere::new(Point::new(sphere.position.x, sphere.position.y, sphere.position.z, sphere.position.radius), sphere.radius, sphere.surface_type, sphere.color)));
            }
        }
        //////////////////////////////////////////////////////////////////////////////////////
        for plan in &scene.plan {
            /*
    fn is_hit(&self,ray:crate::rendering::ray::Ray, interval:Interval)->Option<Hit> {
        let scalar = ray.direction.dot(self.normal);
        // scalar = 0 => plan // ray => no intersection between plan and ray
        if scalar.abs() < 1e-6{
            return None
        }
        // calculate plan/ray intersection
        let root = (self.normal.dot(self.center - ray.origin)) / scalar;
        if !interval.contains(root){
            return None
        }
        // Calculate vectors to two opposing corners of the square
        let half = self.length / 2.0;
        
        let corner1 = self.center - Vec3::new(half, half, half);
        let corner2 = self.center + Vec3::new(half, half, half);
        
        // Check if the intersection point is within the square's boundaries
        let intersection = ray.at(root);
        if intersection.x >= corner1.x
            && intersection.x <= corner2.x
            && intersection.y >= corner1.y
            && intersection.y <= corner2.y
            && intersection.z >= corner1.z
            && intersection.z <= corner2.z
        {
            
            let hit = Hit::new(intersection + self.normal, self.normal, root, Rc::clone(&self.material),ray.come_from_outside(self.normal)) ;
            return Some(hit);
        }
        None 
    }
            */
            //println!("plan");
            let scalar: f64 = self.dot(&plan.normal);
            let mut d: Point = plan.center;
            d.sub(origine);
            let root = plan.normal.dot(&d) / scalar;
            if root < 0.0 || root > distance {
                //println!("plan out");
                continue;
            }
            let intersection = Point::new(origine.x + (root * self.x), origine.y + (root * self.y), origine.z + (root * self.z), self.radius);

            let half = plan.width / 2.0;
            let corner1 = Point::new(plan.center.x - half, plan.center.y - half, plan.center.z - half, plan.center.radius);
            let corner2 = Point::new(plan.center.x + half, plan.center.y + half, plan.center.z + half, plan.center.radius);
            if intersection.x >= corner1.x && intersection.x <= corner2.x 
             && intersection.y >= corner1.y && intersection.y <= corner2.y 
             && intersection.z >= corner1.z && intersection.z <= corner2.z {
                //println!("plan in");
                all_contact_plan.push((intersection, plan.copie()));//Plan::new(plan.center, plan.width, plan.height, plan.rotation_horizontale, plan.rotation_verticale, plan.surface_type, plan.color)));
            }
        }
        //////////////////////////////////////////////////////////////////////////////////////


        ///////CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
        for contact1 in &all_contact_sphere {
            // contact le plus proche parmis les contacts
            let d1 = origine.d3_distance(&contact1.0);
            let mut fewer: bool = true;
            for contact2 in &all_contact_sphere {
                let d2 = origine.d3_distance(&contact2.0);
                if d2 < d1 /*&& d2 > 0.0*/ {
                    fewer = false;
                    break;
                }
            }
            for contact2 in &all_contact_plan {
                let d2 = origine.d3_distance(&contact2.0);
                if d2 < d1 /*&& d2 > 0.0*/ {
                    fewer = false;
                    break;
                }
            }
            if fewer {
                // retourner la couleur du pixel
                if contact1.1.surface_type == Surface::Opaque {
                    // recuperer le pixel de la sphere
                    // TODO
                    //println!("TOUCH,  {:?}", contact1.1.color);
                    let final_color: Color = contact1.1.ombre_opaque(&scene, contact1.0);
                    //println!("{:?}", final_color);
                    return final_color //contact1.1.color //Color::RGB(contact1.1.color.r, contact1.1.color.g, contact1.1.color.b)
                } else if contact1.1.surface_type == Surface::Transparent {
                    // prolonger le rayon en deformant la trajectoire
                    // TODO
                } else if contact1.1.surface_type == Surface::Mirror {
                    // prolonger le rayon en le reflechissant
                    // TODO
                }
            }
        }
        ///////CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
        for contact1 in &all_contact_plan {
            // contact le plus proche parmis les contacts
            let d1 = origine.d3_distance(&contact1.0);
            let mut fewer: bool = true;
            for contact2 in &all_contact_sphere {
                let d2 = origine.d3_distance(&contact2.0);
                if d2 < d1 /*&& d2 > 0.0*/ {
                    fewer = false;
                    break;
                }
            }
            for contact2 in &all_contact_plan {
                let d2 = origine.d3_distance(&contact2.0);
                if d2 < d1 /*&& d2 > 0.0*/ {
                    fewer = false;
                    break;
                }
            }
            if fewer {
                // retourner la couleur du pixel
                if contact1.1.surface_type == Surface::Opaque {
                    // recuperer le pixel de la sphere
                    // TODO
                    //println!("TOUCH,  {:?}", contact1.1.color);
                    let final_color: Color = contact1.1.color;//contact1.1.ombre_opaque(&scene, contact1.0);
                    //println!("{:?}", final_color);
                    return final_color //contact1.1.color //Color::RGB(contact1.1.color.r, contact1.1.color.g, contact1.1.color.b)
                } else if contact1.1.surface_type == Surface::Transparent {
                    // prolonger le rayon en deformant la trajectoire
                    // TODO
                } else if contact1.1.surface_type == Surface::Mirror {
                    // prolonger le rayon en le reflechissant
                    // TODO
                }
            }
        }
        return Color::RGB(0, 0, 0);
    }
}
