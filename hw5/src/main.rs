use std::time::Instant;

struct Session {
    user_id: u32,
    created_at: std::time::Instant,
}

impl Drop for Session {
    fn drop(&mut self) {
        let now = Instant::now();
        let lifetime_ms = now.duration_since(self.created_at).as_nanos();
        println!(
            "сессия {} закрыта. время жизни {} нс",
            self.user_id, lifetime_ms
        );
    }
}
fn main() {
    {
        let s = Session {
            user_id: 42,
            created_at: Instant::now(),
        };
        println!("работаем");
        {
            let s = Session {
                user_id: 52,
                created_at: Instant::now(),
            };
            println!("работаем");
        }
    }
    // <- Drop вызывается ЗДЕСЬ, как в C++ RAII
    println!("после блока"); // уже без сессии, без GC и без утечки
}
