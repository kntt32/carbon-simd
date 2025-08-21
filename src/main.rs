use carbon_simd::*;

fn main() {
    let mut vec = Vec::new();
    for i in 0..=20 {
        vec.push((i - 10) as f32);
    }
    let mut simd = SimdMut::new(&mut vec);
    simd.tanh();
    println!("{:?}", vec);
}
