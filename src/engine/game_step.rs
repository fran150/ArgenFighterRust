pub trait GameStep {
    fn render(&self);
    fn update(&self, t:u128, dt:u128);
}