use super::{Game, StepData};

pub trait GameStep {
    fn render(&mut self, game:&mut Game);
    fn update(&mut self, game:&mut Game, data:&StepData);
}