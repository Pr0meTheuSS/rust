// example code
// #[derive(Clone, Debug)]
// struct Config {
//     name: String,
//     retries: u32,
// }
// fn consume(c: Config) {
//     println!("забрал {:?}", c.name);
// }
// fn main() {
//     let a = Config {
//         name: "prod".into(),
//         retries: 3,
//     };
//     let b = a.clone();
//     // явная глубокая копия
//     consume(a);
//     // move: a больше нельзя трогать
//     // println!("{a:?}");
//     // E0382: value borrowed after move
//     let mut s = String::from("hello");
//     let t = s.clone();
//     // копия кучи
//     s.push_str(" world");
//     // ок: t — независимая
//     let x = 5u32;
//     let y = x;
//     // Copy: обе живы
//     println!("{t} {y} {b:?}");
// }

// Cannot implement because String not copiable
// #[derive(Copy, Clone)]
// struct Config {
//     name: String,
// }

fn takes(s: String) {}

fn main() {
    let s = String::from("string");
}
