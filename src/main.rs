use std::any::Any;

fn print_type_of<T: Any>(value: &T) {
    println!("Тип значения: {}", std::any::type_name::<T>());
    println!("Идентификатор типа: {:?}", value.type_id());
}

fn main() {
    let a = 5;
    let b = "hello";
    let c = vec![1, 2, 3];

    print_type_of(&a);
    print_type_of(&b);
    print_type_of(&c);
}