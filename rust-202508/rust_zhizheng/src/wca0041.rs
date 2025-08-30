

pub mod zeng{
    use std::thread;
    use std::time::{Duration, Instant};

    pub fn thrent(){

        let thre = thread::spawn(||{
            for i in 1..10{
                println!("这是线程A中第：{}次执行",i);
                thread::sleep(Duration::from_millis(1));
            }


        });
        for i in 1..10{
            println!("这是线程B中第：{}次执行",i);
            thread::sleep(Duration::from_millis(1));
        }
        thre.join().unwrap();



    }


//     计算 1+..+10000000000
    pub fn add(){
        let arge = 10_0000_0000u64;
        let start = Instant::now();
        let mut sum = 0;
        for i in 1..arge{
            sum+=i;
        }
    println!("耗时：{}ms", start.elapsed().as_millis());
    println!("sum is {}",sum);
}
}

