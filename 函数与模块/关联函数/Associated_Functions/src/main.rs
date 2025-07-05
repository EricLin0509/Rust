use std::cell::Cell;

fn main() {
    let mut car = Vehicle::create_vehicle("Toyota", "Corolla", 2021);

    car.change_model("Camary");
    println!("{} {} ({})", car.manufacturer, car.model.get(), car.year);

    car.change_year(2025);
    println!("{} {} ({})", car.manufacturer, car.model.get(), car.year);
}

struct Vehicle<'p> {
    manufacturer: String,
    model: Cell<&'p str>,
    year: u32,
}

impl<'p> Vehicle<'p> {
    fn create_vehicle(manufacturer: &str, model: &'p str, year: u32) -> Vehicle<'p> {
        return Vehicle {
            manufacturer: manufacturer.to_string(),
            model: Cell::new(model),
            year,
        };
    }

    fn change_model(&self, new_model: &'p str) {
        self.model.set(new_model);
    }

    fn change_year(&mut self, new_year: u32) {
    self.year = new_year;
    }
}