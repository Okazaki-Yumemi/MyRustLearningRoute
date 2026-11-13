
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


# 复合类型

复合类型是由其他类型组合而成的，最典型的就是结构体struct和枚举enum，例如平面上的一个点point(x,y)，由两个数值类型的值 x, y 组合而来。

看一段代码:
```rs
#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
  true
}

fn close(f: &mut File) {
  true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> !{
  unimplemented!()
}

fn main() {
  let mut f1 = File::from("f1.txt");
  open(&mut f1);
  close(&mut f1);
}

```

有些方法只提供API结构，不提供具体实现。 此外，有的变量在声明之后并未使用，因此在这个阶段我们需要排除一些编译器噪音。

`read`函数也很有意思，这个地方返回 ! ， 因为 unimplemented告诉编译器这个代码尚未实现。类似的还有 todo!()

# 字符串

首先，rust里面的字符串复杂很多，接下来的内容一定仔细看。

先看一段简单的代码:

```rs
fn main() {
  let my_name = "Pascal";
  greet(my_name);
}

fn greet(name: String){
  println!("Hello, {}!", name);
}
```

greet接受name参数，然后打印出来。

而事实上是，这段代码无法通过编译，因为我们实际上传进去的是&str类型的字符串。

## 切片 slice

对于字符串而言，切片就是对 `String` 类型中的某一部分的引用

```rs
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

使用方括号包括的一个序列：[ 开始索引..终止索引]

开始索引是第一个元素的索引位置，终止索引是最后一个元素的索引位置的下一个位置。右半开区间。

切片数据结构内部都会保存开始的位置和切片的长度，其中长度是通过终止索引 - 开始索引的方式计算得来的


使用Rust的 `..` range序列语法的时候，可以使用以下的方式

```rs
let s = String::from("hello world");
let slice = &s[3..]; // 从索引3开始到结尾
let slice = &s[..3]; // 从开头到索引3(不包含索引3)
let slice = &s[..]; // 从开头到结尾
let slice = &s[0..2]; // 从索引0到索引2(不包含索引2)
```

如果想要切片包含String的最后一个字节

```rs
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len]; // 从索引3到结尾
let slice = &s[0..]; // 从开头到结尾
```


> 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：

```rs
 let s = "中国人";
 let a = &s[0..2];
 println!("{}",a);
```

> 因为我们只取 s 字符串的前两个字节，但是本例中每个汉字占用三个字节，因此没有落在边界处，也就是连 中 字都取不完整，此时程序会直接崩溃退出，如果改成 &s[0..3]，则可以正常通过编译。 因此，当你需要对字符串做切片索引操作时，需要格外小心这一点

看看下面这个代码
```rs
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
```

这个函数会报错。

因为 let word 这个地方，对于传到函数里面的s是一个不可变引用，而s.clear()是一个可变引用。

当我们有了可变的借用的时候，Rust不允许我们再有不可变的借用。因为rust承诺这个不可变借用的引用在整个生命周期内都是有效的。

而后面的println!() 又使用了不可变借用。也是就是在 s.clear()处可变借用和不可变借用试图同时生效。

### 其他切片

```rs

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

## 字符串字面量是切片

```rs
let s: &str = "Hello, world!";
```

该切片指向了程序中可执行文件中的某一个点，这也是为什么字符串字面量是不可变的，因为它们被硬编码进了程序中。

## 什么是字符串

字符串是由字符组成的连续集合。 Rust中的字符是Unicode类型，每个字符占据4个字节的内存空间，但是在字符串中不一样，字符串是UTF-8编码，也就是字符串中的字符所占的字节数是变化的(1-4).

Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。虽然语言级别只有上述的 str 类型，但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。

str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。

## String和 &str的转换

之前的代码中已经见到过好几种从 &str 类型生成 String类型的操作:
- `String::from("hello")`
- `"hello".to_string()`

那么如何将String 转换为 &str呢，取得引用就可以了。

```rs

fn main() {
  let s = String::from("hello,world!");
  say_hello(&s);
  say_hello(&s[..]);
  say_hello(s.as_str());

fn say_hello(s: &str) {
  println!("Hello, {}!", s);  
  }
}
```

## 字符串索引

在其他语言中用索引的方式访问字符串的某个字符或者子串是很正常的，但是在rust中这就会报错。

```rs

let s1 = String::from("hello");
let h = s1[0]; // error
```

### 字符串内部

字符串底层的数据存储格式实际上是 `[u8]`,一个字节数组。

例如`Hola` 的长度是4个字节，因为`Hola`的每个字母在 UTF-8 编码中占用一个字节。

而`中国人`的长度是9个字节，因为每个汉字在 UTF-8 编码中占用三个字节。

这种情况下对hello进行索引，访问 &hello[0] 没有任何意义。


还有一个原因导致了 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。

## 字符串切片

前文提到过，字符串切片很危险，因为切片的索引是按照字节来的，但是字符串优势UTF-8来的，无法保证字节刚好落在字符的边界上

```rs

let hello = "中国人";

let s = &hello[0..2]; // error
```

## 操作字符串

由于String是可变字符串，下面介绍Rust字符串的修改\添加\山出


### 追加

push()方法追加char

push_str()方法追加字符串字面量.

两个方法都是在原有的字符串上追加内容，而不是创建一个新的字符串。

```rs

fn main() {
  let mus s = String::from("hello");

  s.push_str(",rust");
  println!("{}", s); // hello,rust

  s.push('!');
  println!("{}", s); // hello,rust!
}
```

### 插入(Insert)

可以使用 `insert()` 方法插入单个字符`char` ， 也可以使用`insert_str()` 方法插入字符串字面量，与push()方法不同，这俩方法要传入插入位置的索引。二要传入插入的字符串。

由于插入要修改原来的字符串，字符串必须是可变的

```rs
fn main(){
  let mut s = String::from("Hello rust!");

  s.insert(5, ',');
  println!("插入字符 insert() -> {}",s);

  s.insert_str(6, " I like");

  println!("插入字符串 insert_str() -> {}",s);
}

```

结果
```bash
插入字符 insert() -> Hello, rust!
插入字符串 insert_str() -> Hello, I like rust!
```

### 替换

1. replace

适用于 String 类型和 &str类型。 replace方法接受两个参数，第一个是要被替换的字符串，第二个是替换成的字符串。该方法会返回一个新的字符串，而不会修改原来的字符串。

```rs
fn main(){
  let string_replace = String::from("I like rust. Learning rust is my favorite.");

  let new_string = string_replace.replace("rust", "Rust");

  dbg!(new_string);
}
```

2. replacen

接受三个参数，前两个一样，第三个表示替换的次数。该方法会返回一个新的字符串，而不会修改原来的字符串。

```rs

fn main(){
  let string_replace = "I like rust. Learning rust is my favorite.";

  let new_string = string_replace.replacen("rust", "Rust", 1);
  dbg!(new_string);
}
```

3. replace_range

仅适用于String类型。replace_range方法接受两个参数，第一个是要被替换的范围，第二个是替换成的字符串。该方法会修改原来的字符串，而不会返回一个新的字符串。

```rs
fn main(){
  let mut string_replace = String::from("I like rust!");

  string_replace.replace_range(7..11, "Rust");
  dbg!(string_replace);
}
```

### 山出

1. pop

删除并返回最后一个字符。 直接操作原来的字符串。但是存在返回值.如果字符串为空，则返回None

```rs

fn main() {
  let mut string_pop = String::from("rust pop 中文!");

  let p1 = string_pop.pop();
  let p2 = string_pop.pop();
  dbg!(p1);
  dbg!(p2);
  dbg!(string_pop);
}

```
```bash
p1 = Some(
   '!',
)
p2 = Some(
   '文',
)
string_pop = "rust pop 中"
```

2. remove

删除并返回字符串中指定位置的字符

```rs

fn main() {
  let mut string_remove = String::from("测试remove方法");

  println!("string_remove 占 {} 字节",std::mem::size_of_val(string_remove.as_str()));


  //删除第一个汉字
  string_remove.remove(0);
  //下面代码会报错
  //string_remove.remove(1);

  dbg!(string_remove);
}
```

3. truncate

删除字符串中指定位置开始到结尾的全部字符

直接操作原始字符串，无返回值

```rs

fn main(){
  let mut string_truncate = String::from("测试truncate方法");

  string_truncate.truncate(6);
  dbg!(string_truncate);
}
```

```bash
string_truncate = "测试t"
```

4. clear

直接清空字符串，原始字符串变成空字符串

```rs
fn main(){
  let mut string_clear = String::from("测试clear方法");

  string_clear.clear();
  dbg!(string_clear);
}
```

### concatenate

1. `+` ，` +=`


使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型。

调用+相当于调用了 std::string里面的标准库 `add()`方法。这里add()方法的第二个参数是引用类型。

+ 返回一个新的字符串，原来的字符串不变。 += 直接修改原来的字符串。

```rs
fn main(){
  let string_append = String::from("Hello");
  let string_append2 = String::from(" Rust");

  let result = string_append + &string_append2; // 注意这里的 & 符号
  let mut resutl = resutl + "!";

  result += "!!!";

  println!("result = {}", result);
}

```
```bash
result = Hello Rust!!!!
```

