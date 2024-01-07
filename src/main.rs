// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}", &region);
//     }
// }

// fn main() {
//     greet_world()
// }

fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for ( i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
    

    // declare a field variable, type is Vec
    // Vec means vector, a growable array
    // <_> is a generic type, which means we can use any type
    let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
    if cfg!(debug_assertions) {
        // output standard error
        eprintln! ("debug: {:?} -> {:?}", record, fields);
    }

    let name = fields[0];
    // 1. try to convert fields[1] to f32 type , if success, give it to length
    // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出length的值
    //   1) 当=右边的表达式执行成功，则会返回一个Ok(f32)的类型，若失败，怎会返回一个Err(e)的类型
    //   2) 同时if let 还会做一次解构匹配，通过ok(length)去匹配右边的Ok(f32)的类型，如果匹配成功，则返回length的值
    // 当然你也可以忽略成功的情况，用if let Err(e) = fileds[1].parse::<f32>() {...}匹配出错误，然后打印出来
    if let Ok(length)= fields[1].parse::<f32>() {
        println!("{}, {}cm", name, length);
    }
    }
}