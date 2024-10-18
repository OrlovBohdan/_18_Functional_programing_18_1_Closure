#[test]

/*
/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
     let movable = Box::new(3);

     let consume = || {
         println!("`movable`: {:?}", movable);
         take(movable);
     };

     consume();
     consume();
}

fn take<T>(_v: T) {}
*/



//1
fn main() {
    let movable = Rc::new(Box::new(3)); // Використовуємо Rc для спільного володіння

    let consume = {
        let movable = Rc::clone(&movable); // Клонування Rc для використання у замиканні
        move || {
            println!("`movable`: {:?}", movable);
            // Не можемо викликати take() з Rc, тому просто передаємо Rc, а не Box
            take(movable.clone());
        }
    };

    consume();
    consume();
}
use std::rc::Rc;

fn take<T>(_v: T) {}

//2
/*fn main() {
    let movable = Rc::new(RefCell::new(Box::new(3))); // Використовуємо Rc та RefCell для змінного доступу

    let consume = {
        let movable = Rc::clone(&movable);
        move || {
            println!("`movable`: {:?}", movable.borrow());
            // Тепер можемо передавати значення у take через borrow
            take(movable.borrow().clone());
        }
    };

    consume();
    consume();
}
use std::rc::Rc;
use std::cell::RefCell;


fn take<T>(_v: T) {}
*/



/*
//1
1. Використовувати Rc для спільного володіння:
Можна використовувати Rc (Reference Counted) для спільного володіння значенням між кількома викликами замикання.
Rc дозволяє кільком частинам коду одночасно володіти ресурсом, але без можливості змінювати його вміст.

//2
2. Використовувати RefCell і Rc для внутрішньої зміни значення:
 залишити можливість модифікувати значення всередині замикання, можна поєднати Rc і RefCell.
Це дозволяє робити змінювані операції над значенням, яке знаходиться всередині Rc.
*/