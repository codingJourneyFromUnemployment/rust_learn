// fn main() {
//   let mut x = 5;
//   println!("The value of x is: {}", x);
//   x = 6;
//   println!("The value of x is: {}", x);
// }

// fn main() {
//   let _x = 5;
//   let _y = 10;
// }

// fn main() {
//   let (a, mut b) : (bool, bool) = (true, false);
//   // a = ture, 不可变； b = false, 可变
//   println!("a = {:?}, b = {:?}", a, b);

//   b = true;
//   assert_eq!(a, b)
// }

// struct Struct {
//   e: i32
// }

// fn main() {
//   let (a, b, c, d, e);

//   (a, b) = (1, 2);
//   // _代表匹配一个值，但我们不关心具体的值是什么，因此没有使用一个变量名而是使用了_
//   [c, .., d, _] = [1, 2 , 3, 4, 5];
//   Struct { e, ..} = Struct { e: 5 };
//   assert_eq!([a, b, c, d, e], [1, 2, 3, 4, 5])
// }

// fn main() {
//   let x =5;
//   //在main的作用域内对之前的x进行shadowing
//   let x = x + 1;
//   {
//     //在当前花括号的作用域内，对之前的x进行shadowing
//     let x = x * 2;
//     println!("The value of x in the inner scope is: {}", x);
//   }

//   println!("The value of x in the outer scope is: {}", x);
// }