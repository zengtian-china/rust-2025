use std::io;
use std::io::Write;
fn main() {
    println!("Hello, world!");
    let mut wood = UserType::new(CommandLineComputer);
    loop {
        wood.get_input();
        println!("computer: {:?}", wood.computer());

    }


}


trait Computer{
    fn compute(&self,input:&str) -> i32;
}
struct CommandLineComputer;

impl Computer for CommandLineComputer {
    fn compute(&self,input:&str) -> i32 {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let sum;
        let mut op:Option<char> = None;

        for chr in input.trim().chars(){
            if chr.is_digit(10){
                if op.is_none(){
                    num1.push(chr);
                }
                else{
                    num2.push(chr);
                }
                continue;
            }

            match chr {
                    '+' | '-' | '*' | '/' if op.is_none() =>  op = Some(chr),
                    _ if chr.is_whitespace() => continue,
                    _ => panic!("Invalid op")
                }

        }
        if num1.is_empty() || num2.is_empty() || op.is_none(){
            panic!("Invalid input");
        }
        let num1 = num1.parse::<i32>().unwrap();
        let num2 = num2.parse::<i32>().unwrap();
        let op = op.unwrap();

        match op{
            '+' => sum = num1 + num2,
            '-' => sum = num1 - num2,
            '*' => sum = num1 * num2,
            '/' => sum = num1 / num2,
            _ => panic!("Invalid op")

        }

        sum
    }
}

struct UserType<T:Computer> {
    computer:T,
    expr:String,
}

impl <T: Computer> UserType<T>{
    fn new(computer:T) -> Self{
        Self{
            computer,
            expr:String::new()
        }
    }
    fn get_input(&mut self) {
        self.expr.clear();
        println!("请输入数据");
        io::stdout().flush().expect("Could not flush stdout") ;
        io::stdin().read_line(&mut self.expr).expect("Failed to read line");
    }
    fn computer(&self) -> i32{
        self.computer.compute(&self.expr)
    }

}