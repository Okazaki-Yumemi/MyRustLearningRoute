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