// Authors: Victor Vu 
// File: main.rs
// Description: Main file for the Banker's Algorithm simulation
// Copyright (C) 2024 Victor V. Vu 
// License: GNU GPL v3 - See https://www.gnu.org/licenses/gpl-3.0.en.html
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

#[derive(Debug, Clone)] // Derive traits for printing and copy of struct
struct SystemState { // Struct to hold the state of the system (structs must use camel case)
    available: Vec<i32>, // available resources
    max: Vec<Vec<i32>>, // max resources each process can request
    allocated: Vec<Vec<i32>>, // resources currently allocated to each process
    need: Vec<Vec<i32>>, // resources each process still needs
    completed_process: Vec<bool>, // mark if a process has completed
}

fn safe_check(state: &SystemState) -> bool { // Function to see if the system is in a safe state
    let mut cloned_resource = state.available.clone(); // clone the available resources
    let mut done_process = vec![false; state.max.len()]; // vector of bool to track of completed processes

    for _ in 0..state.max.len() { // Loop through the processes
        let mut found = false; // start set found to false
        for i in 0..state.max.len() { 
            if !done_process[i] && state.need[i].iter().zip(&cloned_resource).all(|(n, w)| n <= w) { // check if process is not done & need is less than resources
                for j in 0..cloned_resource.len() { 
                    cloned_resource[j] += state.allocated[i][j]; // add allocated resources to cloned resources
                }
                done_process[i] = true; // mark process as done
                found = true;
            }
        }
        if !found { // if no process is found, break the loop
            break;
        }
    }
    done_process.iter().all(|&f| f) // return true if all processes are done
}

fn request_resource(state: &mut SystemState, process_id: usize, request: &[i32]) -> bool { // Function to request resources
    if request.iter().zip(&state.need[process_id]).any(|(r, n)| r > n) // check if request is greater than need
        || request.iter().zip(&state.available).any(|(r, a)| r > a) // check if request is greater than available
    {
        return false; // false if request exceeds need/available resources
    }

    for i in 0..request.len() { // loop through the request
        state.available[i] -= request[i]; // subtract request from available resources
        state.allocated[process_id][i] += request[i]; // add request to allocated resources
        state.need[process_id][i] -= request[i]; // subtract request from needed resources
    }

    if safe_check(state) { // check if the system is in a safe state
        true 
    } else { // if not in a safe state
        for i in 0..request.len() { 
            state.available[i] += request[i]; // add request back to available resources
            state.allocated[process_id][i] -= request[i]; // subtract request from allocated resources
            state.need[process_id][i] += request[i]; // add request back to needed resources
        }
        false 
    }
}

fn release_resource(state: &mut SystemState, process_id: usize) { // Function to release resources
    for i in 0..state.available.len() { 
        state.available[i] += state.allocated[process_id][i]; // add allocated resources back to available resources
        state.allocated[process_id][i] = 0; // set allocated resources to 0
        state.need[process_id][i] = state.max[process_id][i]; // set needed resources to max resources
    }
    state.completed_process[process_id] = true; // mark process as completed
}