```rs
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}
```

```rs
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// String = String + &str + &str + &str + &str
let s = s1 + "-" + &s2 + "-" + &s3;
```

String + &str返回一个 String，然后再继续跟一个 &str 进行 + 操作，返回一个 String 类型，不断循环，最终生成一个 s，也是 String 类型。

`s1`调用`add()`方法之后，所有权被转移给了`add()`，而`add()`调用之后就被释放了，s1也没了。

2. format!宏

```rs
fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

```

## 字符串转义

可以通过转义 \ 的方式输出 ASC II or Unicode

```rs
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```

当然，在某些情况下，可能你会希望保持字符串的原样，不要转义：

```rs

fn main() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

## 操作UTF8

**字符**

如果想用Unicode方式遍历，最好使用 `chars`方法

```rs
for c in "中国人".chars() {
    println!("{}", c);
}
```

输出
```bash
中
国
人
```

**字节**

返回底层字节数组表现形式。
```rs

for b in "中国人".bytes() {
    println!("{}", b);
}
```

output:
```bash
228
184
173
229
155
189
228
186
186
```


**获取子串**

想要准确的从 UTF-8 字符串中获取子串是较为复杂的事情，例如想要从 `holla中国人नमस्ते` 这种变长的字符串中取出某一个子串，使用标准库你是做不到的。 你需要在 crates.io 上搜索 utf8 来寻找想要的功能。

## 深度剖析

为啥 String 可变，而字符串字面值 str 却不可以？

就字符串字面值来说，我们在编译时就知道其内容，最终字面值文本被直接硬编码进可执行文件中，这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性。不幸的是，我们不能为了获得这种性能，而把每一个在编译时大小未知的文本都放进内存，因为有的字符串是在程序运行的过程中动态生成的。

对于string类型，为了支持可变长可增长的文本，需要在堆上面分配一块在编译的时候未知大小的内存。

- 首先向操作系统request memory for string object
- 用完了之后release还给它


其中第一部分由 String::from 完成，它创建了一个全新的 String。

有垃圾回收 GC 的语言中，GC 来负责标记并清除这些不再使用的内存对象，这个过程都是自动完成，无需开发者关心。

而没有gc的话，就要自己手动释放。

rust的话，就是通过变量在离开作用域时自动调用 drop 方法来释放内存。

与其它系统编程语言的 free 函数相同，Rust 也提供了一个释放内存的函数： drop，但是不同的是，其它语言要手动调用 free 来释放每一个变量占用的内存，而 Rust 则在变量离开作用域时，自动调用 drop 函数

# 元组

元组是多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组的顺序也是固定的。

```rs
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

变量`tpu`被绑定了一个元组值 `(500,6.4,1)`, 该元组的类型是`(i32, f64, u8)` 

元组使用括号将多个类型组合到一起。

## 用模型匹配解构元组

```rs
fn main() {
  let tup = (500, 6.4, 1);

  let (x,y,z) = tup;

  println!("The value of y is: {}", y);
}
```

上述代码首先创建一个元组，然后帮他绑定到tup上面，然后用 `let( x,y,z) = tup`来完成模式匹配。。

## 用 `.` 来访问元组

模式匹配可以让我们一次性的把元组中的值取出来，但是如果只是想要访问特定的元素，那么模式匹配就略显繁琐，对此，Rust提供了`.`的访问方式:

```rs

fn main(){
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
}
```

## 元组的使用示例

元组在函数返回值非常有用，例如和下面的代码，可以使用元组返回多个值
```rs
fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() 返回字符串的长度

  (s, length)
}
```

`calculate_length` 函数接收 `s1` 字符串的所有权，然后计算的字符串的长度，接着把字符串所有穿和字符串长度再返回给`s2` 和 `len` 变量

# 结构体

结构体 `struct` 恰恰就是复合数据解构，不过可能有不同的名称

## 结构体语法

**定义结构体**

- 通过关键字 struct 定义
- 一个清晰明确的结构体 `名称`
- 几个有名字的结构体 `字段`

例如:
```rs
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```

该结构体名称是User，拥有四个字段，且每个字段都有对应的字段名及其类型声明，例如Username 代表用户名，类型是string

**创建结构体实例**:

```rs

let userl = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
};
```

有几点值得注意:
1. 初始化实例的时候，每个字段都要进行初始化
2. 初始化时的字段顺序不需要和结构体定义的顺序一致

**访问结构体字段**:

```rs

let mut userl = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
};
```

需要注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，Rust不支持将某个结构体的某个字段标记为可变。

**简化结构体创建**：

构建函数
```rs
fn build_user(email: String, username: String) -> User{
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}
```

他接受两个字符串参数 `email` 和 `username`，然后创建一个User结构体。

事实上，可以通过略缩的方式初始化

```rs
fn build_user(email:String , username: String) -> User{
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```
因为上面和下面有一样的字段名，所以可以通过略缩的方式初始化，和Typescript的方式差不多。


**结构体更新语法**:

在实际场景中，有一种情况很常见: 根据已有的结构体创建新的结构体

```rs

let user2 = User {
  active : user1.active,
  username: user1.username,
  email: String::from("another@email.com"),
  sign_in_count: user1.sign_in_count,
};
```

同时，Rust还有结构体更新语法

```rs

let user2 = User {
  email: String::from("another@example.com"),
  ..user1
};
```

因为user2仅仅在email上面和user1不同，因此我们只需要对 email进行赋值。剩下的通过结构体更新语法 ..user1 即可完成。

`..` 语法表明是凡是没有声明的字符都从user1里面获取

..user1 必须在结构体的尾部使用

> 结构体更新语法跟赋值语句 `=` 很类似。因此在上面代码中 user1的部分字段所有权被转移到了user2中，Username字段发生了所有权转移。作为结构，User1无法再被使用。

在上面，只有username发生了所有权转移，因为username是string类型，没有实现Copy特征，因此发生了所有权转移。而active和sign_in_count是基本类型，发生了拷贝。

user1无法再被使用，但是user1内部的其他字段还是能正常用的。

```rs

# #[derive(Debug)]
# struct User {
#   active: bool,
#   username: String,
#   email: String,
#   sign_in_count: u64,
# }
# fn main(){
  let user1 = User {
    email: String::from("someone.example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  }; 
  let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("anohter@example.com"),
    sign_in_count: user1.sign_in_count,
  };

  println!("{}", user1.active); // true

  println!("{:?}", user1); // error
#}
```

## 结构体中的内存排列

```rs

#[derive(Debug)]

struct File{
  name: String,
  data: Vec<u8>,
}

fn main(){
  let f1 = File {
    name: String::from("f1.txt"),
    data: Vec::new(),
  };

  let f1_name = &f1.name;
  let f1_length = &f1.data.len();

  println!("{:?}", f1);
  println!("{} is {} bytes long", f1_name, f1_length);
}

```

上述的File结构体在内存中，其name和data两个字段分别拥有底层量[u8] 数组的所有权 (String 类型的底层也是[u8]数组), 通过ptr 指针指向底层数组的内存地址.

把结构体中具有所有权的字段转移出去后，就无法访问该字段，但是可以正常访问其它的字段.

## 元组结构体

结构体必须要有名称，但是结构体的字段可以没有名称。这种结构类似于元组。所以被叫做元组结构体.

```rs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);
```

## 单元结构体(Unit-like Struct)

单元结构体和单元类型类似，没有任何字段和属性。

如果我们定义一个类型，但是不关心其内容，只关心行为，就可以用`单元结构体`

```rs

struct AlwaysEqual;

let subject = AlwaysEqual;
// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征

impl SomeTrait for AlwaysEqual {
  // 方法实现
} 
```

## 结构体数据的所有权

在之前User 结构体的定义中，有一处细节，我们使用了自身拥有所有权的String类型而不是基于引用的`&str`类型。 

因为我们想要这个结构体拥有它所有的数据，而不是从其他地方借用数据。

你也可以让User结构体从其他地方借用数据。

这种东西就需要引入 `生命周期` 的概念了。 生命周期能够确保结构体的作用范围要比它所借用的数据的作用范围要小。

```rs

struct User {
  username: &str,
  email: &str,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let userl = User {
    email: "someone@example.com",
    username: "someusername123",
    active: true,
    sign_in_count: 1,
  };
}
```

这个代码会导致报错.

```bash
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected named lifetime parameter // 需要一个生命周期
  |
help: consider introducing a named lifetime parameter // 考虑像下面的代码这样引入一个生命周期
  |
1 ~ struct User<'a> {
2 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:3:12
  |
3 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     username: &str,
3 ~     email: &'a str,
  |
```

## 使用`#[derive(Debug)]`打印结构体

在前面的代码中我们使用 `#[derive(Debug)]`对结构体进行了标记，这样才能使用 `println!("{:?",s};)` 的方式对其进行打印输出。 

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

首先可以观察到上面使用了 `{}` 而不是 `{:?}`， 运行后报错

