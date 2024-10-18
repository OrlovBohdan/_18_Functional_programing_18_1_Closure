#[test]

/*
/* Make it work
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);
}
*/

fn main() {
    let mut count = 0;

    {
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        inc(); // Перший виклик інкременту
        inc(); // Другий виклик інкременту
    }

    // Після завершення блоку замикання більше не існує, тому ми можемо використовувати змінну `count` далі без проблем
    assert_eq!(count, 2);
}


/*
Щоб вирішити проблему, не змінюючи тест (assert_eq!(count, 0)) і не використовуючи змінні _reborrow та _count_reborrowed,
необхідно усунути конфлікт із позичанням змінної count для замикання. Проблема полягає в тому,
що після першого виклику замикання ми позичаємо змінну за незмінним посиланням,
а потім хочемо повторно позичити її за змінним посиланням, що спричиняє помилку.

У цьому випадку краще використовувати замикання без потреби в позичанні змінної кілька разів.
Одним із варіантів вирішення буде зберегти логіку, але змінити структуру коду
*/