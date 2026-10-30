fn main(){
    // 00000010
    let a:u8 = 2;
    // 00000110
    let b:u8 = 3;


    //{:08b} 左高右低输出二进制 01，不足8则高位补 0
    println!("a value is           {:08b}",a);
    println!("b value is           {:08b}",b);
    println!("(a&b) value is       {:08b}",a&b);
    println!("(a|b) value is       {:08b}",a|b);
    println!("(a^b) value is       {:08b}",a^b);
    println!("(!b) value is       {:08b}",!b);
    println!("(a<<b) value is       {:08b}",a<<b);
    println!("(a>>b) value is       {:08b}",a>>b);
    
    let mut a = a;

    a <<= b;

    println!("(a<<b) value is {:08b}",a);
}