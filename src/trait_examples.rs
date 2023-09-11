use std::fmt::Display;

pub trait Vehicle {
    fn drive(&self);
    fn fill_up(&self, amount: f64);
}

pub struct Car {

}

impl Vehicle for Car {
    fn drive(&self) {
        todo!()
    }

    fn fill_up(&self, amount: f64) {
        todo!()
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}