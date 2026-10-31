
# 变量绑定与解耦

## 命名规范

- Crates 包  无
- Modules `snake_case`
- Types `UpperCamelCase`
- 特征Traits `UpperCamelCase`
- 枚举 Enumerations `UpperCamelCase`
- 结构体 Structs `UpperCamelCase`
- 函数 Functions  `snake_case`
- 方法 Methods `snake_case`
- 通用构造器 `General constructors  new or with_more_details`
- 转换构造器  `from_some_other_type`
- 宏 Macros  `snake_case!`
- 局部变量 `Local variables  snake_case`
- 静态类型 Statics  `SCREAMING_SNAKE_CASE`
- 常量Constants  `SCREAMING_SNAKE_CASE`
- 类型参数Type parameters `UpperCamelCase`
- 生命周期 Lifetimes 小写字母 `'a` `'de` `'src`

## 变量绑定

```Rust
let a = "hello world";
```

在rust里面我们给赋值取了个另外一个名字，叫做 变量绑定。

为什么不用赋值而是绑定？这就涉及到rust最核心的原则—— 所有权。
任何的内存对象都是有主任的，而且一般情况下完全属于它的主人。

绑定就是把一个对象绑定给你个变量，让这个变量成为他的主人。该对象之前的主人就会失去对他的所有权。有点像我们cpp智能指针里的unique_ptr

## 变量可变性
rust的变量在默认情况下是不可变的

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

->
```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error
```

`cannot assign twice to immutable variable x`

## 下划线开头来忽略未使用的变量

如果创建了一个变量但是不用，rust会给你一个警告，如果希望rust不要警告可以用下划线打头。

```rust
fn main() {
  let _x = 5;
  let y = 10;
}
```

## 变量解构

`let` 表达式不仅仅用于变量绑定，还能进行复杂变量的结构: 从一个相对复杂的变量中匹配出该变量的一部分内容

```rust
fn main(){
  let (a, mut b): (bool,bool) = (true,false);
  // a = true, b = false b可变
  println!("a = {:?}, b={:?}",a,b);

  b = true;
  assert_eq!(a,b);
}
```

## 解构式赋值

我们可以在赋值语句的左式中使用元组、切片和结构体模式了

```rust
struct Struct {
  e:i32
}

fn main(){
  let (a, b, c, d, e);

  (a,b) = (1,2);
  // _ 代表匹配一个值，但是不关心具体是啥
  [c,...,d,_] = [1,2,3,4,5];
  Struct {e,..} = Struct {e:5};

  assert_eq!([1,2,1,4,5],[a,b,c,d,e]);
}
```
这种使用方式跟之前的 let 保持了一致性，但是 let 会重新绑定，而这里仅仅是对之前绑定的变量进行再赋值。

## 变量和常量之间的差异
变量的值不能更改可能让我们想起 constant常量，和不可变变量一样，常量也是绑定到一个常量名且不允许更改的值，但是常量和变量之间存在1点差异

- 常量不准用mut， 不仅仅默认不可变，而且始终不可变
- const 关键字 而不是 let 关键字

example:

```rust
const MAX_POINTS: u32 = 100_000;
```
常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效。对于需要在多处代码共享一个不可变的值时非常有用，例如游戏中允许玩家赚取的最大点数或光速。

## 变量遮蔽

Rust 允许声明相同的变量名，后面的会遮蔽掉前面的
```rust

fn main(){
  let x = 5;

  //main的作用域内对前面的遮蔽
  let x = x + 1;

  {
    let x = x * 2;
    println!("The value of x in the inner scope is:{}",x);
  }

  println!("The value of x in the outer scope is:{}",x);
}
```

首先把 5 绑定到 x, 然后重复使用 let x = ... 来遮蔽前面的x， 并取原来的值加上 1 , 所以 x 变成了 6. 第三个 let语句同样这笔前面的 x ， 取之前的值并乘上 2， 得到的x 最终为 12 .

第二个 let 生成的是一个完全的新的变量，两个变量只是刚好名字一样。涉及到了一种内存的再分配，而mut声明的变量，可以修改同一个内存地址上面的值，并不会发生内存对象的再分配。性能要更好。

例如一个程序要统计一个空格字符串的空格数量:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

第一个 `spaces` 变量是一个字符串类型，而第二个则是一个权限的变量并且和第一个共享名字。

相反我们可以重复使用更简单的 spaces 变量名

```rust
let mut spaces = "   ";
spaces = spaces.len();
```
运行的话就会发现编译器报错了

