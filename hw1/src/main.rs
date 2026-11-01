use std::{io, println};

fn c2f(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}

fn f2c(temp: f64) -> f64 {
    5.0 / 9.0 * (temp - 32.0)
}

fn main() -> io::Result<()> {
    println!("Введите исходную температуру в шкале Фаренгейта или Цельсия");

    let mut input = String::new();
    // Читаем строку из stdin и сохраняем её в input
    io::stdin().read_line(&mut input)?;
    println!("Вы ввели: {}", input.trim());

    let number: f64 = input
        .trim()
        .parse()
        .expect("Не удалось преобразовать строку в число");

    input.clear();

    println!("Введите режим работы конвертера: c2f, f2c");
    // Читаем строку из stdin и сохраняем её в input
    io::stdin().read_line(&mut input)?;
    println!("Вы ввели: {}", input.trim());

    if input.trim() == "c2f" {
        println!("C: {:?}, F: {:?}", number, c2f(number))
    } else if input.trim() == "f2c" {
        println!("C: {:?}, F: {:?}", f2c(number), number)
    } else {
        println!("Введен неизвестный режим работы конвертера или допущена ошибка в вводе")
    }

    Ok(())
}
