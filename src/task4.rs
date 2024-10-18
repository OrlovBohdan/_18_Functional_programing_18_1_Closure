#[test]

/*
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    /* Make it work, only change the following line */
    let n = example_closure(5);
}
*/

//Тут кожне замикання працює з конкретним типом, і це дозволяє уникнути помилок типізації.
fn main() {
    let example_closure_string = |x: String| x;
    let example_closure_int = |x: i32| x;

    let _s = example_closure_string(String::from("hello"));
    let _n = example_closure_int(5);
}


/*
Проблема в коді полягає в тому, що замикання за замовчуванням має конкретний тип для своїх параметрів.
Оскільки спочатку викликано замикання з рядком (String::from("hello")), замикання стало працювати з типом String.
У наступному виклику передача числа (5), але це викликає помилку, оскільки тип замикання вже визначено як String.

Щоб зробити код універсальним, можна використовувати універсальні типи (generics) у замиканні.
Однак для цього потрібно буде явно оголосити замикання як функцію або використовувати конкретні функції для кожного типу.


*/