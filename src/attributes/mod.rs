use rand::Rng;

pub mod color;
pub mod number;
pub mod shading;
pub mod symbol;

fn gen_one_of<R: Rng + ?Sized, A: Copy>(rng: &mut R, arg: &[A]) -> A {
    let index = rng.gen_range(0, arg.len());
    arg[index]
}
