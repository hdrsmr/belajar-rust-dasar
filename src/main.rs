fn main() {
    println!("Hello, world!");
}


#[test]
fn hello_test() {
    println!("Hello World1");
}


#[test]
fn test_variable() {
    let name = "Hendra sumarno";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Sundus Khair";
    println!("Hello {}", name);

    name = "Bani fay";
    println!("Hello {}", name);
}


#[test]
fn static_text() {
    let name = "Sundus Khair";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Sundus Khair";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn explicit(){
    let age : i32 = 20;
    println!("Hello {}", age);
}


#[test]
fn number(){
    let a:i8  = 20;
    println!("Hello {}", a);

    let b :f32  = 10.12;
    println!("Hello {}", b);

}

#[test]
fn number_conversion(){
    let a:i8  = 20;
    println!("Hello i8 {}", a);
    let b:i16  = a as i16;
    println!("Hello i16 {}", b);
    let c:i32  = a as i32;
    println!("Hello i32 {}", c);

    let d: i64  = 100000000000;
    let e:i8  = d as i8;
    println!("Hello i8 {}", e);


}

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 20;
    let c = a * b;
    let d = a + b;
    let e = a - b;
    let f = a / b;
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("d = {}, e = {}, f = {}", d, e, f);
}

#[test]
fn augmented_operator(){
    let mut  a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean_operator(){
    let a = true;
    let b: bool = false;
    println!("a = {}, b = {}", a, b);
}