rust对类型的要求很严格，不允许把整数类型 usize赋值给字符串类型。 usize是一种CPU相关的整数类型。

# 基本类型

Rust 的每个值都有其确切的主句类型，总的来说可以分为两类: 基本类型和复合类型。 基本类型意味着他们往往是一个最小化原子类型，无法解构为其他类，一般来说由以下的值组成：

- 有符号整数 `i8` `i16` `i32` `i64` `i128` `isize`
- 无符号整数 `u8` `u16` `u32` `u64` `u128` `usize`
- 浮点数 `f32` `f64`
- 字符串切片 `&str`
- 布尔类型 `bool`
- 字符类型 `char`  表示单个Unicode字符,存储为4个字节
- 单元类型 `()` 也叫做unit类型，表示没有值的类型，类似于C语言中的void类型。

## 类型推导和类型标注

rust是静态类型语言，编译器必须在编译期知道所有变量的类型。 Rust编译器很聪明，可以根据变量的值和上下文中的食用方式来自动推导出变量的类型。

```rust
let guess = "42".parse().expect("Not a number!");
```

这段代码的目的是对字符串42进行解析，而编译器在这里无法推导出我们想要的类型。因此，我们要给编译器guess变量一个显式的类型标注:
`let guess: i32="42".parse().expect("Not a number!");`
或者
`"42".parse::<i32>().expect("Not a number!");`

# 数值类型

## 整数类型
有无符号 + 类型大小(位数) 无符号数表示数字只能取整数 or 0, 有符号则表示数字可以取正数、负数和 0

有符号数字类型的范围是 -(2)^(n-1) 到 2^(n-1)-1, 其中 n 是数字的位数。无符号数字类型的范围是 0 到 2^n-1。

整形字面量可以用下表的形式书写:

| 数字字面量 | 示例 |
| --- | --- |
| 十进制 | 98_222 |
| 十六进制 | 0xff |
| 八进制 | 0o77 |
| 二进制 | 0b1111_0000 |
| 字面量 | b'A' |

rust整形默认使用 i32

## 整形溢出

在debug模式编译的时候，Rust会检查整形溢出，若存在这些问题，则使程序在编译的时候 panic. (崩溃，Rust 使用这个术语来表明程序因为错误而退出)

在当使用 --release参数进行release构建的时候，Rust不检测移除。 相反，当检测到溢出的时候，RUST会采用 补码循环溢出的规则处理。 大于该类型最大值的数值会被补码转换成改类型能够支持的对应数字的最小值。 256 -> 0, 257 0 -> 1 

要显式处理可能的输出，就要采用标准库针对原始数字类型提供的这些方法
- `wrapping_*` 方法在所有模式下都按照补码循环溢出规则处理
- `checked_*` 方法在溢出时返回 None，而不是 panic
- `overflowing_*` 方法在溢出时返回一个元组，包含结果和一个布尔值，表示是否发生了溢出
- `saturating_*` 方法在溢出时返回该类型的最大值或最小值，而不是 panic

```rust
assert_eq!(100u8.saturating_add(1), 101);
assert_eq(u8::MAX.saturating_add(1), u8::MAX);
```

```rust
fn main() {
  let a: u8 = 255;
  let b: a.wrapping_add(20);
  println!("{}",b);
}
```

## 浮点类型

```rust

fn main(){
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

`IEEE-754` 定义标准实现。

### 浮点数陷阱

1. 浮点数是想要数字的近似表达。
2. 浮点数在某些特性是反直觉的。 f32,f64上面的比较是 std::cmp::PartialEq 特征，并没有实现std::cmp::Eq特征

Rust 的 HashMap 数据结构，是一个 KV 类型的 Hash Map 实现，它对于 K 没有特定类型的限制，但是要求能用作 K 的类型必须实现了 std::cmp::Eq 特征，因此这意味着你无法使用浮点数作为 HashMap 的 Key，来存储键值对，但是作为对比，Rust 的整数类型、字符串类型、布尔类型都实现了该特征，因此可以作为 HashMap 的 Key。


```rust

fn main(){
  // 下面的代码会报错
  assert!(0.1 + 0.2 == 0.3);
}
```

非要比较的话，只能说是这样来:
```rust

