use std::print;

// From Example
// use crate::{Shape::Rect, Termo::Idle};

// enum Shape {
//     Circle { radius: f64 },
//     Rect(f64, f64),
//     Point,
// }
// fn area(s: &Shape) -> f64 {
//     match s {
//         Shape::Circle { radius } => 3.14159 * radius * radius,
//         Shape::Rect(w, h) => w * h,
//         Shape::Point => 0.0,
//     } // компилятор требует рассмотреть ВСЕ варианты
// }

enum Termo {
    Idle,
    Heating { power_pct: u8 },
    Error { code: u16 },
}

fn describe(t: Termo) -> String {
    match t {
        Termo::Idle => "запущен".to_owned(),
        Termo::Heating { power_pct } => format!("температура:\t{}", power_pct),
        Termo::Error { code } => format!("Ошибка. Код:\t{}", code),
    }
}

fn main() {
    let t = Termo::Idle;
    print!("Termo state:\t{:?}\n", describe(t))
}
