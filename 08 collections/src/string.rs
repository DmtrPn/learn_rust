fn main() {

    let data = "initial contents";

    let c4 = data.to_string();

    // the method also works on a literal directly:
    let c3 = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    let s3 = s1 + &s2;  // note s1 has been moved here and can no longer be used
}

fn main_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}


fn main_iter() {
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
       println!("{}", b);
    }
}


