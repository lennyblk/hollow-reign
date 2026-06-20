pub struct Grace {
    pub id: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

impl Grace {
    pub fn new(id: u32, name: &str, x: f32, y: f32) -> Self {
        Grace { id, name: name.to_string(), x, y }
    }
}
