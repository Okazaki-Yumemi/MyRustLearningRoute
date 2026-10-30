fn main(){
    let twenty = 20;

    let twenty_one = 21;

    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;

    println!("{} + {} + {} = {} ",twenty,twenty_one,twenty_two,addition);

    // 对于较长的数字，可以用 _分割
    let one_million :i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:.2}", forty_twos[0]);
    
}