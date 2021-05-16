fn five() ->i32 { //stating the function and return type
    5  //dont give ';' here as its an expression we want to return
}

fn main(){
    let x=five(); //assigning the value of the function to the variable by calling
    println!("the value of x is {}",x);
}