fn main(){
  let abc:(f32,f32,f32) = (0.1,0.2,0.3);
  let xyz:(f64,f64,f64) = (0.1,0.2,0.3);

  println!("abc (f32)");
  println!("0.1 + 0.2 = {}",(abc.0 + abc.1).to_bits());
  println!("0.3 = {}",(abc.2).to_bits());
  println!();

  println!("xyz (f64)");
  println!("0.1 + 0.2 = {}",(xyz.0 + xyz.1).to_bits());
  println!("0.3 = {}",(xyz.2).to_bits());
  println!();

  assert!(abc.0+abc.1 == abc.2);
  assert!(xyz.0+xyz.1 == xyz.2);
}
```
，对 f32 类型做加法时，0.1 + 0.2 的结果是 3e99999a，0.3 也是 3e99999a，因此 f32 下的 0.1 + 0.2 == 0.3 通过测试，但是到了 f64 类型时，结果就不一样了，因为 f64 精度高很多，因此在小数点非常后面发生了一点微小的变化，0.1 + 0.2 以 4 结尾，但是 0.3 以3结尾，这个细微区别导致 f64 下的测试失败了，并且抛出了异常。

### NaN

对于数学上未定义的结果，例如对负数取平方根，会产生一个特殊的结果: Rust的浮点数类型用 NaN 来处理这些情况

所有的根NaN 交互的结果，都会返回NaN, 并且NaN不能用来比较

```rust
fn main(){
  let x = (-42.0_f32).sqrt();
  assert!(x.x);
}
```

出于防御性的考虑，可以采用is_nan的方法
```rust
fn main(){
  let x = (-42.0_f32).sqrt();
  if x.is_nan() {
    println!("x is NaN");
  }
}
```

## 数字运算
Rust支持所有数字类型的基本数学运算

```rust

fn main(){
  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;

  // remainder
  let remainder = 43 % 5;
}
```

综合性的例子
```rust
fn main() {
  // 编译器会进行自动推导，给予twenty i32的类型
  let twenty = 20;
  // 类型标注
  let twenty_one: i32 = 21;
  // 通过类型后缀的方式进行类型标注：22是i32类型
  let twenty_two = 22i32;

  // 只有同样类型，才能运算
  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  // 对于较长的数字，可以用_进行分割，提升可读性
  let one_million: i64 = 1_000_000;
  println!("{}", one_million.pow(2));

  // 定义一个f32数组，其中42.0会自动被推导为f32类型
  let forty_twos = [
    42.0,
    42f32,
    42.0_f32,
  ];

  // 打印数组中第一个值，并控制小数位为2位
  println!("{:.2}", forty_twos[0]);
}
```


## 位运算

| 运算符 | 描述 |
| --- | --- |
| & | 按位与，相同位置均为1取1，否则 0 |
| | | 按位或，相同位置有1取1，否则 0 |
| ^ | 按位异或，相同位置不同取1，否则 0 |
| ~ | 按位取反，0变1，1变0 |
| ! | 按位取反，0变1，1变0 |
| << | 左移，低位补0 |
| >> | 右移，带符号移动，正数补0，负数补1|


对于移位运算，rust会检查它是否超出该整数的位数范围，超出会报错overflow,

```rust
fn main() {
  let a:u8 = 255;
  let b = a>>7;
  let b = a<<7;
  let b = a>>8;
  let b = a<<8;
}
```

## 序列(Range);

Rust提供了一个非常简洁的方式，用来生成连续的数值，例如 1..5

生成1~4的连续数字，不包含5;

1.. = 5 生成1~5的连续数字，包含5;

```rust
for i in 1..=5 {
  println!("{}", i);
}
```

序列只允许用数字和字符类型。

## 使用As完成类型转换

Rust中可以使用As来完成一个类型到另外一个类型的转换，其最常用于将原始类型转换为其他原始类型。

## 有理数和复数

Rust的标准库和其他语言的标准库相比，准入门槛比较高，因此有理数和复数没有被包含在标准库中。
- 有理数和复数
- 任意大小的整数和任意精度的浮点数
- 固定精度的十进制小数，常用于货币相关。

如何引入 num库？

- 创建新的工程 `cargo new complex_num && cd complex_num`
- 在Cargo.toml 的 `[dependencies]` 下添加 `num = "0.4.0"`


# 总结
- Rust数值类型很多
- 类型转换必须是显示的
- Rust的数值可以在上面使用方法， `13.14_f32.round()`




# 字符，布尔，单元类型

## 字符类型 char

```rust
fn main(){
  let c = 'z';
  let z = 'ℤ';
  let g = '国';
  let heart_eyed_cat = '😻';
}
```

Rust的字符不仅仅是ASC II ， 所有的Unicode值都可以作为Rust字符，包含单个的中文、日文、韩文、emoji符号等等。

`char` 类型表示一个Unicode 标量值，它占用固定的四字节内存与 UTF-32编码单元一致

```rust
fn main(){
  let x = '中';
  println!("字符 '中' 占用了 {} 字节的内存大小",size_of_val(&x));
}
```


## 布尔 bool

Rust中的布尔类型有两个可能的值
```rust
fn main(){
  let t = true;
  let f :bool = false;

  if f{
    println!("这段代码没有被执行");
  }
}
```

## 单元类型Unit


单元类型就是 `()`

唯一的值也是 `()`


main函数就返回这个类型。

没有返回值的函数在rust中是有单独的定义的 
`发散函数(diverging functions)` 顾名思义，无法收敛的函数。

例如常见的println!()的返回值也是单元类型()


# 语句和表达式
Rust的函数体是由一系列语句组成，最后一个表达式来返回值，例如:
```rust
fn add_with_extra(x:i32, y:i32) -> i32{
  let x = x+ 1;
  let y = y + 5;
  x + y;
}
```

语句会执行一些操作但是不会返回一个值，而表达式在球之后会返回一个值。

上面两行是语句，最后一行是表达式，返回值是 x+y 的结果。

rust中，**基于语句和表达式的方式是非常重要的**


## 语句
```rs
let a = 8;
let b:Vec<f64> = Vec::new();
let (a,c) = ("hi", false);
```

以上都是语句，他们完成一个具体的操作，但是没有返回值，因此是语句

由于let是语句，因此不能将let的返回值赋值给别的值

```rs
//error example
let b = (let a = 8);
```

## 表达式

表达式会进行求值，然后返回一个值，例如 5+6 

表达式可以成为语句中的一部分，例如let y = 6 中， 6就是一个表达式。它在求值后返回一个值 6

调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，就可以称为表达式。

```rs
fn main(){
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {}", y);
}
```

上面使用一个语句块表达式给y赋值，该语句是表达式的原因是最后一行是表达式 x+1， 返回了x+1的值。

x+1不能以分号结尾，否则就会从表达式变成语句。表达式不能包含分号，这一点比较重要。如果加上分号就会变成语句，不会返回值，导致y的值是()，也就是单元类型。

```rs

