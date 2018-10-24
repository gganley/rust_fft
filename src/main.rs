extern crate num;
extern crate num_traits;

use num::Complex;

fn main() {
    let mut x: Vec<Complex<f32>> = Vec::new();

    for i in 0..64 {
        x.push(Complex::<f32>::sin(&Complex::<f32>::new(2. * std::f32::consts::PI * i as f32 / 64., 0.)));
    }

    let y = fft(x);
    for i in 0..64 {
        println!("y[{}] = {}", i, y[i].norm());
    }
}

fn fft(mut x: Vec<Complex<f32>>) -> Vec<Complex<f32>>{
    if x.len() == 1 {
        // Bottom
        x
    } else {
        let mut evens = fft(x.iter().step_by(2).cloned().collect());
        let mut odds = fft(x.iter().skip(1).step_by(2).cloned().collect());
        evens.append(&mut odds);
        x = evens;
        for k in 0..(x.len() / 2) {
            let n = x.len();
            let twiddle: Complex<f32> = Complex::<f32>{re: 0., im:((-(2.) * std::f32::consts::PI * k as f32) / n as f32)}.exp();
            let t = x[k];
            x[k] = t + twiddle * x[k+(n /2)];
            x[k+(n / 2)] = t - twiddle * x[k+(n /2)];
        }
        x
    }
}
