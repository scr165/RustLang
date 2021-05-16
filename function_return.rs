fn main(){
    let _x=3; //assigning a variable
    let y={ //we can also return a variable like this
        let x=5;
        x+1 //if there is ';' at the end it is a statement which won't return a value
    }; //dont forget to give semicolon';' here as it is end of variable
    println!("the value of y is {}",y);
}
