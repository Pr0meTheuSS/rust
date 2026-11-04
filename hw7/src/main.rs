// fn push_all(dst: &mut Vec<i32>, src: &[i32]) {
//     for &v in src {
//         dst.push(v);
//     }
// }
// fn main() {
//     let mut v = vec![1, 2, 3];
//     let r1 = &v;
//     let r2 = &v;
//     println!("{r1:?} {r2:?}");
//     let w = &mut v;
//     w.push(4);
//     push_all(&mut v, &[5, 6]);
//     // сколько угодно читателей
//     // читатели закончились здесь (NLL)
//     // теперь один писатель
//     // повторный уникальный заёмprintln!("{v:?} {w:?}");
// }

// Реализовать функции max_in_place (получает &mut Vec<i32> , заменяет минимальный
// элемент на максимальный) и find_all (возвращает Vec индексов по предикату на
// &[i32] ); обе — с юнит-тестами.
fn max_in_place(vec: &mut Vec<i32>) {
    let Some(&max) = vec.iter().max() else {
        return;
    };
    let Some(&min) = vec.iter().min() else {
        return;
    };
    for v in vec.iter_mut() {
        if *v == max {
            *v = min;
        }
    }
}
fn find_all<T, F>(items: &[T], predicate: F) -> Vec<u32>
where
    F: Fn(&T) -> bool,
{
    let mut idxs: Vec<u32> = Vec::new();
    for (index, value) in items.iter().enumerate() {
        if predicate(value) {
            idxs.push(index as u32);
        }
    }

    return idxs;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_in_place_empty_vec() {
        let mut v = vec![];
        max_in_place(&mut v);
        let expected = vec![];
        assert_eq!(v, expected);
    }

    #[test]
    fn max_in_place_simple_vec() {
        let mut v = vec![1, 2, 3];
        max_in_place(&mut v);
        let expected = vec![1, 2, 1];
        assert_eq!(v, expected);
    }

    #[test]
    fn max_in_place_the_same_vec() {
        let mut v = vec![1, 1, 1];
        max_in_place(&mut v);
        let expected = vec![1, 1, 1];
        assert_eq!(v, expected);
    }

    #[test]
    fn find_all_empty_vec() {
        let v: Vec<i32> = vec![];
        let res = find_all(&v, |x| *x > 0);
        assert!(res.len() == 0);
    }
    #[test]
    fn find_all_even() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let res = find_all(&v, |x| (*x % 2) == 0);
        assert!(res.len() == 3);

        assert_eq!(res[0], 1);
        assert_eq!(res[1], 3);
        assert_eq!(res[2], 5);
    }
}