```bash
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

这提示我们结构体上没有实现 `Display` 特征。

报错提示我们使用 `{:?}`.

```rs
println!("rect1 is {:?}", rect1);
```
但是还是报错
```bash
error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`

```

不过编译器让我们去实现`Debug`特征， 不过我们也可以直接
`add #[derive(Debug)] to Rectangle` 来让编译器帮我们实现Debug特征。

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("rectl is {:?}", rect1);
}
```
此时就能够正常打印了。
```rs
$ cargo run
rect1 is Rectangle { width: 30, height: 50 }
```

当结构体比较大的时候，我们可能希望能够有更好的输出表现，此时可以用`{:#?}`来代替`{:?}`，这样就会有更好的输出表现。

```bash
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

如果还是不满意的哈，可以实现Display特征，以向用户更美的展示结构体。

当然，还可以使用 `dbg!()` 宏来打印结构体，`dbg!()` 宏会打印出文件名和行号，方便调试。

它会拿走表达式的所有权，然后打印文件名、行号、debug信息，当然还有需要的表达式结果。最后会把表达式的所有权返回给调用者。

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };

  dbg!(&rect1);
}
```

输出如下
```rs
$ cargo run
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

# 枚举

枚举 enum通过列举可能的成员来定义一个枚举类型，例如扑克牌花色

```rs
enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}
```

**枚举值**只可能是其中的某一个成员，抽象来看四种花色尽管是不同的花色，但是他们都是扑克花色。

**枚举类型是一个类型，包含所有可能的枚举成员，而枚举值是该类型中的具体某个成员的实例**

## 枚举值

现在来创建两个实例

```rs
let heart =  PokerSuit::Hearts;
let diamond = PokerSuit::Diamonds;
```

我们通过:: 操作符来访问Pokersuit下的具体成员，从代码可以清晰看出heart和 diamond都是Pokersuit枚举类型的，接着可以定义一个函数来使用
```rs

fn main() {
  let heart = PokerSuit::Hearts;
  let diamond = PokerSuit::Diamonds;

  print_suit(heart);
  print_suit(diamond);
}

fn print_suit(suit: PokerSuit) {
  match suit {
    PokerSuit::Clubs => println!("Clubs"),
    PokerSuit::Spades => println!("Spades"),
    PokerSuit::Diamonds => println!("Diamonds"),
    PokerSuit::Hearts => println!("Hearts"),
  }
}
```

目前来说，枚举值还不能带有值，因此先用就结构体来实现

```rs

enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}

struct PokerCard {
  suit: PokerSuit,
  value: u8,
}

fn main() {
  let c1 = PokerCard {
    suit: PokerSuit::Hearts,
    value: 1,
  };

  let c2 = PokerCard {
    suit: PokerSuit::Diamonds,
    value: 13,
  };
}

```


不过还可以这样
```rs
enum PokerCard {
  Clubs(u8),
  Spades(u8),
  Diamonds(u8),
  Hearts(u8),
}

fn main() {
  let c1 = PokerCard::Hearts(1);
  let c2 = PokerCard::Diamonds(13);
}

```

直接将数据信息关联到枚举成员上，省去将近一半的代码。

不仅如此，同一个枚举类型下的不同成员还能持有不同的数据类型。

```rs

enum PokerCard{
  Clubs(u8),
  Spades(u8),
  Diamonds(char),
  Hearts(char),
}

fn main(){
  let c1 = PokerCard::Spades(5);
  let c2 = PokerCard::Diamonds('A');
}
```


## 同一化类型

例如我们有一个WEB服务，需要接受用户的长连接，假设连接有两种，TcpStream和TlsStream，但是我们希望对这两个链接的处理流程相同，也就是用同一个函数来处理这两个链接

```rs

fn new (stream: TcpStream)  {
  let mut s = stream;
  if tls {
    s = negotiate_tls(stream);
  }
}

  websocket = WebSocket::from_raw_socket(
    s, ......)
```

## Option 枚举用于处理空值
在其他的编程语言当中，往往都有一个null关键字，用于表面一个变量当前的值为空。

当你对这些null进行操作的时候，例如调用一个方法，就会直接抛出null异常，导致程序的崩溃。

尽管如此，空值的表达依然非常有意义，因为空值表示当前时刻的变量的值是缺失的，有鉴于此，Rust吸取了很多教训，改为使用Option枚举来处理空值。

```rs

enum Option<T> {
  Some(T),
  None,
}
```

其中T是泛型参数，Some(T)表示该枚举成员的数据类型是T

```rs
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

使用None的时候需要告诉rust OptionT的类型，应为编译器只通过None值无法推断出some成员保存的值的类型。

当有一个Some值的时候，我们就知道存在一个值，而这个值保存在Some中。而使用None的时候，其实和空值的意义是相同的。

换句话说，其实就是因为 Option< T> 和 T的类型实际上是不一样的，在进行T的运算的时候必须把它转换为T类型。

这样就避免了我们错误的使用一个空值。

match语法就是用来进行匹配的。

```rs

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```


# 数组

在日常开发中最常使用的数据结构之一就是数组。

在Rust中，常用的数组有两种，第一种是速度很快但是长度固定的array, 第二种是动态可增长但是有性能损耗的vector。

在这里，我们先重点看 Array , 将多个类型的相同的元素依次组合在一起
- 长度固定
- 元素必须有相同的类型
- 依次线性排列

数组是Rust的基本类型，是固定长度的，和其他的语言不同，其他编程语言的数组一般是可变长度的。

## 创建数组

rust中，数组是这样定义的

```rs

fn main() {
  let a = [1, 2, 3, 4, 5];
}

```

array存储在栈上面，性能很优秀

例如要写一年中各个月份的数组
```rs

let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
```

有些时候，还需要声明类型
```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

还可以用下面的语法初始化一个值重复出现n次的数组
```rs
let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
```


## 访问数组元素

因为数组是连续存放元素的，所以可以通过索引来访问

```rs
fn main() {
  let a = [9,8,7,6,5];

  let first = a[0];
  let second = a[1];
}
```

**越界访问**

```rs

use std::io;

fn main() {
  let a = [ 1, 2, 3, 4, 5];
  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");
  
  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");
  
  let element = a[index];

  println!("The value of the element at index {} is: {}", index, element);
}

```


当你尝试使用索引访问元素时，Rust 将检查你指定的索引是否小于数组长度。如果索引大于或等于数组长度，Rust 会出现 panic。这种检查只能在运行时进行，比如在上面这种情况下，编译器无法在编译期知道用户运行代码时将输入什么值。



### 数组元素为非基础类型

```rs
let array = [String::from("Hello"),8];
```

会直接爆炸

因为let array [ 3;5] 底层是一个个copy出来的，而复杂类型的类型是没有实现Copy特征的，因此无法直接复制。


而如果要手搓，太难看
```rs
let array = [String::from("rust is good!"),String::from("rust is good!"),String::from("rust is good!")];

println!("{:#?}", array);
```

正确的写法应该用
```rs
let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

println!("{:#?}", array);
```

## 数组切片

在之前的章节有讲到过切片


```rs
let a : [i32; 5] = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

slice的类型是`&[T]`，其中T是数组的元素类型。

- 切片的长度可以不同
- 创建切片的代价很小
- 切片类型拥有不固定的大小，而切片引用类型则具有固定的大小


```rs
fn main() {
  // 编译器自动推导出one的类型
  let one             = [1, 2, 3];
  // 显式类型标注
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  // 借用arrays的元素用作循环中
  for a in &arrays {
    print!("{:?}: ", a);
    // 将a变成一个迭代器，用于循环
    // 你也可以直接用for n in a {}来进行循环
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
  }
}
```