fn main() {
  assert_eq! (ret_unit_type(), ());
}

fn ret_unit_type(){
  let x = 1;

  // if语句块也是一个表达式，因此可以用于赋值

  let y = if x % 2 == 1 {
    "odd"
  } else {
    "even"
  };

  //或者写成一行
  let z = if x % 2 == 1 {"odd"} else {"even"};
}
```



# 函数

rust的函数在之前见过不少了，没有什么难度。

看看这个例子

```rust
fn add(i: i32, j : i32) -> i32{
  i + j
}
```

该函数是如此的简单。

- `fn` 关键字声明一个函数
- `add` 函数名
- `(i: i32, j: i32)` 函数参数列表，参数的类型必须显式标注
- `-> i32` 函数返回值类型

## 函数要点

- 函数名 和 变量名 使用蛇形命名法
- 函数的位置可以随意放置，rust编译器会在编译期进行函数的解析
- 每个函数参数都要标注类型

## 函数参数

```rs
fn main(){
  another_function(5, 6.1);
}

fn another_function(x: i32, y: f32){
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}
```

这里去掉任意一个参数的类型标注，都会报错，rust是静态类型语言，编译器必须在编译期知道所有变量的类型。

## 函数返回
上一节我们提到过，rust函数中，函数就是表达式，因此可以直接把函数的返回值赋给调用者

函数的返回值就是函数体最后一条表达式的返回值，当然我们也可以使用 `return` 提前返回。 下面的函数使用最后一个表达式来返回一个值

```rs

fn plus_five(x:i32) -> i32{
  x + 5
}

fn main(){
  let x = plus_five(5);
  println!("The value of x is: {}", x);
}
```

1. let x = plus_five(5); 我们使用一个函数的返回值来初始化x变量，因此侧面说明了rust中函数也是表达式
2. x + 5 这条语句没有分号，因此是一个表达式，返回值就是 x+5 的结果。

在看一个
```rust

fn plus_or_minus(x:i32) -> i32{
  if x > 5 {
    return x - 5
  }

  x + 5
}

fn main(){
  let x = plus_or_minus(5);
  println!("The value of x is: {}", x);
}
```

## rust中的特殊返回类型


### 无返回值()

- 单元类型() 本质上就是一个空的元组类型，表示没有返回值的类型

```rs

