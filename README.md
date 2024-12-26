# Bankers-Algorithm
Rust-based simulation of the Banker's Algorithm, designed for resource allocation and deadlock avoidance.

**Overview:**

The Banker's Algorithm is a resource allocation and deadlock avoidance algorithm, using Rust. The simulation involves multiple processes requesting and releasing resources in a controlled manner to ensure the system remains in a safe state. The program checks if resource requests can be granted without leading to a deadlock.

The system reads an input.txt file that specifies the number of resources, processes, available resources, maximum, allocated, and needed resources for each process. It then runs multiple threads to simulate the processes requesting and releasing resources.

**Features:**
- Safe state check: Ensures the system remains in a safe state after each resource request.
- Resource request handling: Processes can request resources, and the system will grant or deny the requests based on the safe state check.
- Resource release: Processes release resources back to the system once they have completed their tasks.
- Multi-threading: Simulates multiple processes running concurrently using Rust's threading capabilities.

***Run Instructions:***
```
Optionally Edit input.txt File
Build Cargo File (in source directory): cargo build
Run Cargo File: cargo run
```

***Requirements:***
- Rust 
- Cargo

***Input File (input.txt):***
```
2               // Number of resource 
6               // Number of processes
10 5            // Available: 10 units of resource 0 & 5 units of resource 1
7 4 | 2 1 | 5 3 // P0: Max = 7 4, Allocate = 2 1, Need = 5 3
3 3 | 1 2 | 2 1 // P1: Max = 3 3, Allocate = 1 2, Need = 2 1
6 2 | 3 0 | 3 2 // P2: Max = 6 2, Allocate = 3 0, Need = 3 2
4 2 | 1 1 | 3 1 // P3: Max = 4 2, Allocate = 1 1, Need = 3 1
5 4 | 0 2 | 5 2 // P4: Max = 5 4, Allocate = 0 2, Need = 5 2
8 1 | 3 0 | 5 1 // P5: Max = 8 1, Allocate = 3 0, Need = 5 1
```
<p align="left">
(Images truncated due to length)
</p>

![Capture](https://github.com/user-attachments/assets/0fa686b4-5d69-4c5f-aeab-305a17af04a3)





