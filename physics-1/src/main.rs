use gnuplot::{Figure, Caption, Color, AxesCommon};

// struct Vec2D {
//     x: f64,
//     y: f64,
// }

#[derive(Clone, Copy, Debug)]
struct State1D {
    x: f64,
    v: f64,
}

fn euler_step<F>(state: State1D, t: f64, dt: f64, accel: F) -> State1D 
where 
    F: Fn(f64, State1D) -> f64
{
    let ax = accel(t, state);
    
    let new_x = state.x + state.v * dt;
    let new_v = state.v + ax * dt;
    
    State1D { x: new_x, v: new_v }
}

fn simulate_1d<F>(s0: State1D, ti: f64, tf: f64, dt: f64, accel: F) -> (Vec<f64>, Vec<f64>, Vec<f64>)
where  
    F: Fn(f64, State1D) -> f64
{
    let mut s = s0;
    let mut t = ti;

    // storage vectors (to return)
    let mut ts = vec![t];
    let mut xs = vec![s.x];
    let mut vs = vec![s.v];

    while t < tf {
        s = euler_step(s, t, dt, &accel);

        t += dt;

        ts.push(t);
        xs.push(s.x);
        vs.push(s.v);
    }

    (ts, xs, vs)
}

fn create_plot(title: &str, x: &Vec<f64>, y: &Vec<f64>, x_label: &str, y_label: &str) -> Figure {


}

fn main() {
    let accel = |_t: f64, _s: State1D| -9.81;
    
    let s0 = State1D { x: 0.0, v: 10.0 };
    let dt = 0.01;
    
    let (ts, xs, vs) = simulate_1d(s0, 0.0, 2.0, dt, accel);

    let n = ts.len() - 1;
    println!(
        "Final state at t = {:.2}s → x = {:.3} m, v = {:.3} m/s",
        ts[n], xs[n], vs[n]
    );

    // Create a line trace of position vs time
    let mut figure = Figure::new();

    figure.axes2d()
        .set_title("1D Motion", &[])
        .lines(&ts, &xs, &[Caption("Euler Method"), Color("blue")]);

    figure.show().unwrap();
}