fn main() {
    println!("Hello, world!");
    let mut num8 = 0;
    let mut num2 =0;
    for n in 1..101 {
        if n %2 ==0 {
            num8 +=1;
        }
        else{
            num2 +=1;
        }
    }
    println!("{}",num8);
    println!("{}",num2);
    println!("{}", factorial(5));

}
fn rec(r: i64) -> i64 {

    let  rec = r + rec(1);
   return rec;
}
fn factorial(i: u64) -> u64 {
    match i {
        0 => 1,
        n => n * factorial(n-1)
    }
}
