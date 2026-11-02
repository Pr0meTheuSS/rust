// enum Msg {
//     Temp(u16),
//     Alert { code: u8, critical: bool },
//     Ping,
// }
// fn handle(m: Msg) -> &'static str {
//     match m {
//         Msg::Temp(t) if t > 850 => "перегрев!",
//         // guard
//         Msg::Temp(_) => "норма",
//         Msg::Alert {
//             code: 0..=9,
//             critical: true,
//         } => "критично",
//         Msg::Alert { .. } => "предупреждение",
//         Msg::Ping => "pong",
//     }
// }
// fn poll_until(ok: bool) -> u32 {
//     let mut n = 0;
//     loop {
//         n += 1;
//         if n > 3 && ok {
//             break n;
//         }
//     }
// }

// fn main() {

// }

use std::println;

fn fizz_bazz(n: u32) -> String {
    let reminds = (n % 3, n % 5);
    match reminds  {
        (0, 0) => {"fizz_bazz".to_owned()},
        (0, _) => {"fizz".to_owned()},
        (_, 0) => {"bazz".to_owned()},
        (_, _) => {n.to_string()},

    }
}

fn main() {
    for i in 1..100 {
        println!("{}, {}", i, fizz_bazz(i));
    }
}
