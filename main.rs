fn main () {
    let mut x = 1..100;

    for i in &mut x {
        if i % 7 == 0{
            println!("bazz");
        }
        if i % 5 == 0 {
            println!("fizz");
        }
        if i % 3 == 0 {
            println!("buzz")
        }
        if i % 5 & i % 3 == 0 {
            println!("fizzbuzz");

    }


    }
}

