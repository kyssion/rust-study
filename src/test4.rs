
#[derive(Debug)]
struct Range{
    weight : i32,
    height : i32,
    testp : test
}
#[derive(Debug)]
struct  test{
    one : i32,two : i32
}


impl Range{
    fn area(&self)-> i32{
        return self.height*self.weight;
    }
    fn test(){
        println!("this is test");
    }
}
pub fn test1(){
    let mut item = Range{
        weight:12,height:33,testp : test{
            one:33,two:33
        }
    };
    println!("{:#?}",area(&mut item));
    println!("{:#?}",item);
    Range::test();
    test_enum();
    test_ver();
}

fn area(item :&mut Range)->i32{
    item.height+=1;
    return item.height*item.weight;
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test_enum(){
    let a = Message::Quit;
}

fn test_enumo_option(item : String) -> Option<i32> {
    return if item.starts_with("ppp"){
        Some(12)
    }else{
        None
    };
}

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            //这个地方很有意思，使用模式拆解匹配命中的枚举中的值，然后返回对应的数据
            Some(i) => Some(i + 1),
        };
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            //注意完全匹配符号_
            _ => (),
        }
        //if let 等价替换match
        let a = if let 1 = some_u8_value {

        }else if let 2 = some_u8_value{

        }else {};
        return None;
    }

fn test_vec(){
    let mut item: Vec<i32> = Vec::new();
    item.push(1);
    item.push(2);
    item.push(3);
    println!("{:#?}",item.get(3));

    let mut item = vec![1,2,3,4,5];
    let mut index = 0;
    let length = item.len();
    while index< length>>1{
        let num = item[index];
        //注意数组类型这个地方对于rust来说是有特殊情况的 item.len 是不可变借用
        let indexp = item.len()-1-index;
        //todo get 方法可以 使用这种写法
        item[index] = item[item.len()-1-index];
        //todo set这种写写法就有问题，很奇怪
        item[indexp] = num;
        index+=1;
    }
    println!("{:#?}",item);
}

fn test_ver(){
    Vec::new();
    let mut item = vec![1,2,3,4,5];
    //todo 这个地方用& 和不用这个个结果相同，不知道为啥
    let p = &item[3];
    let p = item[3];
    println!("{}",item[3]);
}

fn test_str(){
    let item = String::from("test");
    let item2 = String :: from("test2");
    let item3 = item+&item2;
}

