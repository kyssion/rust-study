use std::cmp::Ordering;
use std::io::stdin;
use rand::Rng;
mod test;
mod test3;

fn main(){
    // game();
    // test_type_base();
    // if_else_math(false);
    // test_fn_1();
    test3::test1();
}

fn game(){
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop{
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess :u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };
        println!("You guessed : {}",guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");break;
            }
        }
    }
}

fn test_type_base(){
    //对原来的值进行修改？
    let mut x=5;
    println!("the value of x is : {}",x);
    x = 6;
    println!("the value  of x is : {}",x);
    //常量必须表示出类型信息
    const MAX_POINTS : u32 = 100_000;
    println!("this value of MAX_POINTS is :{}",MAX_POINTS);
    //使用rust 遮蔽特性 ， 重新定义重名的变量名称
    let x = "this is new Str";

    println!("the value of x is : {}", x);
}

fn test_type_base2(){
    //类型转化
    let num :i32 = "123".parse().expect("类型转化失败");
    let long_num : isize = 1000000000000;
    let y : f64 =  0.3334;
    let char_demo = 's';
    //符合类型 元祖
    let tup:(i32,u32,&str) = (244,333,"sdf");
    //元祖访问方法
    println!("data：{} {} {} {} {}",num,long_num,y,char_demo,tup.0);

    let arr = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    //初始化快速分配数组大小
    let mut  item = [1;100];
}

/**
 * 表达式
 */
fn test_fn_1() {
    //rust 是表达式编程语言 ， let 阈值语句并不是表达式 ， 使用{}之后强制变成表达式
    let x = {
        let y = 123;
        y+1
    };
    println!("{}",x);
    let a = rust_return_test();
}

//如果不使用 ； 结尾表示这个是要返回的语句
fn rust_return_test()->i32{
    //注意使用这种表达式的时候一定要有地方可以接收返回值
    return loop {
        let b = if 123>111 {
             123
        }else{
            444
        };
    }
}

//fn rust 控制流

fn if_else_math(i :bool){
    //rust 的控制流程是一个表达式
    let mut is_can_returen = i;
    let ans = if is_can_returen {
        let mut b = 123;
        b+=1;
        123
    } else{
        2222
    };

    //循环表达式取值方法
    let mut ans = loop {
        break 123;
    };
    println!("{}",ans);
    //while是一个表达式但是并不会从 break中返回值
    let item = while ans<1000 {
        ans+=1;
    };
    let ans = for number in 1..4 {

    };
}