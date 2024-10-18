#[test]

/*
fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

/* Fill in the blank */
fn exec<'a, F: __>(mut f: F)  {
    f("hello")
}
*/

fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

/* Заповнення прогалини */
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
}


/*
Ми використовуємо трейт FnMut, оскільки замикання змінює стан зовнішньої змінної s.
Аргумент замикання має бути типу &'a str, оскільки push_str приймає посилання на рядок.
*/