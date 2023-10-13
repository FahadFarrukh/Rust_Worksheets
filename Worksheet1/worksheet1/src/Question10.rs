fn add(num1:u32,num2:u32)->u32{
    num1+num2
}
fn sub(num1:i32,num2:i32)->i32{
    num1-num2
}
fn mul(num1:u32,num2:u32)->u32{
    num1*num2
}
fn div(num1:u32,num2:u32)->u32{
    num1/num2
}

fn main(){

    let addition=add(5,6);
    let subtract=sub(5,6);
    let multiply=mul(5,6);
    let divide=div(5,6);

    println!("\nAddition = {}",addition);
    println!("\nSubtraction = {}",subtract);
    println!("\nMultiplication = {}",multiply);
    println!("\nDivision = {}",divide);

}