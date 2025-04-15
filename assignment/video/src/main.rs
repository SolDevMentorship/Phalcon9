fn main() {
    // println!("Hello, world!");
    let mamankechi_basket = String::from("Tomatoes");
    let adamu = &mamankechi_basket; //Adamu does not own the data(basket of tomatoes)

    // BASIC SYNTAX
    let mut score = 10;
    //mut means you can update the value
    // EXAMPLE BELOW
    score = 30;

    println!("Score: {}", score);

    // DATA TYPES
    let name: &str = "Adamu"; //String slice or string literal

    let business_name: String = String::from("Mama nkeci's Business Store"); //heap allocated string

    let height: f32 = 6.2; //floating point number, 32bits

    let age: u8 = 25; //unsigned integer, 8bits, we also have u64 that is you cannot assign a negative value to it

    let account_balanace: i64 = -1000; //signed integer, 64bit

    let is_student: bool = true; //booolean true or false

    // CONTROL FLOW
    // A way of defining movement like conditionals

    let mut account_balance = 10000;
    let withdrawal_amount = 1000000;

    // if account balance is greater than withdrawal amount, withdraw, else print insuffiencnt funds

    if account_balance > withdrawal_amount {
        println!("You just withdrew; {}", withdrawal_amount);
    } else {
        println!("Inssuficient funds")
    }

    //LOOPS
    //for, while, and loop

    //  WHILE
    let mut stock = 5;

    while stock > 0 {
        stock -= 1;
        println!("Selling item... Stock left: {}", stock);
    }

    println!("Out of stock");

    // ERROR HANDLING

    fn check_homework(name: &str) -> Result<&str, &str> {
        if name == "John" {
            Ok("Homework is done")
        } else {
            Err("Homework is not done")
        }
    }

    let result = check_homework("john");
    match result {
        Ok(message) => println!("{}", message),
        Err(message) => println!("{}", message),
    }
}
