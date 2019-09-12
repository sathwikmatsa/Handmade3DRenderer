use std::collections::HashMap;
use super::object::Object;
use super::light::Light;

pub struct World {
    objects: Hashmap<usize, Box<Object>>,
    lights: Vec<Light>,
}
