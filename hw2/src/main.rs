// overflow managment example
// fn main() {
//     let small: i8 = 127;
//     // let boom = small + 1;
//     // debug: panic 'attempt to add with overflow'
//     println!("{}", small.wrapping_add(1)); // -128: осознанное заворачивание
//     println!("{}", small.saturating_add(1)); // 127: насыщение
//     let big = 300i32 as u8;
//     // усечение: 44 — молча!
//     let safe: Option<u8> = u8::try_from(300i32).ok(); // None: честная ошибка
//     println!("{big} {safe:?}");
// }

fn avg(values: &[u32]) -> Option<f64> {
    let mut avg: f64 = 0f64;
    let len = values.len();
    if len == 0 {
        return None;
    } else {
        for i in values {
            avg += *i as f64 / len as f64
        }
        return Some(avg);
    }
}

fn main() {
    let values = vec![];

    // for v in values.into_iter() {
    // println!("{:?}", v);

    // }

    println!("avg {:?}", avg(&values));
}