fn process_thread(system_state: Arc<Mutex<SystemState>>, process_id: usize) { // Function to simulate process resource requests
    let mut random = rand::thread_rng(); // create a random number generator

    while !system_state.lock().unwrap().completed_process[process_id] { 
        let request: Vec<i32> = { // create a vector of random requests
            let state = system_state.lock().unwrap(); // lock the system state
            state.need[process_id] // get the needed resources for the process
                .iter() // iterate through the needed resources
                .map(|&n| random.gen_range(0..=n)) // generate random number between 0 and needed resource
                .collect() // collect the random numbers into a vector
        };

        let mut state = system_state.lock().unwrap(); // lock the system state

        if request_resource(&mut state, process_id, &request) { // check if the request can be granted
            println!("Process {}: Requesting {:?} ... Process {}: Request granted", process_id, request, process_id); 
            println!(); // add a newline

            if state.need[process_id].iter().all(|&n| n == 0) { // check if all needed resources are 0
                println!("Now available: {:?}", state.available);
                println!("Process Maximum | Allocation | Need");
                println!("--------------------------------------------");

                for (i, ((max, alloc), need)) in state.max.iter().zip(&state.allocated).zip(&state.need).enumerate() { // print the current state
                    if state.completed_process[i] { // print completed if process is done
                        println!("P{} --- completed ---", i);
                    } else { // print the process state
                        println!("P{} {:?} | {:?} | {:?}", i, max, alloc, need);
                    }
                }
                println!("Process {}: has all resources it needs ==> Resources released...", process_id);
                println!(); 

                release_resource(&mut state, process_id); // release resources
            }
        } else { // if request is denied
            println!("Process {}: Requesting {:?} ... Process {}: Request denied", process_id, request, process_id);
            println!();
        }

        println!("Now available: {:?}", state.available);
        println!("Process Maximum | Allocation | Need");
        println!("--------------------------------------------");

        for (i, ((max, alloc), need)) in state.max.iter().zip(&state.allocated).zip(&state.need).enumerate() { // print the current state
            if state.completed_process[i] {
                println!("P{} --- completed ---", i);
            } else { 
                println!("P{} {:?} | {:?} | {:?}", i, max, alloc, need);
            }
        }
        drop(state); // drop the lock
        thread::sleep(std::time::Duration::from_millis(250)); // sleep 0.25sec to simulate process
    }
} 

fn read_input(file_path: &str) -> SystemState { // Function to parse the input file
    let file = File::open(file_path).expect("Can't open input file"); // open the input file
    let reader = BufReader::new(file); // create a buffer reader
    let mut lines = reader.lines(); // create an iterator over the lines
    let resource_amount = lines.next().unwrap().unwrap().parse().unwrap(); // parse number of resources
    let process_amount = lines.next().unwrap().unwrap().parse().unwrap(); // parse number of processes

    // Parse available resources
    let available: Vec<i32> = lines.next().unwrap().unwrap() 
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if available.len() != resource_amount { // Check if available resources match resource amount
        panic!("Available resources and number of resources don't match!");
    }

    let mut max = vec![vec![0; resource_amount]; process_amount]; // create a vector of max resources
    let mut allocated = vec![vec![0; resource_amount]; process_amount]; // create a vector of allocated resources
    let mut need = vec![vec![0; resource_amount]; process_amount]; // create a vector of needed resources

    for i in 0..process_amount { // Loop through the processes
        let line = lines.next().unwrap().unwrap(); // get the next line
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect(); // split the line by '|'
        if parts.len() != 3 { // Check if the line has 3 parts
            panic!("Line must contain 3 parts separated by |");
        }

        max[i] = parts[0].split_whitespace().map(|s| s.parse().unwrap()).collect(); // parse max resources
        allocated[i] = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect(); // parse allocated resources
        need[i] = max[i].iter().zip(&allocated[i]).map(|(m, a)| m - a).collect(); // calculate needed resources
    }

    SystemState { // Return the system state
        available, max, allocated, need, 
        completed_process: vec![false; process_amount], // mark all processes as not completed
    }
}

fn main() { // Main function
    let file_path = "input.txt"; // input file name
    let system_state = Arc::new(Mutex::new(read_input(file_path))); // read file and create a mutex

    { // Lock the system state in a new scope
        let state = system_state.lock().unwrap(); 
        // Print the initial state
        println!("Now available: {:?}", state.available);
        println!("Process Maximum | Allocation | Need");
        println!("--------------------------------------------");

        for (i, ((max, alloc), need)) in state.max.iter().zip(&state.allocated).zip(&state.need).enumerate() { 
            println!("P{} {:?} | {:?} | {:?}", i, max, alloc, need);
        }
        println!(); // new line
    } // Drop the lock

    let mut threads = vec![]; // create vector of threads
    
    for process_id in 0..system_state.lock().unwrap().max.len() { 
        println!("Starting thread for Process {}.", process_id);
        let system_state_clone = Arc::clone(&system_state); // clone the system state
        let thread = thread::spawn(move || { // spawn a new thread
            process_thread(system_state_clone, process_id);
        });
        threads.push(thread); // push the thread to the vector
    }

    for thread in threads { // Join all threads
        thread.join().unwrap();
    }
    println!("All processes have finished.");
}