- 数组类型容易跟数组切片混淆，[ T;n] 描述了一个数组的类型，而 [ T] 描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用 [ T;n] 的形式去描述
- [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
- 在实际开发中，使用最多的是数组切片[ T]，我们往往通过引用的方式去使用&[ T]，因为后者有固定的类型大小

# 流程控制

## if分支控制

```rs

if condition == true {
  //A
} else {
  //B
}
```

```rs
fn main() {
  let condition = true;
  let number = if condition {
    5
  } else {
    6
  };

  println!("The value of number is: {}", number);
}
```

- **if语句块是表达式**, 这里我们使用if表达式返回值给number进行赋值； number的值是5

- 用if来赋值的时候，要保证每个分支返回的类型一样，此处返回的都是整数类型，因此number的类型是i32

## else if

```rs

fn main() {
  let n = 6;

  if n %4 == 0 {
    println!("number is divisible by 4");
  } else if n %3 == 0 {
    println!("number is divisible by 3");
  } else if n %2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
}

```

程序会按照自上至下的顺序执行每一个分支判断，一旦成功，就会跳出if语句块。最终本程序会匹配执行 `else if n % 3 == 0` 的分支，输出 `"number is divisible by 3"`

就算有多个分支是匹配的，也只会有第一个匹配的分支会被执行。

# 循环控制

## for

```rs
fn main(){
  for i in 1..=5 {
    println!("i = {}", i);
  }
}
```

```rs
for 元素 in 集合 {
  //循环体
}
```
使用for的时候我们常常使用集合的引用形式，除非这个地方的所有权我们不要了。

如果不用引用，所有权会被转移到for语句块中，后面就无法再用了。

如果想在循环中修改元素，可以用mut
```rs

for item in &mut collection {
  //循环体
}
```

so: 总结如下
|使用方法 | 等价使用方式 | 所有权 |
|---|---|---|
|for item in collection | for item in IntoIterator::into_iter(collection) | 转移所有权 |
|for item in &collection | for item in collection.iter() | 不可变借用|
|for item in &mut collection | for item in collection.iter_mut() | 可变借用 |


如果想在循环中获得元素的索引:

```rs

fn main(){
  let a = [4,3,2,1];

  for (i,v) in a.iter().enumerate() {
    println!("index = {}, value = {}", i, v);
  }
}
```

如果想用for循环控制10次，但是又不想使用循环变量，可以用下划线 `_` 来代替循环变量

```rs

for _ in 0..10 {
  println!("Hello, world!");
}
```


**循环方式的对比**

```rs
//第一种
let collection = [1,2,3,4,5];
for i in 0..collection.len() {
  println!("i = {}", collection[i]);
}

for item in collection {
  ...
}

```

第一种是循环索引，然后通过索引访问元素，第二种是直接遍历集合的元素。

- 性能: 第一种会因为边界检查导致运行时候有性能损耗 —— Rust 会检查并确认 `index` 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查。

- 安全: 第一种方式的索引访问是非连续的，存在一定可能性在两种访问中collection被修改，导致索引越界访问，第二种方式是连续的访问，安全性更高。

`continue`

```rs

for i in 1..4 {
  if i == 2 {
    continue;
  }
  println!("i = {}", i);
}
```


`break`

```rs

for i in 1..4 {
  if i == 2 {
    break;
  }
  println!("i = {}", i);
}
```


## while loop

```rs

fn main() {
  let mut n = 0;
  while n <= 5 {
    println!("n = {}", n);
    n += 1;
  }

  println!("done!");
}
```

也可以
```rs
fn main() {
  let mut n = 0;
  loop {
    if n > 5 {
      break;
    }
    println!("n = {}", n);
    n += 1;
  }

  println!("done!");
}

```

从上可知，for比while更安全，因为for循环是通过迭代器来实现的，而while循环是通过索引来实现的，索引可能会越界。

loop循环是无限循环，除非在循环体中使用break语句来跳出循环。

# 模式匹配

## match 和 if let
在rust中，最常用的模式匹配就是 `match` 和 `if let`

```rs

enum Direction {
  East,
  West,
  South,
  North,
}

fn  main() {
  let dire = Diection::South;

  match dire {
    Diection::East => println!("East"),
    Dirction::North | Diection::South => println!("North or South"),
    Diection::West => println!("West"),
  };
}

```

- match匹配要求穷尽性，也就是所有的可能性都要匹配到，否则编译器会报错。

- match的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同。

- X | Y 表示匹配X或者Y

### match 匹配

```rs
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```

将模式与 `target` 进行匹配，即为模式匹配，而模式匹配不仅仅局限于match

match允许我们将一个值和一系列的模式相比较，并根据相匹配的模式执行对应的代码

```rs
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quater,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quater => 25,
  }
}
```

match 后紧跟着的是一个表达式，跟 if 很像，但是 if 后的表达式必须是一个布尔值，而 match 后的表达式返回值可以是任意类型，只要能跟后面的分支中的模式匹配起来即可，这里的 coin 是枚举 Coin 类型。

match的分支有两个部分，一个模式和针对该模式的处理代码。

第一个分支的模式是 `Coin::Penny`，其后面的 => 运算符将模式和将要运行的代码分开。 这里的代码就仅仅是表达式1，不同分支之间使用逗号分隔。

每个分支相关联的代码是一个表达式，而表达式的结果只将作为整个match表达式的返回值，如果分支有多行代码，那么需要用 {} 包裹。

### 使用`match` 表达式赋值
```rs

enum IpAddr{
  Ipv4,
  Ipv6,
}

fn main() {
  let ip1 = IpAddr::Ipv6;
  let ip_str = match ip1 {
    IpAddr::Ipv4 => "127.0.0.1",
    _ => "::1",
  };

  println!("ip_str = {}", ip_str);
}
```

### 模式绑定

模式匹配的另外一个作用是从模式中取出来绑定的值

```rs
#[derive(Debug)]
enum UsState{
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

```

其中，Coin::Quarter这个成员还放了一个值。接下来，我们希望在模式匹配中，获取到这个值，并打印出来。

```rs
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}
```

在上面的代码中，在匹配到Coin::Quarter这个成员的时候，我们将其内部的值绑定到了state上面，这样就可以在分支中使用state了。

再来看一个更复杂的例子.

```rs

enum Action {
  Say(String),
  Move { x: i32, y: i32 },
  ChangeColorRGB(i32, i32, i32),
}

fn main() {
  let actions = [
    Action::Say("Hello Rust!".to_string()),
    Action::Move { x: 10, y: 20 },
    Action::ChangeColorRGB(255, 0, 0)
  ];

  for action in actions {
    match action {
      Action::Say(s) => {
        println!("Say: {}", s);
      },
      Action::Move { x, y } => {
        println!("Move to ({}, {})", x, y);
      },
      Action::ChangeColorRGB(r, g, _) => {
        println!("Change color to RGB({}, {}, _)", r, g);
      }
    }
  }
}


```

### 穷尽匹配

```rs
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
    };
}
```
在上面，我们没有处理dire = Direction::West的情况，因此编译器会报错，提示我们没有穷尽匹配。

### _ 通配符

```rs
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

我们只关心1,3,5,7的情况，其他的情况我们都不关心，因此使用了通配符 `_` 来匹配其他的情况。

```rs
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}
```

然而在某些情况下，我们其实只关心 某一个值是否存在，此时match就显得有些繁琐了，Rust提供了`if let`语法来简化这种情况。

## `if let` 匹配

有些时候会遇到只有一个模式的值需要被处理，其他值直接忽略，那么match就要写成这样

```rs
let v = Some(3u8);
match v {
  Some(3) => println!("three"),
  _ => (),
}
```

如果只想处理Some(3)的情况，那么可以使用if let来简化

```rs
if let Some(3) = v {
  println!("three");
}
```

## matches! 宏

Rust标准库中的一个非常实用的marco: `matches!`， 可以将一个表达式和模式进行匹配，然后返回匹配的结果 true or false

例如有一个动态数组，里面存着
```rs
enum MyEnum {
  Foo,
  Bar,
}

fn main() {
  let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
}

```

现在如果想要对v进行过滤，只保留Foo

```rs
v.iter().filter( |x| x == MyEnum::Foo);
```
然而，这个地方会报错，因为我们无法将x和一个枚举成员进行比较，因为x是一个引用类型，而MyEnum::Foo是一个枚举成员。

```rs
v.iter().filter(|x| matches!(x, MyEnum::Foo));
```

再看俩例子
```rs
let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));
```
在上面，我们使用了 `matches!` 来匹配一个字符是否是字母，第二个例子中，我们使用了 `if` 来进行条件判断，看看 `bar` 是否是一个Some值，并且其内部的值是否大于2。

## 变量遮蔽

无论是match 还是 if let, 这里都是一个全新的代码块，而且这里的绑定相当于新变量，如果使用同名变量，会发生变量遮蔽。

```rs
fn main(){
  let age = Some(30);
  println!("在匹配前，age = {:?}", age);
  if let Some(age) = age {
    println!("在匹配中，age = {:?}", age);
  }

  println!("在匹配后，age = {:?}", age);
}
```


输出是这样的
```bash
在匹配前，age = Some(30)
在匹配中，age = 30
在匹配后，age = Some(30)
```

在if let中， =  右边的age是外部的age，而左边的 i 32age是一个新的变量，发生了变量遮蔽。

对于match也是

```rs
fn main() {
  let age = Some(30);
  println!("在匹配前，age是{:?}", age);
  match age{
    Some(age) => println!("在匹配中，age是{:?}", age),
    _ => (),
  }
  println!("在匹配后，age是{:?}", age);
}
```

需要注意的是，match 中的变量遮蔽其实不是那么的容易看出，因此要小心.


# 方法 Methods

在面向对象语言中，方法不是一个陌生的东西，class里面就充斥着方法的概念，在Rust中，方法的概念也大差不差

```rs
object.method();
```

## 定义方法
rust采用 impl来定义方法，例如以下代码

```rs

struct Circle {
  x:f64,
  y:f64,
  radius: f64,
}

impl Circle {
  //new 是 Circle的一个关联函数，因为她的第一个参数不是self,且new不是关键字
  // 这种方法用于初始化当前结构体的实例
  fn new(x:f64, y:f64, radius: f64) -> Circle {
    Circle { x, y, radius }
  }


  //circle的方法 &self表示借用当前的Circle结构体
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }

}
```

其他语言的定义都在class中，而rust的对象定义和方法定义是分离的。这种数据和使用分离的方式，给予使用者极高的灵活度。

再来看一个例子

```rs

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}


fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```

该例子定义了一个Rectangle结构体，并定义了一个area方法。用于计算面积。


impl Rectanlge {} 表示为 Rectangle结构体实现方法，impl是implementation的缩写。

**self, &self 和 &mut self**

在area的签名里面，我们使用 &self替代 rectangle: &Rectangle.

&self 其实是 self:&Self的简写。 在一个impl块内，Self指代被实现方法的结构体类型。self指代此类型的实例。换句话说，self指代的是Rectangle结构体实例。

self依然有所有权的概念。

- `self` 表示Rectangle 的所有权转移到该方法中，用的很少
- `&self` 表示该方法对Rectangle的不可变借用
- `&mut self` 表示该方法对Rectangle的可变借用

简单总结来说，使用方法代替函数有以下的好处
- 不用再函数签名中重复书写self对应的类型
- 代码的组织性和内聚性更强，对于代码维护和阅读来说，好处巨大。

**方法名和结构体字段名相同**

```rs

impl Rectangle {
  fn width(&self) -> bool {
    self.width > 0
  }
}

fn main() {
  let rect1 = Rectangle{
    width: 30,
    height: 50,
  };

  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
  }
}

```

当我们使用
`rect1.width()` 调用方法,`rect1.width`访问字段。

一般来说，方法和字段同名。往往适用于实现getter访问器. 例如

```rs

mod my {
  pub struct Rectangle {
    width: u32,
    pub height: u32,
  }

  impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
      Rectangle { width, height }
    }

    pub fn width(&self) -> u32 {
      return self.width;
    }

    pub fn height(&self) -> u32 {
      return self.height;
    }
  }
}

fn main(){
  let rect1 = my::Rectangle::new(30, 50);

  println!("The rectangle has a nonzero width; it is {}", rect1.width());
  println!("The rectangle has a nonzero height; it is {}", rect1.height());

  println!("The rectangle has a nonzero height; it is {}", rect1.width);// 这里会报错，因为width是私有的，无法访问

  println!("The rectangle has a nonzero height; it is {}", rect1.height);// 这里不会报错，因为height是公有的，能够访问
}

```

在c/cpp中，我们可以用 . or -> 在对象上面调用方法，而 -> 在一个对象的指针上调用方法。换句话说， if object 是一个 ptr， 那么 object -> something() 和 (*object).something() 是等价的。


Rust并没有 -> 这个运算符，相反，rust有一个自动引用和解引用的功能。方法调用是Rust中少数几个拥有这种行为的地方。

当使用object.something()调用方法的时候，Rust会自动为Object添加 & (视可见性添加&mut) 、 * 以便使 object与方法签名匹配。也就是说，这些代码是等价的。

```rs
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
#
# impl Point {
#    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
#
#        f64::sqrt(x_squared + y_squared)
#    }
# }
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
p1.distance(&p2);
(&p1).distance(&p2);
```


## 带有多个参数的方法

方法和函数一样，可以使用多个参数

```rs

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: & Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };

  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

```


## 关联函数

如何为一个结构体定义一个构造器方法？也就是接受几个参数，然后构造并返回该结构体的实例。

其实答案在开头的代码就给出了，参数不包含self的函数就是关联函数，通常用于构造器。

```rs
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }
}
```

因为是函数，所以不能通过 . 来调用，而是通过 :: 来调用。



## 多个impl定义

Rust允许我们为一个结构体定义多个impl块，目的是提供更多的灵活性。例如当方法多了之后，可以把相关的方法放在同一个 impl块中，那么就可以形成多个impl块。各自完成一块目标

```rs
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## 为枚举实现方法

```rs
#![allow(unused)]

enum Message{
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    // 方法体
  }
}

fn main() {
  let m = Message::Write(String::from("hello"));
  m.call();
}

```


# 泛型和特征

# 泛型 Generics

为什么泛型重要，什么是泛型？rust的泛型有几种？

我们在编程中，经常有这样的一个需求，用同一功能的函数处理不同类型的数据，无论是整数还是浮点数甚至是自定义类型。

在不支持泛型的语言里面，我们需要

```rs

fn add_i8(a: i8, b: i8) -> i8 {
  a + b
}

fn add_i16(a: i16, b: i16) -> i16 {
  a + b
}
fn add_i32(a: i32, b: i32) -> i32 {
  a + b
}
...

```

上面的这个代码太罗嗦了。实质上，泛型就是一种多态。泛型主要目的是为程序员提供编程的便利，减少代码的臃肿，同时可以极大丰富语言本身的能力.

```rs

fn add<T>(a: T, b: T) -> T {
  a + b
}
```

## 泛型详解

使用泛型参数有一个先决条件就是必须在使用前对其进行声明:
```rs
fn largest<T>(list: &[T]) -> &T
```

列表中的元素类型为T，largest<T> 对泛型参数T进行声明，然后才从函数里面进行使用该泛型参数 list: &[T]

总之，可以这样理解函数定义: 函数largest 有泛型类型T，它有个参数list，其元素是类型为T的数组切片 最后该函数的返回值也是T

下面是一个错误的泛型函数
```rs

fn largest<T>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for &item in list.iter() {
    if item > *largest {
      largest = &item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);
}

```

但是实际上呢运行的时候会报错。因为我们不是任何的类型都能够进行比较。因此上面的错误中编译器建议我们给T增加一个类型限制，使用
`std::cmp::PartialOrd`特征对`T`进行限制。

事实上，上面写的add函数也需要我们给他增加一个类型限制，使用 `std::ops::Add<Output = T>`特征对`T`进行限制.

```rs
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
  a + b
}
```
## 显示地指定泛型的类型参数
有时候编译器无法推断你想要的泛型参数

```rs
use std::fmt::Display;

fn creat_and_print<T>() where T:From<i32> + Display {
  let a : T =  100.into();
  println!("a = {}", a);
}

fn main(){
  creat_and_print();
}

```

运行就会报错，因为编译器不知道你想要的泛型类型是什么，因此需要显示地指定泛型的类型参数

```rs
fn main(){
  creat_and_print::<i32>();
  creat_and_print::<f64>();
} 
```

## 结构体内使用泛型

结构体中的字段类型也可以用泛型来定义，下面代码定义了一个坐标点`point`，它可以存放任何类型的坐标值

```rs
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };
}
```

x和y的类型是相同的，都是T类型。而且必须是相同的类型，不能是不同的类型。

如果希望X，Y既能是相同类型，也能是不同类型，那么可以使用两个泛型参数

```rs
struct Point<T, U> {
  x: T,
  y: U,
}
```

## 枚举中使用泛型

提到枚举类型，Option永远是第一个被想起来的。

```rs
enum Option<T> {
  Some(T),
  None,
}
```

Option<T>是一个拥有泛型T的枚举类型，第一个成员就是Some(T),存放了一个类型为T的值，得益于泛型的引入，我们可以在任何一个需要返回值的函数中使用Option<T> 枚举类型作为返回值

## 方法中使用泛型

上一章中，我们讲到什么是方法以及如何在结构体和枚举中定义方法。

```rs

struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point { x: 5, y: 10 };

  println!("p.x = {}", p.x());
}
```

使用泛型参数之前，需要提前声明 impl<T>, 然后才能在Point<T> 中使用，这样Point中尖括号的类型是泛型而不是一个具体的类型。

这种时候，Point<T> 就不再是一个泛型的声明，而是一个完整的结构体类型。

```rs
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

在这个例子中, `T,U` 是定义在结构体中的泛型参数，而V,W是专门定义在方法上面的泛型参数，并不冲突，一个是结构体泛型，一个是函数泛型。

### 为具体的泛型类型实现方法

对于 `Point<T>` 类型，你不仅能定义基于T的方法，还能针对特定的具体类型进行方法定义:

```rs

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

这段代码意味着 `Point<f32>` 类型会有一个方法 distance_from_origin， 而其他类型的 `Point<T>` 则没有这个方法。

## const泛型

在之前的泛型中，可以抽象为一句话，针对类型实现的泛型，所有的泛型都是为了抽象不同的类型，那么有没有针对值的泛型呢？

在数组那一节，有提到过很重要的一点，[i32;2] 和 [i32;3]是不同的数组类型

```rs

fn display_array(arr: [i32; 3]){
  println!("{:?}" , arr);
}

fn main() {
  let arr: [i32; 3] = [1, 2, 3];
  display_array(arr);
  let arr2: [i32; 4] = [1, 2, 3, 4];
  display_array(arr2); // 这里会报错，因为arr2的类型是[i32; 4]，而display_array的参数类型是[i32; 3]
}

````

运行就报错拉

所以我们得修改代码

```rs
fn display_array(arr: &[i32]){
  println!("{:?}" , arr);
}

fn main() {
  let arr: [i32; 3] = [1, 2, 3];
  display_array(&arr);
  let arr2: [i32; 4] = [1, 2, 3, 4];
  display_array(&arr2); // 这里不会报错，因为display_array的参数类型是&[i32]，而arr2的类型是[i32; 4]，可以转换为&[i32]
}
```

接着，将i32改为所有类型的数组

```rs
fn display_array<T: std::fmt::Debug>(arr: &[T]){
  println!("{:?}" , arr);
}

