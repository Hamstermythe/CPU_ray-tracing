use crate::Plan;
use crate::Sphere;
use crate::Light;



#[derive(Debug, PartialEq, Clone)]
pub struct Scene {
    pub light: Vec<Light>,
    pub triangle: Vec<Plan>,
    pub rectangle: Vec<Plan>,
    pub plan: Vec<Plan>,
    pub sphere: Vec<Sphere>,
    pub light_ambiance_intensity: f64,
}
impl Scene {
    pub fn new() -> Scene {
        let light: Vec<Light> = Vec::<Light>::new();
        let triangle: Vec<Plan> = Vec::<Plan>::new();
        let rectangle: Vec<Plan> = Vec::<Plan>::new();
        let plan: Vec<Plan> = Vec::<Plan>::new();
        let sphere: Vec<Sphere> = Vec::<Sphere>::new();
        let light_ambiance_intensity: f64 = 0.5;
        Scene { light, triangle, rectangle, plan, sphere, light_ambiance_intensity }
    }
    pub fn add_light(&mut self, light: Light) {
        self.light.push(light);
    }
    pub fn add_triangle(&mut self, triangle: Plan) {
        self.triangle.push(triangle);
    }
    pub fn add_rectangle(&mut self, rectangle: Plan) {
        self.rectangle.push(rectangle);
    }
    pub fn add_plan(&mut self, plan: Plan) {
        self.plan.push(plan);
    }
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.sphere.push(sphere);
    }
}
