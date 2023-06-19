struct Car {
    owner: String,
}

impl Car {
    fn new(owner: String) -> Self {
        Car { owner }
    }

    fn transfer_ownership(&mut self, new_owner: String) {
        self.owner = new_owner;
    }

    fn display_owner(&self) {
        println!("The current owner of the car is {}.", self.owner);
    }
}

fn main() {
    // Create a car instance with an initial owner
    let mut car1 = Car::new(String::from("John"));

    // Display the initial owner
    car1.display_owner();

    // Transfer ownership of car1
    car1.transfer_ownership(String::from("Alice"));

    // Display the new owner
    car1.display_owner();
}
