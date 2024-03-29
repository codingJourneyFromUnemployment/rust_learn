fn main() {
  //使用let来声明变量，进行绑定，a是不可变的
  //此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32, 相当于int32
  //语句的末尾必须以分号结尾
  let a = 10;
  //指定b的类型为i32
  let b: i32 = 20;
  //指定一个可变的变量c
  let mut c = 30i32;
  //在数值和类型中间添加一个下划线，可以提高可读性
  let d = 40_i32;
  //使用一个函数的返回值来作为另一个函数的参数
  let e = add(add(a, b), add(c, d));

  //println!是一个宏调用，看起来像是函数，但是它返回的是宏定义的代码块
  //该函数将指定的格式化字符串输出到标准输出中（控制台）
  //{}是占位符，在具体执行过程中会把e的值代入进来
  println!("(a + b) + (c + d) = {}", e);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i:i32, j:i32) -> i32 {
  //返回相加值，这里可以省略return关键字
  i + j
}