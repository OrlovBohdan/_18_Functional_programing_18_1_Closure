#[test]

/*
/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: __>(mut f: F) {
    f("hello");
}
*/

fn main() {
    let mut s = String::new();

    // Замикання змінює `s`, тому його тип має бути `FnMut`.
    let update_string = |str| -> String {
        s.push_str(str);
        s.clone()  // Повертаємо копію зміненої строки
    };

    exec(update_string);
}

// Використовуємо трейт `FnMut`, оскільки замикання змінює стан.
fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}


/*
FnMut: Замикання захоплює змінну s за посиланням і змінює її (через push_str).
Тому потрібно використовувати FnMut, щоб дозволити змінювати змінну.
У функції exec параметр f визначено як замикання типу FnMut(&'a str) -> String,
оскільки замикання приймає посилання на рядок і повертає новий рядок.
*/