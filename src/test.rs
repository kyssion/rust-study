//拥有所有权要解决的所有问题都是：跟踪代码的哪些部分正在使用堆上的哪些数据，最小化堆上的重复数据的数量以及清理堆上的未使用数据，以免耗尽空间。
// 一旦了解了所有权，您就不必经常考虑堆栈和堆了，但是知道管理堆数据是所有权存在的原因，可以帮助您解释其工作原理。

//todo 所有权转移例子1  ， 同一个作用域 ， 两次赋值 第一个变量失去所有权
pub fn self_info(){
    //所有权. 值所有者唯一 ， 超出范围值删除
    let mut one = String::from("test");
    let two = one;
    //这里不能使用 one了  因为违反了 所有者唯一原则 ， rust 内存回收的时候将会倒置内存多次回收
    //println!("{}",one);

    //Todo 如果使用的话 需要强制拷贝复制

    let one_p = String::from("test");
    let two_p = one_p.clone();
    println!("{}",two_p);

    //TODO 这种问题之后在 堆中分配空间的时候才会遇到 ， 如果使用的 栈空间 默认就是用的copy方法

    let a = 123;
    let b = a;
    println!("{}",b);

    // 范围基于作用域的
    string_info();
}

fn string_info(){
    //这种字符串不可变 栈中创建
    let mut a = "sdfsdf";
    // 堆中创建
    let mut s = String::from(a);
    s.push_str("ppppp");
    // a 使用不可变类创建的所有不能进行修改
    //a+="sdf";
    println!("{} , {}" , a,s);
}

//todo 所有权转移例子2 ， 一个变量进入另一个范围 所有权丢失但是可以重新使用返回值获取所有权
pub fn test_item(){
    let str = String::from("test");
    //这里发生所有权转移，转移到内部作用领域
    let str = test_fn(str);
    println!("{}",str);
}
//这里可以使用元组返回所有权信息
fn test_fn(mut some_string:String)->String{
    some_string.push_str("tet");
    //将所有权返回给上层
    return some_string;
}

//TODO 参考 和借阅

pub fn test_item2(){
    let mut str = String::from("test");
    test_fn2(&mut str);
    test_fn2(&mut str);
    let one = &mut str;

    //todo 注意这个所有权和 变量的所有权 类似 ， 变量的所有权在一个范围中允许所有权转移但是第一个变量将会不可使用
    //todo 但是 参考在一个作用域中将不允许多个所有者
    //主要是为了防止使用指针导致数据征用的问题

    //todo 两个或多个指针同时访问相同的数据。
    // 至少有一个指针用于写入数据。
    // 没有用于同步对数据访问的机制。
    //let two = &mut str;
    //println!("{},{}",one,two);

    //TODO 注意如果不可变引用和可变引用同时存在的时候 ， 应该保证产生不可变之后到使用不可变引用之间  没有任何修改数据的操作（指针或者值）， 没有任何的可变引用
    let r3 = &mut str; // no problem
    println!("{}",r3);

    let r1 = &str; // no problem
    let r2 = &str; // no problem
    println!("{} {}",r1,r2);

    println!("{}",str);
}

pub fn test_fn2(s: &mut String){
    s.push_str("sdfsdf");
}

//todo 返回内部引用 错误写法 ： s 引用的作用是在 内部的 ， 返回一个可修改的没有意义
// pub fn test_fn3() -> &mut String{
//     let mut s  = String::from("sdf");
//     return &mut s;
// }

//TODO 错误 因为 s 在最后会释放掉 ， 所以指针将会变成悬空指针
// pub fn test_fn3() -> &String{
//     let mut s  = String::from("sdf");
//     return &s;
// }


//todo 直接返回变量 ， 所有权转移，而不是使用参考 ， 指针

pub fn test_fn3() -> String{
    let mut s  = String::from("sdf");
    return s;
}

pub fn test_splic(){
    let mut str = String::from("test one item");
    let index = test_fn4(&str);
    println!("{}",index);
    str.clear();
    println!("{}",str);

    //字符串切片 , rust 中的切片属于引用，操作方法和引用类似

    let mut string = String::from("test splice item ");
    let one = &mut string[1..2];
    let mut one_str = String::from(one);
    one_str.push_str("ttttt");
    println!("{}", one_str);

    //todo 注意切片返回的是不可变引用
    let mut arr = [1,3,4,5,6,7,8,9];
    let new_one = test_split(&mut arr);
    println!("{}",new_one[0]);

}

fn test_fn4(s:&String)->usize{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item== b' '{
            return i;
        }
    }
    return s.len();
}

fn test_split(arr:&mut [i32]) -> & [i32]{
    return & arr[3..arr.len()];
}