#[test]

/*

/* Implement `call_me` to make it work */
fn call_me {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
*/

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);  // Викликає замикання
    call_me(function);  // Викликає функцію
}

// Реалізація функції `call_me`, яка приймає будь-яке замикання або функцію
fn call_me<F>(f: F)
where
    F: Fn(),  // Тип `F` має реалізовувати трейт `Fn` (замикання, що не приймає аргументів і не повертає значення)
{
    f();
}

fn function() {
    println!("I'm a function!");
}




/*
call_me<F>: Це універсальна функція, яка приймає аргумент типу F. Тип F повинен реалізовувати трейт Fn(),
що означає, що цей тип має бути функцією або замиканням, яке не приймає параметрів і не повертає значення.
У main ми викликаємо call_me, передаючи в нього або замикання, або функцію.
Оскільки обидва вони реалізують трейт Fn(), це працює коректно.
*/