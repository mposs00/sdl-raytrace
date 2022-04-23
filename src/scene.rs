use crate::vec3::Vec3f;
use crate::object::Object;
use crate::light::Light;

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub bg_color: Vec3f
}