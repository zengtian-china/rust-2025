use std::thread;
use std::process::{Command, Stdio};
use std::time::Instant;
use std::env;
use std::io::{BufRead, BufReader};

// 单线程计算
fn single_threaded_sum(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

// 多线程计算
fn multi_threaded_sum(n: u64, num_threads: usize) -> u64 {
    let chunk_size = n / num_threads as u64;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = 1 + i as u64 * chunk_size;
        let end = if i == num_threads - 1 {
            n
        } else {
            start + chunk_size - 1
        };

        let handle = thread::spawn(move || {
            let mut sum = 0;
            for i in start..=end {
                sum += i;
            }
            sum
        });

        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

// 多进程计算
fn multi_process_sum(n: u64, num_processes: usize) -> u64 {
    let chunk_size = n / num_processes as u64;
    let mut children = vec![];

    for i in 0..num_processes {
        let start = 1 + i as u64 * chunk_size;
        let end = if i == num_processes - 1 {
            n
        } else {
            start + chunk_size - 1
        };

        // 启动子进程
        let child = Command::new(std::env::current_exe().unwrap())
            .arg(start.to_string())
            .arg(end.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        children.push(child);
    }

    let mut total = 0;

    for child in children {
        let output = child.wait_with_output().expect("Failed to wait on child");
        let sum_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let sum: u64 = sum_str.parse().expect("Invalid sum from child");
        total += sum;
    }

    total
}

// 子进程逻辑（计算 start..=end 的和）
fn compute_sum(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    for i in start..=end {
        sum += i;
    }
    sum
}

// 验证公式计算
fn formula_sum(n: u64) -> u64 {
    n * (n + 1) / 2
}

// 主函数入口
fn main() {
    let args: Vec<String> = env::args().collect();

    // 如果是子进程
    if args.len() == 3 {
        let start: u64 = args[1].parse().unwrap();
        let end: u64 = args[2].parse().unwrap();
        let sum = compute_sum(start, end);
        println!("{}", sum);
        return;
    }

    // 父进程逻辑
    let n = 1_000_000_000;
    let num_threads = 4;
    let num_processes = 4;

    // 单线程
    let start = Instant::now();
    let single_sum = single_threaded_sum(n);
    let duration = start.elapsed();
    println!("Single-threaded time: {:?}", duration);
    println!("Single-threaded sum: {}", single_sum);

    // 多线程
    let start = Instant::now();
    let thread_sum = multi_threaded_sum(n, num_threads);
    let duration = start.elapsed();
    println!("Multi-threaded time: {:?}", duration);
    println!("Multi-threaded sum: {}", thread_sum);

    // 多进程
    let start = Instant::now();
    let process_sum = multi_process_sum(n, num_processes);
    let duration = start.elapsed();
    println!("Multi-process time: {:?}", duration);
    println!("Multi-process sum: {}", process_sum);

    // 验证结果
    let expected = formula_sum(n);
    assert_eq!(single_sum, expected);
    assert_eq!(thread_sum, expected);
    assert_eq!(process_sum, expected);
    println!("✅ All results are correct!");
}