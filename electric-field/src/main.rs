use ndarray::{Array2};

// constants
const K: f64 = 8.99e9; // coloumb's constant

const ELEMENTARY_CHARGE: f64 = 1.60217663e-19; // elementary charge 

struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
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

struct Charge {
    magnitude: f64,
    position: Vector2D, // position vector
}

struct ElectricField {
    grid: Grid<Vector2D>, // grid of field strength vectors 
    charges: Vec<Charge>,
}

impl ElectricField {
    fn calc_field_strength(&self, pos: &Vector2D) -> Vector2D { // takes in position vector -> returns field vector
        // Coulomb's Law

        let mut field_strength = Vector2D{ x: 0.0, y: 0.0 };

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
    
}
