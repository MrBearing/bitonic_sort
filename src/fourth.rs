use super::SortOrder;
use std::cmp::Ordering;
use rayon;

const PARALLEL_THRESHOLD: usize = 4096;


pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
  match *order {
    SortOrder::Ascending  => sort_by(x, &|a, b| a.cmp(&b)),
    SortOrder::Descending => sort_by(x, &|a, b| b.cmp(&a)),
  }
}




pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(),String>
  where T: Send,
        F: Sync + Fn(&T, &T) -> Ordering
{
  if x.len().is_power_of_two() {
    do_sort(x, true, comparator);
    Ok(())
  } else {
    Err(format!(" The length of x is not a power of two. (x. len(): {})", x. len()))
  }
}

fn do_sort<T , F>(x: &mut [T], forward: bool , comparator: &F)
    where T: Send,
          F: Sync + Fn(&T, &T) -> Ordering
{
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    // 一度可変の参照を作って要るので2回目に参照を作る事ができずエラーが出る
    // let first = first;
    // let second = second;

    let (first, second) = x.split_at_mut(mid_point);

    if mid_point >= PARALLEL_THRESHOLD{
      rayon::join(|| do_sort(first, true, comparator),
                  || do_sort(second, false, comparator));
    }else{
      do_sort(first, true, comparator);
      do_sort(second, false, comparator);
    }
    sub_sort(x, forward, comparator);
  }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where T: Send,
          F: Sync + Fn(&T, &T) -> Ordering
{ 
  if x.len() > 1 {
    compare_and_swap(x, forward, comparator);
    let mid_point = x.len() / 2;
    let (first, second) = x.split_at_mut(mid_point);
    if mid_point >= PARALLEL_THRESHOLD {
      rayon::join(|| sub_sort(first, forward, comparator),
                  || sub_sort(second, forward, comparator));  
    } else {
      sub_sort(first, forward, comparator);
      sub_sort(second, forward, comparator);
    }
  }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
  let swap_condition = if forward {
    Ordering::Greater
  } else {
    Ordering::Less 
  };

  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if comparator(&x[i] , &x[mid_point + i]) == swap_condition {
      x.swap(i, mid_point+i);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{ sort, sort_by};
  use crate::SortOrder::*;
  use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};
  use std::cmp::Ordering;

  #[derive(Debug, PartialEq)] // トレイトの自動導出
  struct Student {
    first_name: String,
    last_name: String,
    age: u8,
  }

  impl Student {
    fn new(first_name: &str, last_name: &str, age: u8) -> Self {
      Self{
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        age,
      }
    }
  }

  #[test]
  fn sort_u32_large() {
    {
      let mut x = new_u32_vec(65536);
      assert_eq!(sort(&mut x ,&Ascending), Ok(()));
      assert!(is_sorted_ascending(&x));
    }
    {
      let mut x = new_u32_vec(65536);
      assert_eq!(sort(&mut x ,&Descending), Ok(()));
      assert!(is_sorted_descending(&x));
    }
}


  #[test]
  fn ordering_resut(){
    assert_eq!( 14u8. cmp(&16u8), Ordering:: Less); // 14 は 16 よりも 小さい
    assert_eq!( 15u8. cmp(&15u8), Ordering:: Equal); // 15 と 15 は 等しい
    assert_eq!( 16u8. cmp(&14u8), Ordering:: Greater); // 16 は 14 よりも 大きい
  }



  #[test]
  fn sort_students_by_age_ascending(){
    let taro = Student::new(" Taro", "Yamada", 16);
    let hanako = Student::new(" Hanako", "Yamada", 14);
    let kyoko = Student::new(" Kyoko", "Ito", 15);
    let ryosuke = Student::new(" Ryosuke", "Hayashi", 17); // ソート 対象のベクタを作成 する
    let mut x = vec![&taro, &hanako, &kyoko, &ryosuke]; // ソート 後の期待値 を 作成 する
    let expected = vec![&hanako, &kyoko, &taro, &ryosuke]; 

    assert_eq!(
      sort_by(& mut x, &|a, b| a.age.cmp(&b.age)),
      Ok(())
    );
    // 結果 を 検証 する 
    assert_eq!(x, expected);
  }
  
  #[test]
  fn sort_students_by_name(){
    let taro = Student::new(" Taro", "Yamada", 16);
    let hanako = Student::new(" Hanako", "Yamada", 14);
    let kyoko = Student::new(" Kyoko", "Ito", 15);
    let ryosuke = Student::new(" Ryosuke", "Hayashi", 17); // ソート 対象のベクタを作成 する
    let mut x = vec![&taro, &hanako, &kyoko, &ryosuke]; // ソート 後の期待値 を 作成 する 
    let expected = vec![ &ryosuke, &kyoko, &hanako, &taro,]; //名前順(苗字->名前)

    assert_eq!(sort_by(&mut x,
        &|a, b| a.last_name.cmp(&b.last_name)
            .then_with(|| a.first_name.cmp(&b.first_name))), Ok(())
    );
    // 結果 を 検証 する 
    assert_eq!(x, expected);
  }

  #[test]
  fn sort_u32_ascending() {
    let mut x :Vec<u32> = vec![10,30,11,20,4,330,21,110];
    assert_eq!(sort(&mut x, &Ascending), Ok(()));

    assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
  }

  #[test]
  fn sort_u32_decending() {
    let mut x :Vec<u32> = vec![10,30,11,20,4,330,21,110];
    assert_eq!(sort(&mut x, &Descending), Ok(()));

    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
  }

  #[test]
  fn sort_str_ascending(){
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
    assert_eq!(sort(&mut x, &Ascending), Ok(()));
    assert_eq!(x,vec![ "GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
  }
  #[test]
  fn sort_str_decending(){
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
    assert_eq!(sort(&mut x, &Descending), Ok(()));
    assert_eq!(x,vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
  }

  #[test]
  fn sort_to_fail(){
    let mut x = vec![10, 30, 11];
    assert!(sort(&mut x, &Ascending).is_err());
  }

  // コンパイルエラーになる！！
  // #[test]
  // fn sort_mixed(){
  //   if false {
  //     let x = vec![1,2,"a","b"]; // 
  //     sort(&mut x ,false);
  //   }
  // }
  // rustc --explain 308 エラーナンバーを検索してエラー事由を探せる
 
}