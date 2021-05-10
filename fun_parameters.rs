fn a_function(x: i32){ //type of variable
    println!("the value of x is {}",x);//
}
fn b_function(y:i32,z:i32){
    println!("the value of y is {} and z is {}",y,z);
}

fn main(){
    println!("this is the main function");
    a_function(5);//calling afunction withh a value as paramter
    b_function(1,2);
}
