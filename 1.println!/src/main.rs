
fn main() {
    println!("Hello, world!");

    let name = "Amiya";
    let age = 18;
    let amiya = "rabbit";

    println!("Her name is {}.",name);
    println!("She is a {1}.And she is {0} years old",age,amiya);    //第一个花括号显示第二个变量，第二个反过来

    println!("{person} {state} {another_person}.",
            person = "I",state = "love",
            another_person = "you"     //设置变量名对应的数值
    );

    let his_name = "Bob";
    let his_age = 30;
    println!("{his_name} is {his_age} years old");                  //将期望打印的变量名放在语句中

    let num = 44;
    println!("|{:10}|",num);        //右对齐（默认）
    println!("|{:<10}|",num);       //左对齐
    println!("|{:>10}|",num);       //右对齐
    println!("|{:^10}|",num);       //居中
    println!("|{:0>7}|",num);       //右对齐并用0填充
    println!("|{:*>10}|",num);

    let pi = 3.1415926535;
    println!("{:.5}",pi);               //保留五位小数

    let hello = "hello,rust!";
    println!("{:.8}",hello);            //截取前八位（字符串默认左对齐）

    let u = 231;                   //禁止转换
    println!("{:b}",u);
    println!("{:#b}",u);
    println!("{:o}",u);
    println!("{:#o}",u);
    println!("{:x}",u);
    println!("{:#x}",u);
    println!("{:X}",u);
    println!("{:#X}",u);
    println!("{:e}",u);
    println!("{:#e}",u);
    println!("{:E}",u);
    println!("{:#E}",u);
    println!("{:p}",&u);
    println!("{:#p}",&u);

    print!("Loading");                  //print!默认不换行 
    print!("...");                      //println!默认在最后会换行
    println!("Done!");
    println!("Well dong!");

    let v = vec![1, 2, 3];
  //println!("{}", v);      //编译错误
    println!("{:?}", v);    //[1, 2, 3]
    println!("{:#?}", v);    //[1, 2, 3]
}