
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


}
