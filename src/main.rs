//CS 490-01 FA-22
//11-15-2022
//Colin O'Kain & Josh Payne
//Created and compiled in VSCode on Windows

use std::collections::BinaryHeap;   // For the process heap
use rand::Rng;                      // For random number generation
use std::cmp::Ordering;             // Used in Process methods for ordering minheap
use std::thread;                    // For the individual producer/consumer threads
use std::time::Duration;            // For the sleep time in threads
use std::sync::{Arc, Mutex};        // For concurrent control of the process heap

fn main() {
    // Generation phases input
    println!("Enter number of generation phases for the producer:");
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read input");
    let num_of_phases : i32 = input.trim().parse().unwrap();

    // Sleep time input
    println!("Enter sleep time in ms for the producer to pause between generation phases:");
    input  = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read input");
    let sleep_time : i32 = input.trim().parse().unwrap();

    // Number of processes input
    println!("Enter number of processes to generate each phase:");
    input  = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read input");
    let num_of_processes : i32 = input.trim().parse().unwrap();

    println!("\nStarting Simulation");
    
    // Defining the minheap that stores the individual processes
    let process_heap : Arc<Mutex<BinaryHeap<Process>>> = Arc::new(Mutex::new(BinaryHeap::new()));

    // PRODUCER THREAD
    println!("... producer is starting its work ...");
    let heap_reference1 = Arc::clone(&process_heap);    // A thread safe reference to the shared process heap
    let producer = thread::spawn(move || {
        for phase_number in 0 .. num_of_phases {        // Loop through each phase
            {   // BEGIN CRITICAL SECTION
                let mut heap = heap_reference1.lock().unwrap(); // Unlock the heap and get a reference to it
                for process_number in 1 .. num_of_processes + 1 {   // Loop through each process to generate per phase
                    let process_id = (phase_number * num_of_processes) + process_number; 
                    let process = Process::new(process_id, "Process".to_string());  // Generating this iteration's process
                    heap.push(process); // Pushing the process to the process heap
                }
            }   // END CRITICAL SECTION

            // In between phases, sleep for the specified time
            println!("\n... producer is sleeping ...\n");
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }

        // After all phases are complete, print to console that the producer has finished
        println!("\n... producer has finished: {} nodes were generated ...\n", num_of_phases * num_of_processes);
    });

    // CONSUMER 1 THREAD
    let heap_reference2 = Arc::clone(&process_heap);    // A thread safe reference to the shared process heap
    let consumer1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));   // Sleeping for a short period of time before starting so that the producer will have time to generate some processes

        let mut executed_count = 0;     // Counter to keep track of the number of processes this specific consumer has executed
        let mut process : Process;      // Local reference to a specific process that will be popped from the heap
        loop {
            { // BEGIN CRITICAL SECTION
                let mut heap = heap_reference2.lock().unwrap(); // Unlock the heap and get a reference to it

                if heap.len() == 0 {    // If the heap is empty, the consumer has finished
                    break;              // Break out of the loop so the thread may close
                }

                process = heap.pop().unwrap();  // If the heap is not empty, pop the next process from the heap and store it in the local reference
            } // END CRITICAL SECTION

            thread::sleep(Duration::from_millis(process.sleep_time as u64));    // Simulate executing the process
            println!("\tConsumer1: executed process {}, pri: {}, for {} ms", process.id, process.priority, process.sleep_time);
            executed_count += 1;    // Increment the counter of processes executed by this consumer
        }

        println!("\n...Consumer1 has completed and executed {} processes", executed_count);
    });

    // CONSUMER 2 THREAD
    let heap_reference3 = Arc::clone(&process_heap);    // A thread safe reference to the shared process heap
    let consumer2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));   // Sleeping for a short period of time before starting so that the producer will have time to generate some processes

        let mut executed_count = 0;     // Counter to keep track of the number of processes this specific consumer has executed
        let mut process : Process;      // Local reference to a specific process that will be popped from the heap
        loop {
            { // BEGIN CRITICAL SECTION
                let mut heap = heap_reference3.lock().unwrap();     // Unlock the heap and get a reference to it

                if heap.len() == 0 {    // If the heap is empty, the consumer has finished
                    break;              // Break out of the loop so the thread may close
                }           

                process = heap.pop().unwrap();  // If the heap is not empty, pop the next process from the heap and store it in the local reference
            } // END CRITICAL SECTION

            thread::sleep(Duration::from_millis(process.sleep_time as u64));        // Simulate executing the process
            println!("\t\tConsumer2: executed process {}, pri: {}, for {} ms", process.id, process.priority, process.sleep_time);
            executed_count += 1  // Increment the counter of processes executed by this consumer
        }

        println!("\n...Consumer2 has completed and executed {} processes", executed_count);
    });
    


    // wait for all threads to finish before exiting the main program thread
    producer.join().unwrap();
    consumer1.join().unwrap();
    consumer2.join().unwrap();

    println!("\n\nBoth consumers have completed.");
}


#[derive(Eq)]           // Deriving basic methods for Process. These methods are used for the minheap ordering
#[derive(PartialEq)]
#[derive(Clone)]        // Used to clone Process before pushing into queue/heap
struct Process {        // Process struct that gets placed in the queue/heap
    id : i32,
    priority : i32,
    sleep_time : i32,
    description : String
}

impl Process {  // Defining some basic methods for the Process struct
    pub fn new(process_id : i32, process_description : String) -> Self {    // Constructor for Process
        let process_priority = rand::thread_rng().gen_range(0..101);        // Making priority a random integer in the range 0 through 100
        let process_sleep_time = rand::thread_rng().gen_range(100..2001);   // Making sleep time a random integer in the range 100 to 2000

        Process {   // Building the struct using parameters and the random values above
            id : process_id,
            priority : process_priority,
            sleep_time : process_sleep_time,
            description : process_description
        }
    }
}

impl Ord for Process {  // Defining the ord method for Process
    fn cmp(&self, p2: &Self) -> Ordering {
        self.priority.cmp(&p2.priority)
    }
}

impl PartialOrd for Process {   // Defining the partial ord method for the Process struct. This allows for comparative operators between two Processes
    fn partial_cmp(&self, p2: &Self) -> Option<Ordering> {
        p2.priority.partial_cmp(&self.priority)  // Comparing in this order so that heap is sorted as a minheap and not a maxheap
    }
}

