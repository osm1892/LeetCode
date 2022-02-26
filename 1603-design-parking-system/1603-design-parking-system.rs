struct ParkingSystem {
    storage: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            storage: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.storage[car_type as usize - 1] > 0 {
            self.storage[car_type as usize - 1] -= 1;
            true
        } else {
            false
        }
    }
}