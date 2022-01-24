use super::{GameStep, Game, StepData};

pub struct Stepables {
    objects: Vec<Box<dyn GameStep>>
}

impl Stepables {
    pub fn new() -> Stepables {
        return Stepables {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, item:Box<dyn GameStep>) -> usize {
        self.objects.push(item);
        return self.objects.len() - 1;
    }

    pub fn remove(&mut self, index: usize) {
        self.objects.remove(index);
    }

    pub fn update(&mut self, game: &mut Game, data: &StepData) {
        for item in self.objects.iter_mut() {            
            item.update(game, data);
        }
    }

    pub fn render(&mut self, game: &mut Game) {
        for item in self.objects.iter_mut() {
            item.render(game);
        }
    }
}