use crate::Point;
use crate::Surface;

use sdl2::pixels::Color;


#[derive(Debug, PartialEq, Clone)]
pub struct Plan {
    pub center: Point, // centre du plan
    pub width: f64, // largeur du plan
    pub height: f64, // hauteur du plan
    pub rotation_horizontale: f64, // rotation du plan sur l'axe xz
    pub rotation_verticale: f64, // rotation du plan sur l'axe xy  
    pub surface_type: Surface, // type de surface
    pub color: Color, // couleur du plan
    pub normal: Point, // vecteur normal au plan
    pub point_list: Vec<Point>, // liste des points dans l'ordre du contour
    //pub AZE: f64 = 100.0,
}
impl Plan {
    pub fn new(center: Point, width: f64, height: f64, rotation_horizontale: f64, rotation_verticale: f64, surface_type: Surface, color: Color) -> Plan {
        let mut normal = Point::new(0.0, 0.0, 0.0, 0.0);
        normal.rotation_compared_cam_xy(0.0, rotation_horizontale, rotation_verticale);
        normal.rotation_compared_cam_xz(0.0, rotation_horizontale, rotation_verticale);
        let ratio_total = 1.0 / normal.norm_squared().sqrt();
        normal.x *= ratio_total;
        normal.y *= ratio_total;
        normal.z *= ratio_total;
        //println!("plan.new(), normal: {:?}", normal);
        let point_list: Vec<Point> = Vec::<Point>::new();
        //let plan: Plan = Plan { center, width, height, rotation_horizontale, rotation_verticale, normal, point_list };
        //let haut_gauche = Point::new((center.x - width / 2.0) * normal.x, (center.y + height / 2.0) * normal.y, center.z * normal.z);
        //let haut_droite = Point::new((center.x + width / 2.0) * normal.x, (center.y + height / 2.0) * normal.y, center.z * normal.z);
        //let bas_droite = Point::new((center.x + width / 2.0) * normal.x, (center.y - height / 2.0) * normal.y, center.z * normal.z);
        //let bas_gauche = Point::new((center.x - width / 2.0) * normal.x, (center.y - height / 2.0) * normal.y, center.z * normal.z);
        Plan { center, width, height, rotation_horizontale, rotation_verticale, surface_type, color, normal, point_list }
    }
    pub fn copie(&self) -> Plan {
        let mut point_list: Vec<Point> = Vec::<Point>::new();
        for point in &self.point_list {
            point_list.push(Point::new(point.x, point.y, point.z, point.radius));
        }
        Plan { center: self.center, width: self.width, height: self.height, rotation_horizontale: self.rotation_horizontale, rotation_verticale: self.rotation_verticale, surface_type: self.surface_type, color: self.color, normal: self.normal, point_list: point_list }
    }
    pub fn add_point(&mut self, point: Point) {
        self.point_list.push(point);
    }
    pub fn make_ligne_on_xz(vec: &mut Vec<Plan>, origine: Point, length: f64, direction: f64, orientation_horizontale: f64, orientation_verticale: f64/*, color: Color*/) {
        //let origine = Point::new(10.0, 0.0, 0.0, 0.0);
        //let length = 10.0;
        //let orientation_verticale = 0.0;
        //let orientation_horizontale = 0.0;
        //let direction = 0.0;
        let cos = direction.to_radians().cos();
        let sin = direction.to_radians().sin();
        let mut x = 0.0;
        while x < length {
            let new_centre = Point::new(origine.x + (x * cos), origine.y, origine.z + (x * sin), 0.0);
            if x % 2.0 == 0.0 {
                let new_plan = Plan::new(new_centre, 1.0, 1.0, orientation_horizontale, orientation_verticale, Surface::Opaque, Color::RGB(255, 0, 255));
                vec.push(new_plan);
            } else {
                let new_plan = Plan::new(new_centre, 1.0, 1.0, orientation_horizontale, orientation_verticale, Surface::Opaque, Color::RGB(0, 255, 255));
                vec.push(new_plan);
            }
            x += 1.0;
        }
    }
    pub fn make_ligne_on_xy(vec: &mut Vec<Plan>, origine: Point, length: f64, direction: f64, orientation_horizontale: f64, orientation_verticale: f64/*, color: Color*/) {
        let cos = direction.to_radians().cos();
        let sin = direction.to_radians().sin();
        let mut y = 0.0;
        while y < length {
            let new_centre = Point::new(origine.x + (y * cos), origine.y + (y * sin), origine.z, 0.0);
            if y % 2.0 == 0.0 {
                let new_plan = Plan::new(new_centre, 1.0, 1.0, orientation_horizontale, orientation_verticale, Surface::Opaque, Color::RGB(50, 20, 255));
                vec.push(new_plan);
            } else {
                let new_plan = Plan::new(new_centre, 1.0, 1.0, orientation_horizontale, orientation_verticale, Surface::Opaque, Color::RGB(10, 255, 80));
                vec.push(new_plan);
            }
            y += 1.0;
        }
    }
    pub fn make_tunnel_xz(vec: &mut Vec<Plan>, origine: Point, width: f64, direction: f64/*, orientation_horizontale: f64, orientation_verticale: f64, color: Color*/) {
        let cos = direction.to_radians().cos();
        let sin = direction.to_radians().sin();
        let centre_droite = Point::new(origine.x + (sin * (width / 2.0)), origine.y, origine.z + (cos * (width / 2.0)), 0.0);
        let centre_gauche = Point::new(origine.x - (sin * (width / 2.0)), origine.y, origine.z - (cos * (width / 2.0)), 0.0);
        let centre_dessus = Point::new(origine.x, origine.y + (width / 2.0), origine.z, 0.0);
        let centre_dessous = Point::new(origine.x, origine.y - (width / 2.0), origine.z, 0.0);
        let plan_droite = Plan::new(centre_droite, width, width, direction + 90.0, 0.0, Surface::Opaque, Color::RGB(255, 180, 0));
        let plan_gauche = Plan::new(centre_gauche, width, width, direction + 90.0, 0.0, Surface::Opaque, Color::RGB(255, 180, 0));
        let plan_dessus = Plan::new(centre_dessus, width, width, direction + 90.0, 90.0, Surface::Opaque, Color::RGB(255, 180, 0));
        let plan_dessous = Plan::new(centre_dessous, width, width, direction + 90.0, 90.0, Surface::Opaque, Color::RGB(90, 90, 90));
        vec.push(plan_droite);
        vec.push(plan_gauche);
        vec.push(plan_dessus);
        vec.push(plan_dessous);
    }
    pub fn make_tunnel_xy(vec: &mut Vec<Plan>, origine: Point, width: f64, direction: f64/*, orientation_horizontale: f64, orientation_verticale: f64, color: Color*/) {
        let cos = direction.to_radians().cos();
        let sin = direction.to_radians().sin();
        let centre_droite = Point::new(origine.x + (sin * (width / 2.0)), origine.y + (cos * (width / 2.0)), origine.z, 0.0);
        let centre_gauche = Point::new(origine.x - (sin * (width / 2.0)), origine.y - (cos * (width / 2.0)), origine.z, 0.0);
        let centre_face = Point::new(origine.x + (cos * (width / 2.0)), origine.y, origine.z + (sin * (width / 2.0)), 0.0);
        let centre_dos = Point::new(origine.x - (cos * (width / 2.0)), origine.y, origine.z - (sin * (width / 2.0)), 0.0);
        let plan_droite = Plan::new(centre_droite, width, width, direction + 90.0, direction, Surface::Opaque, Color::RGB(255, 180, 0));
        let plan_gauche = Plan::new(centre_gauche, width, width, direction + 90.0, direction, Surface::Opaque, Color::RGB(255, 180, 0));
        let plan_face = Plan::new(centre_face, width, width, direction, direction, Surface::Opaque, Color::RGB(255, 180, 0));
        let plan_dos = Plan::new(centre_dos, width, width, direction, direction, Surface::Opaque, Color::RGB(90, 90, 90));
        vec.push(plan_droite);
        vec.push(plan_gauche);
        vec.push(plan_face);
        vec.push(plan_dos);
    }
    pub fn make_cube_xz(vec: &mut Vec<Plan>, origine: Point, width: f64, direction: f64/*, orientation_horizontale: f64, orientation_verticale: f64, color: Color*/) {
        let cos = direction.to_radians().cos();
        let sin = direction.to_radians().sin();
        let centre_droite = Point::new(origine.x + (sin * (width / 2.0)), origine.y, origine.z + (cos * (width / 2.0)), 0.0);
        let centre_gauche = Point::new(origine.x - (sin * (width / 2.0)), origine.y, origine.z - (cos * (width / 2.0)), 0.0);
        let centre_dessus = Point::new(origine.x, origine.y + (width / 2.0), origine.z, 0.0);
        let centre_dessous = Point::new(origine.x, origine.y - (width / 2.0), origine.z, 0.0);
        let centre_face = Point::new(origine.x + (cos * (width / 2.0)), origine.y, origine.z + (sin * (width / 2.0)), 0.0);
        let centre_dos = Point::new(origine.x - (cos * (width / 2.0)), origine.y, origine.z - (sin * (width / 2.0)), 0.0);
        let plan_droite = Plan::new(centre_droite, width, width, direction + 90.0, direction, Surface::Opaque, Color::RGB(150, 150, 255));
        let plan_gauche = Plan::new(centre_gauche, width, width, direction + 90.0, direction, Surface::Opaque, Color::RGB(150, 150, 255));
        let plan_face = Plan::new(centre_face, width, width, direction, direction, Surface::Opaque, Color::RGB(150, 150, 255));
        let plan_dos = Plan::new(centre_dos, width, width, direction, direction, Surface::Opaque, Color::RGB(150, 150, 255));
        let plan_dessus = Plan::new(centre_dessus, width, width, direction, direction + 90.0, Surface::Opaque, Color::RGB(150, 150, 255));
        let plan_dessous = Plan::new(centre_dessous, width, width, direction, direction + 90.0, Surface::Opaque, Color::RGB(180, 110, 0));
        vec.push(plan_droite);
        vec.push(plan_gauche);
        vec.push(plan_dessus);
        vec.push(plan_dessous);
        vec.push(plan_face);
        vec.push(plan_dos);
    }
        /*pub fn low_texture(vec: &mut Vec<Plan>) {
        let width = 10.0;
        let height = 10.0;
        let mut center = Point::new(25.0, 0.0, 0.0, 0.0);
        let horizontale: f64 = -45.0;
        let verticale: f64 = 0.0;
        let mut cos_y = verticale.to_radians().cos();
        let sin_y = verticale.to_radians().sin();
        let cos_z = horizontale.to_radians().cos();
        let sin_z = horizontale.to_radians().sin();
        /*if sin_y == 1.0 || sin_y == -1.0 {
            cos = 0.0;
        }*/
        /*if sin_y < 0.0 {
            cos = -cos;
        }*/
        let mut mul_x = 0.0; //sin_y * sin_z;
        if sin_y > sin_z {
            mul_x = sin_y;
        } else {
            mul_x = sin_z;
        }
        /*let mut normal = Point::new(0.0, 0.0, 0.0, 0.0);
        normal.rotation_compared_cam_xy(0.0, horizontale, verticale);
        normal.rotation_compared_cam_xz(0.0, horizontale, verticale);
        let ratio_total = 1.0 / normal.norm_squared().sqrt();
        normal.x *= ratio_total;
        normal.y *= ratio_total;
        normal.z *= ratio_total;*/
        //let chunk_width = 0.5;
        //dist to add/sub = (width/2) - 0.5
        let mut y = 1.0;
        while y < height {
            let mut x = 1.0;
            while x < width {
                let new_centre = Point::new(center.x + ((x - (width/2.0) - 0.5) * mul_x), center.y + ((y - (height/2.0) - 0.5) * cos_y), center.z + ((x- (width/2.0) - 0.5) * cos_z), 0.0);
                //let new_centre = Point::new(center.x, center.y + (y - (height/2.0) - 0.5), center.z + (x - (width/2.0) - 0.5), 0.0);
                let mut color = Color::RGB(0, 0, 0);
              if x-y == 0.0 {
                  color = Color::RGB(0, 255, 255);
              } else {
                  color = Color::RGB(255, 0, 255);
              }
              vec.push(Plan::new(new_centre, 1.0, 1.0, horizontale, verticale, Surface::Opaque, color));
              x += 1.0;
            }
            y += 1.0;
        } 
        //let plan1 = Plan::new(Point::new(0.0, 0.0, 0.0, 0.0), 100.0, 100.0, 0.0, 0.0, Surface::Opaque, Color::RGB(0, 0, 0));
    }*/
    /*
    pub fn draw_plan_rectangle(&self, position: Point, width: f64, heigth: f64, rotation_xz: f64, rotation_xy: f64) {}
    pub fn draw_plan_triangle(&self, position: Point, width: f64, height: f64, rotation_xz: f64, rotation_xy: f64) {}
    pub fn draw_volume_rectangle(&self, position: Point, width: f64, heigth: f64, depth: f64, angle: f64) {}
    pub fn draw_volume_triangle(&self, position: Point, width: f64, height: f64, angle: f64) {}
    */
}