fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

      let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Метод split_whitespace возвращает итератор по срезам строки,
    // разделённых пробелам, для строки text.
    // Метод or_insert возвращает изменяемую ссылку (&mut V) на значение ключа.
    // Мы сохраняем изменяемую ссылку в переменной count,
    // для этого, чтобы присвоить переменной значение,
    // необходимо произвести разыменование с помощью звёздочки (*).
    // Изменяемая ссылка удаляется сразу же после выхода из области видимости цикла for,
    // поэтому все эти изменения безопасны и согласуются с правилами заимствования.


    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
