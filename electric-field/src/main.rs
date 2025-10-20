use ndarray::{Array2};

// constants
const K: f64 = 8.99e9; // coloumb's constant

const ELEMENTARY_CHARGE: f64 = 1.60217663e-19; // elementary charge 

struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn delta(&self, dr: &Vector2D) -> Vector2D {
        Vector2D{ x: self.x - dr.x, y: self.y - dr.y }
    }

    fn unit_vector(&self) -> Vector2D {
        let mag = self.magnitude();
        Vector2D{ x: self.x / mag, y: self.y / mag }
    }
}

struct Grid<T> {
    space: Array2<T>
}

impl Grid<T> {
    fn new<T>(size: u64) -> Self {
        Self {
            space: Array2::<T>::zeros((size, size)), // find a way to have empty spots in array for charges (empty charges?)
        }
    }

    fn update<T>(&self, member: T, position: Vector2D) {
        self.space[[position.x, position.y]] = member;
    }
}

struct Charge {
    magnitude: f64,
    position: Vector2D, // position vector
}

struct ElectricField {
    grid: Grid<Vector2D>, // grid of field strength vectors 
    charges: Vec<Charge>,
}

impl ElectricField {
    fn new() -> Self {
        Self {
            grid: Grid::<Charge>new(),
            charges: vec![]
        }
    }

    fn add_charge(&self, position: Vector2D, q: f64) {
        let charge = Charge{ magnitude: q, position: position };

        self.charges.push(charge);
        self.grid.update(charge, position);
    }

    fn calc_field_strength(&self, pos: &Vector2D) -> Vector2D { // takes in position vector -> returns field vector
        // Coulomb's Law

        let mut field_strength = Vector2D::new(0.0, 0.0);

        for charge in self.charges.iter() {
            let dr = pos.delta(&charge.position);

            let dr_mag = dr.magnitude();
            let dr_hat = dr.unit_vector();

            let e_mag = K * charge.magnitude / dr_mag.powi(2); 

            field_strength.x += e_mag * dr_hat.x;
            field_strength.y += e_mag * dr_hat.y;
        }

        field_strength 
    }
}

fn main() {
    // let e_field = ElectricField::new();

    println!("Test");
}
