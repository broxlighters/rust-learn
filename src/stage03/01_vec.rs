fn main() {
    println!("Stage 03 / Vec");
    println!("本节目标：理解 Vec 的创建、追加、遍历，以及索引和 get 的区别。\n");

    let mut tasks = vec![
        String::from("learn Vec"),
        String::from("read ownership notes"),
    ];

    println!("初始任务列表 = {:?}", tasks);

    tasks.push(String::from("build Todo CLI"));
    println!("push 之后 = {:?}", tasks);

    println!("\n遍历 Vec：");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
    println!("\n索引访问：");
    println!("第一个任务 = {}", tasks[0]);

    println!("\nget 访问：");
    match tasks.get(10) {
        Some(task) => println!("找到任务：{task}"),
        None => println!("第 11 个任务不存在。"),
    }

    let a = match tasks.get(100) {
        Some(task) => task,
        None => panic!("没有第 2 个任务。"),
    };


    println!("\n本节结论：");
    println!("1. Vec 适合存放数量可变的一组数据。");
    println!("2. push 可以在末尾追加元素。");
    println!("3. tasks[index] 直接取值，但越界会 panic。\n4. get 更安全，会返回 Option。");

    println!("\n试一试：");
    println!("1. 再 push 一个任务，观察长度变化。");
    println!("2. 把 tasks[0] 改成 tasks[10]，看看会发生什么。");
    println!("3. 用 get(1) 和 get(100) 对比返回结果。");
}
