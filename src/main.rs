#![allow(unused)]
#[derive(Debug)]
struct My(bool,isize,usize);
#[derive(Debug)]
struct MyInfo{
    name:&'static str,
    cgpa:f64,
    num:i64
}
fn main() {
    let m=My(true,-5,5);
    //println!("{:?}",m);
    let x=MyInfo{
        name:"Tanvir",
        cgpa:3.59,
        num:01534103985
    };
    println!("{:?}",x)
   
}
fn number(x:usize){
    if x==5{
        println!("It is 5")
    }else if x==6{
        println!("It is 6")
    }else{
        println!("you")
    }
}
fn name(k:bool){
    while k==true{
        println!("Tanvir");
    }
}

