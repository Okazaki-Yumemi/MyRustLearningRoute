
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    fn color(&self) -> String {
        match self {
            TrafficLightColor::Red => String::from("red"),
            TrafficLightColor::Yellow => String::from("yellow"),
            TrafficLightColor::Green => String::from("green"),
        }
    }
}

fn main() {
    for (color, expected) in [
        (TrafficLightColor::Red, "red"),
        (TrafficLightColor::Yellow, "yellow"),
        (TrafficLightColor::Green, "green"),
    ] {
        assert_eq!(color.color(), expected);
        println!("{:?}", color);
    }
}