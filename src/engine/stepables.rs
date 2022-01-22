use super::GameStep;

pub struct Stepables<'a> {
    objects: Vec<Box<&'a dyn GameStep>>
}

impl<'a> Stepables<'a> {
    pub fn new() -> Stepables<'a> {
        return Stepables {
            objects: Vec::new()
        }
    }

    pub fn add<T:GameStep>(&mut self, item:&'a T) -> usize {
        self.objects.push(Box::new(item));
        return self.objects.len() - 1;
    }

    pub fn remove(&mut self, index: usize) {
        self.objects.remove(index);
    }

    pub fn get(&self, index:usize) -> &Box<&'a dyn GameStep> {
        &self.objects[index]
    }

    pub fn update(&self, t:u128, dt:u128) {
        for item in &self.objects {
            item.update(t, dt);
        }
    }

    pub fn render(&self) {
        for item in &self.objects {
            item.render();
        }
    }
}