use core::fmt;

trait Refill {
	fn refill(&mut self);
}

struct Player {
	healt_points: i32,
}
struct MagicWand {
	magic_points: i32,
}
struct Vehicle {
	fuel_remaining: i32,
}

impl Player {
	const MAX_HEALTH: i32 = 100;
}

impl MagicWand {
	const MAX_MAGIC: i32 = 100;
}

impl Vehicle {
	const MAX_FUEL: i32 = 100;
}

impl Refill for Player {
	fn refill(&mut self) {
		self.healt_points = Self::MAX_HEALTH;
		println!("self.healt_ponits => {}", self.healt_points)
	}
}

impl Refill for MagicWand {
	fn refill(&mut self) {
		self.magic_points = Self::MAX_MAGIC;
		println!("self.magic_ponits => {:?}", self.magic_points)
	}
}

impl Refill for Vehicle {
	fn refill(&mut self) {
		self.fuel_remaining = Self::MAX_FUEL;
		println!("self.fuel_remaining => {}", self.fuel_remaining);
	}
}

impl fmt::Debug for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Player")
			.field("healt_points", &self.healt_points)
			.finish()
	}
}

pub fn execute() {
	let player = Player { healt_points: 50 };
	let magic_wand = MagicWand { magic_points: 30 };
	let vehicle = Vehicle { fuel_remaining: 0 };

	let mut objects: Vec<Box<dyn Refill>> =
		vec![Box::new(player), Box::new(magic_wand), Box::new(vehicle)];

	for obj in objects.iter_mut() {
		obj.refill();
	}
}
