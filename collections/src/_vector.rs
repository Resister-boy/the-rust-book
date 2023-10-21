pub fn _vector() {
    let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  let third: Option<&i32> = v.get(2);


  let mut v = vec![100, 32, 57];

  for i in &mut v {
      *i += 50;
  }

  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];

}