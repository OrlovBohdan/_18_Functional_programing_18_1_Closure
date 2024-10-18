#[test]
/*
/* Make it work with least amount of changes*/
fn main() {
    let color = String::from("green");

    let print = move || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;

    println!("{}",color);
}
*/

fn main() {
    let color = String::from("green");

    // Remove `move` to capture `color` by reference
    let print = || println!("`color`: {}", color);

    print();
    print();

    // Now you can borrow `color` again since the closure only holds an immutable reference.
    let _reborrow = &color;

    println!("{}", color);
}


/*
Щоб вирішити проблему з мінімальними змінами, можна просто видалити ключове слово move з замикання.
У такому разі замикання захопить color за посиланням, і color можна буде використовувати після виклику замикання
*/