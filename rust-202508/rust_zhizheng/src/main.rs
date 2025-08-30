mod wca0041;
mod qu;

use std::thread;
use std::time::Duration;
use crate::wca0041::zeng;
fn main() {
    // let mut student:Box<_> = Box::new(Student::new(String::from("zengtian")));
    // println!("{}", student.get_name());
    // zeng::thrent();
    zeng::add();
    // let thre = thread::spawn(||{
    //     for i in 1..10{
    //         println!("这是线程A中第：{}次执行",i);
    //         // thread::sleep(Duration::from_millis(1));
    //     }
    //
    //
    // });
    //
    // for i in 1..10{
    //     println!("这是线程B中第：{}次执行",i);
    //     thread::sleep(Duration::from_millis(1 as u64));
    // }
    // thre.join().unwrap();
}


trait Person{
    fn get_name(&self) -> String;
}

struct Student {
    name: String
}

impl Person for Student {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Student {
    fn new(name: String) -> Self {
        Self {
            name
        }
    }
}