mod common;

trait SystemOfEquations {
    fn len(&self) -> usize;

    fn residuals(&self, x: &[f64], r: &mut [f64]);

    fn derivative(&self, x: &[f64], equation: usize, d: &mut [f64]);
}

struct Paths(Vec<([f64; 3], [f64; 3])>);

impl SystemOfEquations for Paths {
    fn len(&self) -> usize {
        self.0.len() * 3
    }

    fn residuals(&self, x: &[f64], r: &mut [f64]) {
        for p in 0..self.0.len() {
            r[3 * p] = self.0[p].1[0] + self.0[p].0[0] * x[6 + p] - x[3] - x[0] * x[6 + p];
            r[3 * p + 1] = self.0[p].1[1] + self.0[p].0[1] * x[6 + p] - x[4] - x[1] * x[6 + p];
            r[3 * p + 2] = self.0[p].1[2] + self.0[p].0[2] * x[6 + p] - x[5] - x[2] * x[6 + p];
        }
    }

    fn derivative(&self, x: &[f64], equation: usize, d: &mut [f64]) {
        d.fill(0.0);
        let p = equation / 3;
        let i = equation % 3;
        d[0 + i] = -x[6 + p];
        d[3 + i] = -1.0;
        d[6 + p] = self.0[p].0[i] - x[i];
    }
}

fn solve_newton(system: &impl SystemOfEquations, x: &mut [f64]) {
    assert_eq!(system.len(), x.len());
    let mut residuals = vec![0.0; system.len()];
    let mut jacobian = vec![vec![0.0; x.len()]; system.len()];
    let mut step = vec![0.0; x.len()];
    let mut last_error = f64::INFINITY;
    let mut last_best = 0;
    loop {
        system.residuals(x, &mut residuals);
        let error = residuals.iter().map(|&x| x * x).sum::<f64>();
        if error < last_error {
            last_error = error;
            last_best = 0;
        } else {
            last_best += 1;
        }
        for equ in 0..system.len() {
            system.derivative(x, equ, &mut jacobian[equ]);
        }
        common::solve_linear(&mut jacobian, &mut residuals, &mut step);
        for i in 0..x.len() {
            x[i] = x[i] - step[i];
        }
        if last_best > 100 {
            break;
        }
    }
}

fn main() {
    let paths = common::read_input().into_iter().take(3).collect::<Vec<_>>();
    let mut x = vec![0.0; 6 + paths.len()];
    solve_newton(&Paths(paths), &mut x);
    println!("Result: {}", x[3].round() + x[4].round() + x[5].round());
}
