#[test]

/*
/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: __ {

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
*/

fn main() {
    let greeting = "hello";
    // Тип, який не копіюється.
    // `to_owned` створює дані з правом власності з позичених.
    let mut farewell = "goodbye".to_owned();

    // Захоплює 2 змінні: `greeting` за посиланням та
    // `farewell` за значенням.
    let diary = || {
        // `greeting` захоплено за посиланням: вимагає використання `Fn`.
        println!("I said {}.", greeting);

        // Мутація змушує захопити `farewell` за
        // змінним посиланням. Тепер вимагає `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Виклик `drop` вручну змушує захопити `farewell`
        // за значенням. Тепер вимагає `FnOnce`.
        mem::drop(farewell);
    };

    // Виклик функції, яка застосовує замикання.
    apply(diary);

    // `double` відповідає обмеженню трейту `apply_to_3`
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}



use std::mem;

// Функція, яка приймає замикання як аргумент і викликає його.
// <F> означає, що F — це параметр з узагальненим типом.
fn apply<F>(f: F)
where
// Замикання не приймає аргументів і нічого не повертає.
    F: FnOnce(),
{
    f();
}

// Функція, яка приймає замикання і повертає `i32`.
fn apply_to_3<F>(f: F) -> i32
where
// Замикання приймає `i32` і повертає `i32`.
    F: Fn(i32) -> i32,
{
    f(3)
}



/*

У функції apply замикання має тип FnOnce, оскільки воно захоплює змінну farewell за значенням і викликає drop, що робить замикання викликаємим лише один раз.
У функції apply_to_3 використовується трейт Fn, оскільки замикання працює без змін стану і може бути викликане кілька разів.

*/