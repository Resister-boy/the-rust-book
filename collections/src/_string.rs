pub fn _string() {
  let mut s = String::from("lo");
  s.push('l');

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let mut s3 = s1.clone() + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요
  let mut s4 = s1.clone() + "-" + &s2 + "-";
  let mut s5 = format!("{}-{}", s1.clone(), s2);

  println!("s3-{}, s4-{}, s5{}", s3, s4, s5);
}