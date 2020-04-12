use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

/*
pub fn new_u32_vec(n: usize) -> Vec<u32> {
  let mut rng = Pcg64Mcg::from_seed([0; 16]);

  let mut v = Vec::with_capacity(n);

  for _ in 0..n {
    v.push(rng.sample(&Starndard));
  }
  v
}
*/

pub fn new_u32_vec(n: usize) -> Vec<u32> {
  let mut rng = Pcg64Mcg::from_seed([0; 16]);
  rng.sample_iter(&Standard).take(n).collect() // 戻り値するのでセミコロン無
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] >= pair[1])
}