use std::fmt::Debug;

fn report<T: Debug>(item: T) {
  println!("{:?}", item);
}
```
上面这个函数就会隐式返回一个单元类型()，因为没有返回值。

```rs
fn clear(text:&mut String) -> () {
  *text = String::from("");
}
```
上面这个函数显式返回一个单元类型()。

### 发散类型!

用 ！ 作为类型的时候表示函数永不反悔，这种语法通常用于能够让程序崩溃的函数
```rs
fn dead_end() -> ! {
  panic!("程序，你爆炸吧！");
}
```

```rs
fn forerver() -> ! {
  loop {
    println!("我永远不会返回");
  };
}
```

# 所有权和借用

Rust的内存安全性是其最为显著的特征。在以往，内存安全几乎都是通过GC的方式实现，但是GC会引来性能、内存占用的问题。

Rust因此采用了所有权系统。

# 所有权

所有的程序都得和计算机内存打交道，如何从内存中申请空间来存放程序的运行内容，如何在不需要的时候释放这些空间，成了重中之重。

在语言的演变中，产生了三种流派

- GC: 程序运行的时候不断寻找不再使用的内存，eg: java go
- 手动管理: 程序员自己去申请和释放内存，eg: C C++
- 所有权系统: 编译器在编译期就能知道内存的使用情况，eg: Rust

## 例子

```c
int* foo(){
  int a ;
  a = 100;
  char *c = "xyz";
  return &a;
}
```

a和c都是局部变量，但是局部变量a存在栈中，离开作用域之后，a申请的栈上内存都会被回收，从而造成了悬空指针的问题。

变量c是常量字符串，存储在常量区域， "xyz" 只有在整个程序结束后才能回收这片内存。

所以内存安全问题，一直都是程序员很头痛的问题。

## Stack和Heap

### 栈
栈按照顺序存储值，并以相反顺序取出值，也叫做后进先出。

增加顺序叫做进栈，移出数据叫做出栈。

因为上述的实现方式，栈中的所有数据都必须占用已知且固定大小的内存空间。假设数据大小未知，那么在取出数据的时候无法得到想要的。

### 堆

对于大小未知或者可能变化的数据，我们需要将它存储在堆上。

当向堆上放入数据的时候，需要请求一定大小的内存空间，操作系统在堆上某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该地址的指针，该过程被称为在堆上分配内存。

接着，指针会被放到栈中，因为指针的大小是固定的。后续，我们通过栈中的指针来找到堆上的实际位置进而访问该数据

由上可知，堆是一种缺乏组织的数据结构

### 性能区别

栈上分配内存更快。因为在堆上放的时候需要先找到一块足够存放数据的内存空间。接着做一些记录为下一次分配做准备。


### 所有权和堆栈

当你的代码调用一个函数，传递给函数的参数依次被压入栈，调用结束时，这些值将按照相反的顺序被移除。

因为堆上数据缺乏组织，因此跟踪这些数据何时分配和释放时非常重要的。否则堆上的数据将内存泄漏

## 所有权原则

1. Rust中每一个值都被一个变量所拥有，该变量被称为值的所有者。
2. 一个值只能同时被一个变量拥有，一个值只能拥有一个所有者。
3. 当所有者离开作用域时，该值将被丢弃。

### 变量作用域
作用域时一个变量在程序中有效的范围
```rs
let s = "hello"; // s 进入作用域
```

变量s绑定到了一个字符串字面值，该字符串字面值是硬编码到程序代码中的。
`s` 变量从声明的点开始直到当前作用域的结束都是有效的。

### 简单介绍一下string

我们已经见过字符串字面值 `let s : &str = "hello"` , s是被硬编码进程序的字符串值。字符串字面值是很方便的，但是它并不适用于所有场景

- 字符串值是不可变的，因为被硬编码到了程序中
- 并非所有字符串都能在编写代码的时候得知

例如，字符串是需要程序运行的时候由用户动态输入然后存储在内存中的，这种情况下，字符串字面值就完全无用武之地。

Rust提供了动态字符串类型 String, 该类型被分配在堆上，因此可以动态收缩，也就能存储在编译时大小未知的文本。

可以使用下面的方法基于字符串字面量来创建String类型
```rs
let s = String::from("hello");
```

`::` 是一种调用运算符，表示调用String类型的from关联函数。因为string存储在堆上，因此它是动态的。可以这样修改:

```rs

let mut s = String::from("hello");

s.push_str(", world!"); // push_str() 在字符串后追加字面值