fn main() {
  let arr:[i32; 3] = [1, 2, 3];
  display_array(&arr);
  let arr2:[f64; 4] = [1.0, 2.0, 3.0, 4.0];
  display_array(&arr2);
}

```

不过得给T增加一个Debug特征的约束，因为我们要打印数组。

通过引用我们可以很轻松的解决任何类型数组的问题，但是某些场景下引用不适宜呢？

好在现在有了const泛型，const泛型是针对值的泛型，允许我们在编译期就知道数组的长度。

```rs
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]){
  println!("{:?}" , arr);
}

fn main() {
  let arr:[i32; 3] = [1, 2, 3];
  display_array(arr);
  let arr2:[f64; 4] = [1.0, 2.0, 3.0, 4.0];
  display_array(arr2);
}
```

如上所示，我们定义了一个类型为 `[T; N]` 的数组，N是一个const泛型参数，表示数组的长度。这样我们就可以在编译期知道数组的长度，而不需要在运行时去计算。

N就是const泛型，定义的语法是 `const N: usize`

### const泛型表达式

假设我们某段代码需要在内存很小的平台上工作，因此需要限制函数参数占用的内存大小，此时就可以使用 const 泛型表达式来实现：

```rs
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn something<T>(val:T)
where
  Assert<{core::mem::size_of::<T>() < 786432}>: IsTrue,
  {
    // do something
  }

fn main() {
  something([0u8;0]); //ok
  something([0u8;512]); //ok
  something([0u8;786432]); //error
  
}
```

**const fn**
常量函数。

const fn 允许我们在编译期对函数进行求值，从而实现更高效更灵活的代码设计

**为什么需要 const fn？**

通常情况下函数是在运行的时候被调用和执行的，然而在某些场景下我们希望在编译期就计算出一些值，以提高运行时候的性能or满足某些编译期的约束条件。

有了const fn， 我们可以在编译期执行这些函数，从而将计算结果直接嵌入到生成的代码里，提高性能。


const fn 的基本用法:
```rs
const fn add(a: usize, b: usize) -> usize{
  a + b
}

const RESULT: usize = add(5,10);

fn main() {
  println!("RESULT = {}", RESULT);
}
```

const fn的限制:
const fn的结果是在编译期就能确定的，因此const fn的函数体中不能包含任何运行时才能确定的值。例如不可以将随机数生成器写成const函数。

**结合const fn 和 const泛型**

将 const fn 和 const 泛型结合可以实现更加灵活和高效的代码设计。例如创建一个固定大小的缓冲区结构，其中缓冲区大小由编译器计算确定

```rs

struct Buffer<const N: usize> {
  data: [u8; N],
}

const fn compute_buffer_size() -> usize {
  factor * 1024
}

const factor: usize = 4;

fn main() {
  const SIZE: usize = compute_buffer_size();
  let buffer = Buffer::<SIZE> { data: [0; SIZE] };

  println!("Buffer size: {}", buffer.data.len());
}
```

在这个例子中，compute_buffer_size是一个常量函数，它根据传入的factor计算缓冲区的大小。在 main 函数中，我们使用compute_buffer_size 计算缓冲区的大小为4096字节，并将其作为泛型参数传递给buffer 结构体。这样，缓冲区的大小在编译器就被确定下来，避免了运行时的计算开销.


## 泛型的性能。

rust中泛型是 0成本的抽象。使用的时候完全不需要担心性能问题。

然而代价嘛就是损失了编译速度和增加了二进制文件的大小。

具体来说，rust在编译的时候通过泛型代码的单态化(monmorphization)来生成针对每个具体类型的代码，这样在运行时就不需要进行类型检查和动态分发，从而提高了性能。

编译器将寻找所有泛型代码被调用的位置并对具体类型生成代码。

让我们看看一个使用标准库中的Option枚举的例子
```rs
let integer = Some(5);
let fload = Some(5.0);
```

当rust编译这些代码，就会进行单态化。


# Trait 特征

如果我们想要定义一个文件系统，那么把这个系统和底层存储解耦是很重要的。文件操作主要包括四个 `open` \ `write` \ `read` \ `close`

但是呢，这些操作可能发生在硬盘、IO、内存...

如果你要为每一种情况都单独实现一套代码，那这种实现将过于繁杂，而且也没那个必要。

通过Trait，我们可以定义一套通用的接口，让不同的底层存储实现这些接口，从而达到解耦的目的。

在之前的代码中，我们也多次见过特征的使用，例如 #[derive(Debug)]，它在我们定义的类型(struct)上自动派生 Debug 特征，接着可以使用 println!("{:?}", x) 打印这个类型；再例如：

```rs
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
  a + b
}
```

通过 `T: std::ops::Add<Output = T>`，我们约束了泛型类型T必须实现Add特征，才能使用 + 运算符。

特征定义了一组可以被共享的行为。

## 定义特征

定义特征是把一些方法组合在一起，目的是定义一个实现某些目标所必需的行为的集合。

例如我们现在有文章 `post` 和 微博 `weibo` 两种内容载体，而我们想要对相应的内容总结。无论是文章内容还是微博内容都可以在某个时间点被总结，那么总结这个行为就是共享的。

```rs
pub trait Summary {
  fn summarize(&self) -> String;
}
```

这里使用`trait`关键字声明一个特征，`Summary`是名字。在打括号里面定义了该特征的所有方法。

特征只定义行为是什么样子的，而不定义具体实现。所以，我们只定义特征方法的签名，而不进行实现。此时方法签名结尾是 `;`,而不是 `{}`。

接下来，每一个实现这个特征的类型都必须提供该方法的具体实现。


## 为类型实现特征

首先来为Post和Weibo实现`Summary` 特征

```rs

pub trait Summary {
  fn summarize(&self) -> String;
}


pub struct Post {
  pub title: String,
  pub content: String,
  pub author: String,
}

impl Summary for Post{
  fn summarize(&self) -> String {
    format!("{} by {}", self.title, self.author)
  }
}

pub struct Weibo {
  pub username: String,
  pub content: String,
}

impl Summary for Weibo {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

```

接下来就可以在这个类型上面调用特征的方法

```rs
fn main() {
  let post = Post{
    title: String::from("Rust is awesome!"),
    content: String::from("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."),
  }

  let weibo = Weibo{
    username: String::from("rustacean"),
    content: String::from("Rust is awesome!"),
  }

  println!("1 new post: {}", post.summarize());
  println!("1 new weibo: {}", weibo.summarize());
}
```

### 特征定义和实现的位置

上述的summary 是 pub的，这意味着它可以在其他模块中被访问和使用。

关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！

例如我们可以为上面的Post类型实现Display特征，这是因为Post类型定义在当前的作用域中。

但是我们没有办法在当前作用域中为String类型实现Display特征，因为String类型和Display特征都不在当前作用域中定义。

### 默认实现

你可以在特征中为方法提供一个默认的实现，这样实现该特征的类型就不需要再提供该方法的具体实现了。

```rs
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}
```
上面为Summary 定义了一个默认实现

```rs
impl Summary for Post {}

impl Summary for Weibo {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

Post类型使用了Summary的默认实现，而Weibo类型提供了自己的实现。

默认实现允许调用相同特征中的其他方法，哪怕没有默认实现。例如，特征可以提供很多功能而只需要实现指定的一个部分内容。

例如我们可以定义Summary特征，使其具有一个需要实现的Summarize_author方法。然后调用该方法来实现Summarize方法的默认实现。

```rs
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}
```

为了使用Summary,只需要实现Summarize_author方法即可。

```rs
impl Summary for Weibo {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

println!("1 new weibo: {}", weibo.summarize());
```


## 使用特征作为函数参数

之前提到过，特征如果仅仅是用来实现方法，那真的有些大材小用

不过，现在先定义一个函数，使用特征作为函数参数。

```rs
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

impl Summary 语法表示参数 item 可以是任何实现了 Summary 特征的类型。

你可以使用任何Summary特征的类型作为该函数的参数，同时在函数体内，还可以调用该特征的方法，例如summarize方法。具体地说可以传递Post或者Weibo的实例来作为参数。

## 特征约束

虽然impl Trait 很好理解但是其实只是语法糖，背后是特征约束的概念。

```rs
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}
```
T:Summary 被称为特征约束。

简单的场景下，impl Trait 这种语法糖就够用了。但是对于复杂的场景，特征约束可以让我们拥有更多的灵活性。

```rs
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
  println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
}
```

如果函数是不同的类型，那很好，但是如果我们希望item1和item2是相同的类型呢？那么就需要使用特征约束

```rs
pub fn notify<T: Summary>(item1: &T, item2: &T) {
  println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
}

```

泛型类型T说明了 `item1` 和 `item2`必须拥有同样的类型，同时 `T:Summary` 说明了T必须实现Summary特征

### 多重约束

除了单个约束条件，我们还可以指定多个约束条件，例如除了让参数实现Summary特征之外，还可以让参数实现Display特征

```rs
pub fn notify(item: &(impl Summary + Display)) {}


