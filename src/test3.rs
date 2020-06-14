pub fn test1(){
    let a = "111";
    {
        println!("a: {}",a);
        let b = "222";
    }
    //错误超出了所有权范围
    // println!("b:{}",b);
    test();
}

fn test2(){
    let a = {
        let b = "222";
        b
    };
    //错误超出了所有权范围
    println!("a:{}",a);
}

fn test3(){
    let a= String::from("test");
    let b = a;
    //错误 ， 所有权转移了
    //println!("{}",a);
    //------可以使用clone方法给予新的值

    let a = String::from("test");
    let b = a.clone();
    println!("{}",a);
}
//
fn test4(){
    let a = String::from("test1");
    /// a 变量所有权下移，但前失去了a的所有权
    let b = add_str_no_clone(a);
    /// b 获取了 新的所有权范围，所以可以使用了
    println!("{}",b);
}

/// 内部生成一个新的变量 ，重新返回给上层，将所有权付给上层 ， 原来的值将回收
fn add_str(str : String) -> String{
    let mut new_str = str.clone();
    new_str.push_str("_add_ok");
    return new_str
}

/// 将但前的所有权转移下去 ， 并且再将原来的值返回出来
fn add_str_no_clone(mut str : String) -> String {
    str.push_str("_add_ok");
    return str;
}

fn test5() {
    let mut a = String::from("test");
    let mut b = add_str_l(&mut a);
    let mut c = &mut b;
    /// 错误 ，这个时候所有权已经转移到了b中了
    ///a.push_str("test");

    c.push_str("test");
    b.push_str("test");
    println!("{}",a);
}

fn add_str_l(str: &mut String) -> &mut String{
    str.push_str("_add_ok_l");
    return str;
}

fn test6(){
    let mut a = String::from("test");
    let item = &a;
    let item2 = &a;
    ///
    ///  这之间不能存在修改原始数据的逻辑，和 元数据的&mut 引用
    ///
    ///
    println!("{} {}",item,item2);

    /// 之后就可以进行处理了
    a.push_str("test");
    println!("{}",a);
    /// a 的第一次借阅
    let item = &mut a;
    /// 这个函数也用了借阅 所以是第二次
    a.push_str("test");
    /// 以第二次为准 ， 使用第一次出现问题
    println!("{}",a);
}

fn test7(){
    let mut a = String::from("test");
    let mut core = String::from("test");
    // let mut b = &mut a;
    let mut b = add_str_ls(&mut a);

    let mut c = &mut a;
    let mut d = add_str_ls(&mut core);
    // let mut d =&mut b;

    c.push_str("ttt");
    d.push_str("test");
    println!("{}",a);
}

fn add_str_ls(str: &mut String)->&mut String{
    str.push_str("_add_ok_l");
    return str;
}


fn test(){
    let ccc = 123;
    let mut a = [1,2,3,4,5,6];
    let b = &a[1..3];
    println!("{:?}",b);
}
