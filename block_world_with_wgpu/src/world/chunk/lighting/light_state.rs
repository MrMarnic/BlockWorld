pub struct LightState {
    pub level: i32
}

impl LightState {
    pub fn new(level:i32) -> LightState {
        return LightState { level }
    }
}