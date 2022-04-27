use crate::point::Point;
use crate::vector::Vector;

struct Projectile {
    starting_position: Point,
    current_position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(velocity: Vector) -> Projectile {
        let initial_position = Point::new(0.0, 1.0, 0.0);

        Projectile {
            starting_position: initial_position,
            current_position: initial_position,
            velocity,
        }
    }

    fn has_landed(&self) -> bool {
        self.current_position.y <= 0.0
    }

    fn tick(&mut self, env: &Environment) {
        self.current_position = self.current_position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }

    fn distance_traveled(&self) -> f64 {
        (self.current_position - self.starting_position).magnitude()
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Environment {
        Environment { gravity, wind }
    }
}

pub struct Cannon {
    environment: Environment,
    projectile: Projectile,
}

impl Cannon {
    pub fn new(gravity: Vector, wind: Vector, velocity: Vector) -> Cannon {
        Cannon {
            environment: Environment::new(gravity, wind),
            projectile: Projectile::new(velocity),
        }
    }

    fn tick(&mut self) {
        let p = &mut self.projectile;
        let e = &self.environment;
        p.current_position = p.current_position + p.velocity;
        p.velocity = p.velocity + e.gravity + e.wind;
    }

    pub fn fire(&mut self) -> f64 {
        while !self.projectile.has_landed() {
            self.tick();
        }

        self.projectile.distance_traveled()
    }
}
