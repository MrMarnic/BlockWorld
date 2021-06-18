pub struct LightSource {
    pub level: i32
}

impl LightSource {
    pub fn new(level:i32) -> LightSource {
        return LightSource { level }
    }
}