
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

