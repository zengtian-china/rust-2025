fn main() {
    println!("Hello, world!");
}


trait Computer{
    fn compute(&self,input:&str) -> u32;
}
struct CommandLineComputer;

impl Computer for CommandLineComputer {
    fn compute(&self,input:&str) -> u32 {
        todo!()
    }
}

struct UserType<T:Computer> {
    computer:T,
    expr:String,
}

impl <T: > UserType<T>{
    fn new(computer:T) -> Self{
        Self{
            computer,
            expr:String::new()
        }
    }
    fn get_input(&self) {
        println!("请输入数据");
        io::stdin().read_line(&mut self.expr).expect("Failed to read line");
    }
    fn computer(&self) -> u32{
        self.computer.compute(&self.expr)
    }

}