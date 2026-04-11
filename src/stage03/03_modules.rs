mod task_service {
    pub struct Task {
        pub title: String,
        pub done: bool,
    }

    impl Task {
        pub fn mark_done(&mut self) {
            self.done = true;
        }

        pub fn print_task(&self) {
            let status = if self.done { "[x]" } else { "[ ]" };
            println!("{status} {}", self.title);
        }
    }

    pub fn build_task(title: &str) -> Task {
        Task {
            title: title.to_string(),
            done: false,
        }
    }

}

use task_service::build_task;

fn main() {
    println!("Stage 03 / Modules");
    println!("本节目标：理解为什么代码变多后要拆模块，以及怎么用 mod 和 use。\n");

    // let task = task_service::build_task("learn module basics");
    // task_service::print_task(&task);

    let mut task = build_task("learn module basics");
    task.print_task();

    task.mark_done();

    task.print_task();
    
    println!("\n本节结论：");
    println!("1. mod 用来声明模块，把相关代码放在一起。");
    println!("2. pub 决定模块外能不能访问。");
    println!("3. use 可以把长路径导入成更短的名字。");

    println!("\n试一试：");
    println!("1. 再给 task_service 增加一个 mark_done 函数。");
    println!("2. 去掉 pub，看看哪些地方会报错。");
    println!("3. 不用 use，改成 task_service::print_task(&task) 试试。");
}