pub fn notify<T: Summary + Display>(item: &T) {}
```

通过这两个特征约束，我们可以在函数体内调用Summary和Display特征的方法。


### where 语法

当特征约束很多

```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  // 函数体
}
```

我们还能做出一些改进
```rs
fn some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
  // 函数体
}
```

### 使用特征约束有条件的使用方法和特征

特征约束可以让我们在指定类型+特征的情况下，使用该类型的方法和特征。

```rs
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self{
    Self{
      x,
      y,
    }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
```

cmp_display 方法，并不是所有的 Pair<T> 结构体对象都可以拥有，只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法。 


也可以有条件的实现特征，例如标准库为任何实现了 Display 和 Debug 特征的类型实现了 ToString 特征。

```rs
impl<T: Display + Debug> ToString for T {
  fn to_string(&self) -> String {
    // 方法体
  }
}
```

## 函数返回中的 `impl Trait`

可以通过 `impl Trait` 来说明一个函数返回了一个类型,该类型实现了某个特征

```rs
fn returns_summarizable() -> impl Summary {
  Weibo {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
  }
}
```

因为weibo实现了Summary特征，所以可以作为返回值。

但是对于 returns_summarizable 的调用者而言，他只知道返回了一个实现了 Summary 特征的对象，但是并不知道返回了一个 Weibo 类型

这种 impl Trait 形式的返回值，在一种场景下非常非常有用，那就是返回的真实类型非常复杂，你不知道该怎么声明时（毕竟 Rust 要求你必须标出所有的类型），此时就可以用 impl Trait 的方式简单返回。

但是这种返回值方式有一个很大的限制：只能有一个具体的类型，例如：


```rs
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Post {
            title: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Weibo {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
        }
    }
}
```

以上的代码就无法通过编译，因为它返回了两个不同的类型 Post 和 Weibo。


## 通过Derive 派生特征

在本书中形如 `#[derive(Debug)]` 的语法是一个属性（attribute），它允许我们为类型自动派生一些特征的实现。

例如Debug特征允许我们使用 `println!("{:?}", x)` 来打印类型的实例。

Copy特征也有一套自动派生的规则，允许我们使用 `#[derive(Copy, Clone)]` 来为类型自动派生Copy和Clone特征的实现。

derive派生的是Rust默认给我们提供的特征，极大的简化了我们为类型实现特征的工作量。

## 调用方法需要引入特征

在一些场景下，使用as 关键字会有很大的限制，因为想要在类型转换上拥有完全的控制,那么我们需要 TryInto
```rs

use std::convert::TryInto;

fn main() {
  let a:i32  = 10;
  let b:u16  = 100;

  let b_ = b.try_into().unwrap();

  if a < b_ {
    println!("a is less than b");
  }
}

```


# 几个综合例子

## 为自定义类型实现 + 

```rs
use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}
```

# 特征对象
在上一节重有一段代码无法通过编译
```rs
fn returns_summarizable(swith: bool) -> impl Summary {
  if switch {
    Post {
      //...
    }
  }else{
    Weibo {
      //...
    }
  }
}
```

其中Post和Weibo 都实现了Summary特征，因此上面的函数试图通过返回impl Summary 来返回这两个类型,但是编译器却无情的通过报错了。原因是impl Trait 形式的返回值只能有一个具体的类型，而Post和Weibo是两个不同的类型。

现在考虑一个问题，如果我们要做一个游戏，需要将多个对象渲染在屏幕上，这些对象渲染在屏幕上，属于不同的类型，存储在列表里面，渲染的时候需要循环该列表并顺序渲染每个对象。

我们可以考虑利用枚举
```rs
#[derive(Debug)]
enum UiObject {
  Button,
  SelectBox，
}

fn main() {
  let objects = [
    UiObject::Button,
    UiObject::SelectBox,
  ];

  for o in &objects {
    draw(o);
  }
}

fn draw(o : UiObject) {
  println!("draw {:?}", o);
}
```

但是问题是如果对象集合无法预先知道，就很麻烦.

总之，在编写UI库的时候，我们无法知道所有的UI对象类型，只知道是:
- UI对象类型不同
- 需要一个统一的类型处理这些对象
- 需要对每一个对象调用`draw`方法

在继承的语言中，可以定义一个名为Component的基类，在上面定义一个draw方法，然后让所有的UI对象继承这个基类，并实现draw方法。然后在渲染的时候，使用Component类型的列表来存储所有的UI对象，并调用draw方法。

## 特征对象定义

为了解决上面的问题，Rust引入了一个概念 —— 特征对象

在介绍特征对象之前，先来为之前的UI组件定义一个特征
```rs
pub trait Draw{
  fn draw(&self);
}
```
只要组件实现了 Draw，就可以调用draw方法来进行渲染，假设有一个 `button` 和 `selectBox` 实现了 Draw特征
```rs
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}

```

此时呢，还需要一个动态数组来存储这些ui对象
```rs
pub struct Screen {
  pub components: Vec<?>,
}
```
上面的?的意思是我们应该填入什么类型，可以说就之前学过的内容，你不知道应该填入什么类型，因为Button和SelectBox是两个不同的类型，无法存储在同一个Vec中。

那么我们能不能把Draw特征的对象作为类型，填入到Vec中呢？答案是可以的，这就是特征对象的作用。

特征对象指向实现了Draw特征的类型的实例，也就是指向Button or SelectBox的实例
```rs
trait Draw {
  fn draw(&self) -> String;
}

impl Draw for u8 {
  fn draw(&self) -> String {
    format!("u8: {}", self)
  }
}

impl Draw for f64 {
  fn draw(&self) -> String {
    format!("f64: {}", self)
  }
}

// 若T实现了Draw特征，那么Box<T> 可以被隐式的转换成函数参数签名中的Box<dyn Draw>类型
fn draw1(x: Box<dyn Draw>) {
  //由于实现了Deref特征，Box<dyn Draw>可以被解引用为dyn Draw类型，因此可以调用draw方法
  println!("{}", x.draw());
}

fn draw2(x: &dyn Draw) {
  println!("{}", x.draw());
}

fn main() {
  let x = 1.1f64;

  let y = 1u8;

  draw1(Box::new(x));
  draw1(Box::new(y));
  draw2(&x);
  draw2(&y);
}
```

上面的代码有几个地方比较重要:
- `draw1`函数的参数是 `Box<dyn Draw>`形式的特征对象，该特征对象是通过`Box::new(x)`的方式创建的
- `draw2`函数的参数是 `&dyn Draw`形式的特征对象，该特征对象是通过`&x`的方式创建的
- `dyn` 关键字只用在特征对象的类型声明上面，在创建的时候无需使用 `dyn`

然后，继续完善之前的UI组件，可以来先实现Screen
```rs
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}
```
其中存储了一个动态数组，里面的元素类型是Draw特征对象，`Box<dyn Draw>`, 任何实现了Draw特征的类型都可以被存储在这个数组中。

再来为Screen定义 run方法,用于将列表里面的UI组件渲染在屏幕上
```rs
impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
```

至此，我们就完成了之前的目标，在列表中存储了不同类型的UI组件，并且可以调用它们的draw方法进行渲染。

再来看看，如果我们通过泛型来实现的情况
```rs
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}

impl<T> Screen<T> {
  where T:Draw {
    pub fn run (&self) {
      for component in self.components.iter() {
        component.draw();
      }
    }
  }
}
```
上面的screen列表中，存储了类型为T的元素，然后在screen中使用特征约束让T实现了Draw特征，进而可以调用draw方法。

但这种方式有一个很大的限制，那就是Screen只能存储同一种类型的UI组件，无法存储不同类型的UI组件。

