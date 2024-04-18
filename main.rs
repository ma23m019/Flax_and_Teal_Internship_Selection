fn f(t: f64, y: f64) -> f64 {
    1.0 - t * t + y
}

fn rk4(f: fn(f64, f64) -> f64, a: f64, b: f64, y0: f64, n: usize) {
    let h = (b - a) / n as f64;
    let mut t = a;
    let mut y = y0;

    for _ in 0..n {
        let k1 = h * f(t, y);
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0);
        let k3 = h * f(t + h / 2.0, y + k2 / 2.0);
        let k4 = h * f(t + h, y + k3);

        y += (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        t += h;
        println!("y({:.1}) = {:.4}", t, y)
    }
}

fn main() {
    let a = 0.0;
    let b = 2.0;
    let y0 = 0.5;
    let n = 10;
    println!("\nThe values of y at different values of t are given as follows:\n");
    rk4(f, a, b, y0, n);
}
