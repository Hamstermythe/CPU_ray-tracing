use crate::Point;
//use crate::Rgba;
use crate::Surface;
use crate::Scene;

use sdl2::pixels::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub position: Point,
    pub radius: f64,
    pub surface_type: Surface, // 0 = opaque, 1 = transparent, 2 = mirror
    pub color: Color,
}
impl Sphere {
    pub fn new(position: Point, radius: f64, surface_type: Surface, color: Color) -> Sphere {
        Sphere { position, radius, surface_type, color }
    }

    pub fn ombre_opaque(&self, scene: &Scene, hit_point: Point) -> Color {
        if scene.light.len() < 1 {
            return Color::RGB(0, 0, 0)
        }
        let mut total_light: f64 = 0.0;
        let mut total_shadow: f64 = 1.0;
        let mut r: f64 = 255.0;
        let mut g: f64 = 255.0;
        let mut b: f64 = 255.0; 
        for light in &scene.light {
            // 100 / light.d3_distance(&hit_point)
            let hit_dist: f64 = light.position.d3_distance(&hit_point);
            let mut light_intensity = 1.0;
            let mut ombre_intensity = 1.0;
            if hit_dist > 0.0 {
                light_intensity = light.radius / light.position.d3_distance(&hit_point); // (100 / 300) = 0.3333; (100 / 200) = 0.5 ; (100/ 10) =  10
                let ratio_ombre: f64 = light.position.d3_distance(&hit_point) - light.position.d3_distance(&self.position) ;
                if ratio_ombre < 0.0 {
                    ombre_intensity = 1.0;
                } else {
                    // WITH AMBIANCE LUMINOSITY
                    ombre_intensity = 0.5 + (scene.light_ambiance_intensity * (1.0 - (ratio_ombre / self.radius)));

                    // WITHOUT ANMBIANCE LUMINOSITY,   SPATIAL WORLD
                    // ombre_intensity = 0.5 + (1.0 - (ratio_ombre / self.radius *2.0));
                }
            }
            
            if light_intensity > 1.0 {
                light_intensity = 1.0;
            }
            if ombre_intensity > 1.0 {
                ombre_intensity = 1.0;
            }
            //println!("{}, {}", light_intensity, ombre_intensity);
            if total_light < light_intensity {
                total_light = light_intensity;
            }
            total_shadow *= ombre_intensity;

            r = (light.color.r as f64 + r as f64) - 255.0;
            g = (light.color.g as f64 + g as f64) - 255.0;
            b = (light.color.b as f64 + b as f64) - 255.0;
        }
        //println!("{}, {}", total_light, total_shadow);
        let color_multiplier: f64 = total_light * total_shadow;
        // ajouter: (couleur_lumiere + couleur_obj) - 255 
        return Color::RGB(
            (self.color.r as f64 * color_multiplier * (r / 255.0)) as u8,
            (self.color.g as f64 * color_multiplier * (g / 255.0)) as u8,
            (self.color.b as f64 * color_multiplier * (b / 255.0)) as u8
        )
    }
}