现在我们来尝试运行
```rs
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

上面通过`Box::new(T)`的方式创建了两个`Box<dyn Draw>`特征对象, 如果我们后面还需要别的组件，让这个组件实现Draw就可以了

使用特征对象和 Rust 类型系统来进行类似鸭子类型操作的优势是，无需在运行时检查一个值是否实现了特定方法或者担心在调用时因为值没有实现方法而产生错误。如果值没有实现特征对象所需的特征， 那么 Rust 根本就不会编译这些代码：

```rs
fn main() {
  let screen = Screen {
    components: vec![
      Box::new(String::from("Hi")),
    ],
  };

  screen.run(); // 编译器报错，因为String没有实现Draw特征
}
```

因为string 类型没有实现Debug特征，编译器就会报错，不会让上面的代码运行.如果想要`String` 类型被渲染在屏幕上，那么是需要为其实现`Draw`特征即可.

注意`dyn`不能单独作为特征对象的定义。 原因是特征对象可以是任意实现了某个特征的类型，编译器在编译期不知道类型的大小。但是 &dyn和Box<dyn>是可以的，因为它们是指针类型，大小是已知的。

## 特征对象的动态分发

泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的具体类型生成一份代码，这种方式是静态分发(static dispatch)，因为是在编译期完成的，对于运行期性能完全没有任何影响。

与静态分发相对的是动态分发(dynamic dispatch)，动态分发是在运行期完成的。特征对象就是动态分发的一个例子。

当使用特征对象的时候，Rust必须使用动态分发，因为编译器无法知晓所有可能用于特征对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，rust在运行的时候使用特征对象中的指针来知晓需要调用哪个方法。
动态分发也组织编译器有选择的内敛方法代码，这会相对应的禁用一些优化。

结合上文的内容可以了解
- 特征对象的大小不固定
- 几乎总是使用特征对象的引用方式

## Self 和 self
在rust 里面，有两个self， 一个指代当前的实例对象，一个指代特征或者方法的别名
```rs
trait Draw{
  fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;
impl Draw for Button {
  fn draw(&self) -> Self {
    self.clone()
  }
}

fn main() {
  let button = Button;
  let new_button = button.draw();
}

```

上述代码中，self指代的是当前的实例对象，也就是button.draw()中的button实例，Self指代的是当前的类型，也就是Button类型。

## 特征对象的限制

不是所有特征都有特征对象，只有对象安全的特征才行. 当一个特征的所有方法都有如下属性的时候，它的对象才是安全的:
- 方法的返回值不能是Self类型
- 方法的参数不能是泛型类型

因为一旦有了特征对象，就不再需要知道实现该特征的具体类型是什么了，如果特征方法返回了具体的Self类型，但是特征对象忘记了真正的类型，那么就无法知道返回值的类型是什么了。
但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么

标准库里面的Clone就不对象安全的，因为其返回了Self类型。

String 类型实现了 Clone 特征， String 实例上调用 clone 方法时会得到一个 String 实例。类似的，当调用 Vec<T> 实例的 clone 方法会得到一个 Vec<T> 实例。clone 的签名需要知道什么类型会代替 Self，因为这是它的返回值。

如果违反了对象安全的规则，编译器就会报错，提示该特征不对象安全。例如之前的screen 结构体存放实现了clone特征的类型，那么就无法使用特征对象来存放这些类型，因为Clone特征不对象安全。


# 深入了解特征

特征之于Rust更甚接口or别的语言，因此特征在Rust里面也相对复杂。

## 关联类型

方法一章中，我们讲到了关联函数，但是实际上关联类型和关联函数并没有任何交集，虽然他们名字有一半是一样的。

管理类型是在特征定义的语句块中，声明一个自定义类型，这样就可以在特征的方法签名中使用该类型

```rs
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
```

Iterator中有一个Item关联类型，用于替代遍历的值的类型

同时，next方法也返回了一个Item类型，不过使用Option枚举进行了包裹。
假如迭代器里面的值是i32类型，那么Item就是i32类型，next方法返回的就是Option<i32>类型。

Self用来指代当前的类型，Self::Item就是指代当前类型的Item关联类型。
```rs
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
     // --snip --
  }
}

fn main() {
  let c = Counter{}
  c.next()
}

```

我们为counter实现了Iterator特征，并且指定了Item关联类型为u32类型，那么next方法返回的就是Option<u32>类型。

那比如为什么我们不需要写泛型呢?
```rs
pub trait Iterator<Item> {
  fn next(&mut self) -> Option<Item>;
}
```

但是这样的可读性是很差的，当你使用泛型的时候，你需要在所有地方都写上泛型类型，而关联类型只需要在特征定义的地方写一次即可。

例如
```rs
pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
  type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
  fn is_null(&self) -> bool;
}
```

例如上面的代码，写address肯定就比写泛型类型要好很多，尤其是当你需要在多个地方使用这个类型的时候。


如果使用泛型：
```rs
trait Container<A,B> {
  fn contains(&self,a:A,b:B) -> bool;
}

fn difference<A,B,C>(container: &C) -> i32
where
  C: Container<A,B>,
{
  // --snip--
}
```
如果使用泛型，函数的头部也必须增加泛型声明，安东尼是如果我们用关联类型

```rs
trait Container {
  type A;
  type B;
  fn contains(&self,a:Self::A,b:Self::B) -> bool;
}

fn difference<C>(container: &C) -> i32
where
  C: Container,
{
  // --snip--
}
```

关联类型还可以被特征约束

```rs

trait Container {
  type A: Display;
  type B;
  fn contains(&self,a:Self::A,b:Self::B) -> bool;
}
```

## 默认泛型类型参数

使用泛型类型参数的时候，可以为其指定一个默认的具体类型,例如标准库的std::ops::Add特征
```rs
trait Add<RHS = Self> {
  type Output;

  fn add(self, rhs: RHS) -> Self::Output;
}
```

这里他给RHS指定了一个默认类型Self，这样在实现Add特征的时候，如果没有指定RHS类型，那么就会使用Self类型作为RHS类型。

我们举一个例子
```rs
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
  x:i32,
  y:i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

fn main() {
  let p1 = Point { x: 1, y: 0 };
  let p2 = Point { x: 2, y: 3 };
  assert_eq!(p1 + p2, Point { x: 3, y: 3 });
}

```

上面的代码为Point结构体提供了+的能力，就是运算符重载.

上面的例子中并没有为 Point 实现 Add<RHS> 特征，而是实现了 Add 特征（没有默认泛型类型参数），这意味着我们使用了 RHS 的默认类型，也就是 Self

接下来我们来创建一个不同的

```rs
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}
```

这里，是进行 Millimeters + Meters 两种数据类型的 + 操作，因此此时不能再使用默认的 RHS，否则就会变成 Millimeters + Millimeters 的形式。使用 Add<Meters> 可以将 RHS 指定为 Meters，那么 fn add(self, rhs: RHS) 自然而言的变成了 Millimeters 和 Meters 的相加。


## 调用同名的方法

```rs
trait Pilot {
  fn fly(&self);


trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

```

这里不仅两个特征都有一个同名的方法fly，而且Human类型本身也有一个同名的方法fly。

当调用Human类型的fly方法时，Rust会优先调用Human类型本身的方法，而不是特征的方法。

```rs
fn main() {
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();
}
```

运行后依次
```bash
This is your captain speaking.
Up!
*waving arms furiously*
```

因为 fly 方法的参数是 self，当显式调用时，编译器就可以根据调用的类型( self 的类型)决定具体调用哪个方法。

但是如果方法没有self参数呢，例如关联函数

```rs
trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

fn main() {
  println!("A baby dog is called a {}", Dog::baby_name());
}

```

`Dog::baby_name()`调用的是Dog类型本身的baby_name方法，而不是Animal特征的baby_name方法。

而且也无法使用 `Animal::baby_name()`来调用Animal特征的baby_name方法，因为编译器无法知道你想要调用的是哪个类型的Animal特征的baby_name方法。

此时，就需要**完全限定语法**

```rs
fn main() {
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

尖括号内的 `<Dog as Animal>` 告诉编译器我们想要调用的是 Dog 类型实现的 Animal 特征的 baby_name 方法。 通过as关键字，告诉编译器Animal就是Dog类型实现的特征。

最终会调用`impl Animal for Dog`中定义的baby_name方法，输出结果为
```bash
A baby dog is called a puppy
```

总的来看,完全限定语法提供了一种方式来明确指定要调用哪个方法,避免了歧义。

```rs
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

上面的定义中，第一个参数就是方法接收器(三种方式: self, &self, &mut self)，如果是关联函数则不需要传入接收器。

## 特征定义中的特征约束

优势我们会需要让某个特征A能够使用另外一个特征B的功能，这种情况下，不仅仅要为类型实现特征A，还需要为类型实现特征B。为了避免这种情况，我们可以在特征A的定义中添加一个特征约束，要求实现特征A的类型必须同时实现特征B。

```rs
use std::fmt::Display;

trait OutlinePrint: Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}
```

在这里面，我们看到了 OutlinePrint: Display，和我们之前看到过的特征约束很像。

只不过用在了特征定义中而不是函数参数中。
是的，在某种意义上来说，这和特征约束非常类似，都用来说明一个特征需要实现另一个特征，这里就是：如果你想要实现 OutlinePrint 特征，首先你需要实现 Display 特征。

## 在外部类型上实现外部特征

在特征章节中，有提到孤儿规则，简单来说，就是特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。

这里提供一个办法来绕过孤儿规则，那就是使用newtype 模式，简而言之：就是为一个元组结构体创建新类型。该元组结构体封装有一个字段，该字段就是希望实现特征的具体类型。

```rs
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```
其中
`struct Wrapper(Vec<String>)` 就是一个元组结构体，定义了一个新的类型Wrapper，里面封装了一个Vec<String>类型的字段。

既然 new type 有这么多好处，它有没有不好的地方呢？答案是肯定的。注意到我们怎么访问里面的数组吗？self.0.join(", ")，是的，很啰嗦，因为需要先从 Wrapper 中取出数组: self.0，然后才能执行 join 方法。

当然，解决办法还是有的，要不怎么说 Rust 是极其强大灵活的编程语言！Rust 提供了一个特征叫 Deref，实现该特征后，可以自动做一层类似类型转换的操作，可以将 Wrapper 变成 Vec<String> 来使用。这样就会像直接使用数组那样去使用 Wrapper，而无需为每一个操作都添加上 self.0。

