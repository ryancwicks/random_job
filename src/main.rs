
use std::env;
use std::fs;
use rand::Rng;

fn main() {
    let filename = "job_list.txt";
    let args: Vec<String> = env::args().collect();

    let num_jobs = match args.len() {
        0 => 1,
        1 => 1,
        _ => match args[args.len()-1].parse::<usize>() {
            Ok(val) => val,
            _ => { 
                    println!("Must pass an integer value for number of jobs.");
                    std::process::exit(1)
                },
            },
        };


    let job_contents = fs::read_to_string(filename).expect ("Something went wrong reading the job file.");
    let job_vec: Vec<&str>  = job_contents.lines().collect();
    let total_jobs = job_vec.len();

    let mut rng = rand::thread_rng();

    println!("\nYour top {} random jobs\n-------------------------", num_jobs);
    for _ in 0..num_jobs {
        let index = rng.gen_range(0, total_jobs);
        let job: Vec<&str> = job_vec[index].trim().split(" ").collect();
        let job_name = job[1..].join(" ");

        println! ("{}", job_name);
    }
    println!("\n");
}
