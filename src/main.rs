//CS 490-01 FA-22
//11-15-2022
//Colin O'Kain & Josh Payne
//Created and compiled in VSCode on Windows

extern crate rand;

use std::collections::{VecDeque, BinaryHeap};
use rand::Rng;
use std::cmp::Ordering;

//main program
//purpose: Generate a user-defined amount of processes and randomly assign a priority then output all processes based on creation order then priority order
fn main() {

    println!("Welcome to the process node generator. Please enter the number of process nodes to generate:");

    //Getting user input
    let mut user_input = String::new();
    let _result = std::io::stdin().read_line(&mut user_input);
    user_input = user_input.trim().to_string();

    let user_input_int : i32;
    //Making sure user input is parsable
    if let Ok(result) = user_input.parse::<i32>() {
        user_input_int = result;
    }
    else {
        user_input_int = -1;
    }

    //Creating the queue and heap
    let mut queue: VecDeque<Process> = VecDeque::new();
    let mut min_heap: BinaryHeap<Process> = BinaryHeap::new();

    //Creating all of the processes, cloning each one, and adding them to the queue and heap
    for n in 0..user_input_int{
        let mut rng = rand::thread_rng();

        let process = create_process(n+1, rng.gen_range(0..101), rng.gen_range(100..2000), format!("Process Node {}", n));

        let process2 = process.clone();

        queue.push_back(process);
        min_heap.push(process2);
    }

    println!("Verifying. The queue contains {} elements", queue.len());
    println!("Verifying. The heap contains {} elements\n", min_heap.len());

    println!("Now, draining the Queue, one process at a time ...");

    //Outputting values of processes from queue
    for _n in 0..queue.len()
    {
        if let Some(process)  = queue.pop_front()
        {
            println!("Pid: {}, pri: {}, sleep: {}, desc: {}", process.process_id, process.priority, process.sleep_time, process.description);
        }
    }

    println!(" ");
    println!("Now, draining the MinHeap, one process at a time ...");

    //Outputting values of processes from heap
    for _n in 0..min_heap.len()
    {
        if let Some(process)  = min_heap.pop()
        {
            println!("Pid: {}, pri: {}, sleep: {}, desc: {}", process.process_id, process.priority, process.sleep_time, process.description);
        }
    }
}

//Derives the implementation of the Clone method from std
#[derive(Clone)]

//Process struct definition
struct Process
{
    process_id : i32,
    priority : i32,
    sleep_time : i32,
    description : String
}

//Creates an instance of a process and returns it
fn create_process(pid : i32, priority_input : i32, sleep : i32, desc : String) -> Process {
    let process = Process {
        process_id : pid,
        priority : priority_input,
        sleep_time : sleep,
        description : desc,
    };

    return process;
}

//Definitions on how to order the processes for the heap based on the priority
impl Ord for Process {
    fn cmp(&self, other: &Process) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

//Defines how to test equality of two processes (only tests the priority, doesn't give value of actual object equality)
impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Process { }


