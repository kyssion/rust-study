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
    println!("{}",str);
}

pub fn test_fn2(s: &mut String){
    s.push_str("sdfsdf");
}