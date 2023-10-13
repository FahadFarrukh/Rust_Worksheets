
fn numbers(num1:u32)->String{
    if num1%2==0{
        return String::from("Even");
    }
    else{
        return String::from("Odd");
    }
 
}

 
fn main(){
let num=6;

    let answer=numbers(num);
    println!("The number {} is {}",num,answer);

    let mut x=1;
    let mut y=0;
    
    while x<=10{
        y=y+x;
        x=x+1;
    }
    println!("{}",y);
}