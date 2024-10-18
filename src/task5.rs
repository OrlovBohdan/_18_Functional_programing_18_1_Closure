#[test]

/*
/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
*/

//1


fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| { z == x.len() })
}
fn fn_once<F>(func: F)
where
    F: FnMut(usize) -> bool,  // Зміна на FnMut
{
    let mut func = func;  // Для використання FnMut замикання має бути mut
    println!("{}", func(3));
    println!("{}", func(4));
}

//2

/*fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| { z == x.len() })
}
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,  // Зміна на Fn
{
    println!("{}", func(3));
    println!("{}", func(4));
}*/


/*
//1
1. Використовувати трейт FnMut:
Замість FnOnce можна використовувати FnMut, оскільки замикання змінює свій внутрішній стан через захоплення змінної x.
Трейт FnMut дозволяє замиканням змінювати стан і бути викликаними більше одного разу.

//2
2. Використовувати трейт Fn:
Якщо замикання лише читає дані з оточення (як у твоєму випадку, воно лише перевіряє довжину вектора, але не змінює стан),
можна використовувати трейт Fn, який дозволяє викликати замикання будь-яку кількість разів, не змінюючи його стан
*/