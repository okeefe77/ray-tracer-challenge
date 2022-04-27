mod cannon;
mod color;
mod point;
mod util;
mod vector;

use cannon::Cannon;
use vector::Vector;
fn main() {
    let mut cannon = Cannon::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
        Vector::new(1.0, 2.0, 0.0).normalize() * 2.0,
    );

    println!("Firing cannon!");
    let distance = cannon.fire();

    println!(
        "The projectile landed {} units away from its starting position.",
        distance
    );
}
