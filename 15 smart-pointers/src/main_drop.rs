struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(unused_variables)]
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    let d = CustomSmartPointer {
        data: String::from("some data for D"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
