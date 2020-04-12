use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
  if x.len().is_power_of_two() {
    match *order {
      SortOrder::Ascending  => do_sort(x, true),
      SortOrder::Descending => do_sort(x, false),
    };
    Ok(())
  } else {
    Err(format!(" The length of x is not a power of two. (x. len(): {})", x. len()))
  }
}

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(),String>
  where F: Fn(&T,T) -> Ordering
{
  if is_power_of_two(x.len()) {
    do_sort(x, true, comparator);
    Ok(())
  } else {
    Err(format!(" The length of x is not a power of two. (x. len(): {})", x. len()))
  }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
  if x.len() > 1 {
    let mid_point = x.len()/2;
    do_sort(&mut x[..mid_point], true);
    do_sort(&mut x[mid_point..], false);
    sub_sort(x,up);
  }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool){ 
  if x.len() > 1 {
    compare_and_swap(x,up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
  } // false の場合の動作は？
}

fn compare_and_swap<T: Ord>(x: &mut [T],up: bool) {
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if (x[i] >  x[mid_point + i]) == up {
      x.swap(i,mid_point+i);
    }
  }

}

#[cfg(test)]
mod tests {
  use super::sort;
  use crate::SortOrder::*; 

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
  fn sort_students_by_age_ascending(){
    let taro = Student::new(" Taro", "Yamada", 16);
    let hanako = Student::new(" Hanako", "Yamada", 14);
    let kyoko = Student::new(" Kyoko", "Ito", 15);
    let ryosuke = Student::new(" Ryosuke", "Hayashi", 17); // ソート 対象 の ベクタ を 作成 する
    let mut x = vec![& taro, &hanako, &kyoko, &ryosuke]; // ソート 後 の 期待値 を 作成 する 
    let expected = vec![& hanako, &kyoko, &taro, &ryosuke]; 
    assert_ eq!( 
      // sort_ by 関数 で ソート する。 第 2 引数 は ソート 順 を 決める クロージャ 
      // 引数 に 2 つ の Student 構造 体 を とり、 age フィールド の 値 を cmp メソッド で 
      // 比較 する こと で 大小 を 決定 する 
      sort_by(& mut x, &|a, b | a. age. cmp(& b. age)), Ok(()) ); // 結果 を 検証 する 
      assert_eq!( x, expected);
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