println!("{}", s); // 这段代码会打印 `hello, world!`
```

## 变量绑定背后的数据交互

### 转移所有权

```rs
let x = 5;
let y = x;
```

这段代码并没有发生所有权的转移： 代码先将5绑定到变量x，接着拷贝x值给y,最终x和y都等于5. 因为整数是rust基本数据类型，是固定大小的简单值。因此这两个值都是通过自动拷贝的方式来赋值的。


```rs

let s1 = String::from("hello");
let s2 = s1;
```

String是一个复杂类型，由存储在栈中的堆指针，字符串长度，容量共同组成，其中堆指针是最重要的。它指向了真实存储字符串的堆内存。至于长度和容量，就是: 容量指堆内存分配空间的大小，长度是目前已经使用的大小。

例如vector. 我们一般都会存一个capacity,一个size.用来实际存储数据的大小和分配的空间大小。

总之，String类型指向了一个堆上的空间，这里存储着它的真实数据，下面对上面的代码分成两种情况讨论

1. 拷贝String 和 存储在堆上的字节数组。 如果该语句是拷贝所有数据(深拷贝) 那么无论是 String 本身 还是底层的堆上数据，都会被全部拷贝。
这对于性能而言会造成非常大的影响。

2. 只拷贝String本身。这样很快但是如果这样的话，s1和s2都会指向同一块堆内存，这样就会出现问题。因为当s1和s2都离开作用域时，Rust会尝试释放这块堆内存两次，从而造成了悬空指针的问题。

因此，rust这样解决问题。
当 s1被赋给了s2之后，rust会认为s1已经不再有效了，s1的所有权被转移给了s2。也就是说，s1不再是一个有效的变量，不能再使用它。

```rs

let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1); // 这段代码会报错
```

想必读者都听说过: 浅拷贝和深拷贝。那么拷贝指针，长度和容量而不拷贝数据，这听起来就像是浅拷贝。但是又因为Rust同时使用第一个变量`s1`无效了，因此这个操作被称为移动(move)，而不是浅拷贝。

上面的例子可以解读为s1被移动到了s2中。

再看一个

```rs

fn main(){
  let x: &str = "hello";
  let y = x;
  println!("{},{}",x,y);
}
```

这段代码实际上不会报错。因为和String有一个本质的区别，string中 s1持有了通过 String::from() 分配的堆内存，而在这里，x只是引用了存储在二进制可执行文件中的字符串。并没有持有所有权

因此 `let y = x ` 只是对这个值进行了拷贝


### 克隆(深拷贝)

Rust不会自动创建数据的深拷贝，因此，任何自动的复制都不是深拷贝。可以被认为对运行时的性能影响比较小

如果我们确实需要深度复制String中堆上的数据，而不仅仅是栈上的数据，可以使用clone方法

```rs
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

如果性能无关紧要，可以使用clone简化编程体验。

但是对于执行较为频繁的代码，最好别用。

### 拷贝(浅拷贝)

```rs
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
原因是，整形这样的基本类型在编译的时候是已知大小的，会被存储在栈上，所以拷贝实际值是极快的。

Rust有一个叫做Copy的特征，可以用在类似整形这样在栈中存储的类型。

如果一个类型拥有了Copy特征，一个旧的变量在被赋值给其他变量之后仍然可用。

那么什么类型是可以copy的呢？ 一般认为基本类型的组合是可以的。不需要分配内存或者形式资源的类型是可以copy的。

- 整数类型
- 布尔类型
- 浮点类型
- 字符类型
- 元组类型，只要元组中的类型都是Copy的，那么这个元组类型也是Copy的
- 不可变引用 &T

## 函数传值和返回

将值传给函数，一样会发生移动 or 复制

```rs

fn main(){
  let s = String::from("hello");
  takes_ownership(s); // s的值移动到了函数里， 所以之后不能再使用s
  
  println!("{}",s); // 这段代码会报错

  let x = 5; // x是copy的
  makes_copy(x); // 因为i32是Copy的，所以之后仍然可以使用x
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
  println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊的操作
```

同样的，函数的返回值也具有所有权，例如

```rs

fn main(){
  let s1 = gives_ownership(); // gives_ownership 将返回值移动给 s1

  let s2 = String::from("hello"); // s2 进入作用域

  let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中, 它也将返回值移动给 s3
}

fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数
  let some_string = String::from("hello"); // some_string 进入作用域
  some_string // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(some_string: String) -> String { // some_string 进入作用域
  some_string // 返回 some_string 并移出给调用的函数
}
```