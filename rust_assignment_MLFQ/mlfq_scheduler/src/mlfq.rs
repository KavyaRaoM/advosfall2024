// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // TODO: Implement this function
        // Add the process to the appropriate queue based on its priority
        // Ensure the priority is within the valid range (0 to num_levels - 1)
        // This function adds a process to a queue based on its priority level.
        // The queues are managed by priority, with higher priorities processed first.
    
        // Determine the process priority, ensuring it is within valid bounds.
        let priority = if process.priority < self.num_levels {
            process.priority  // Use the process's priority if it is within the allowed range.
        } else {
            self.num_levels - 1 // If the priority is too high, assign it to the lowest priority queue.
        };
    
        // Add the process to the appropriate priority queue.
        self.queues[priority].push(process);
    }

    pub fn execute_process(&mut self, queue_index: usize) {
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete
        // This function handles the execution of a process from the specified queue.
        // It will run the process for its allocated time quantum or until the process finishes.
    
        if let Some(mut process) = self.queues[queue_index].pop() {
            // Check if there is a process to execute in the queue.
            // Remove (pop) the process from the queue to execute it.
    
            let quantum = self.time_quanta[queue_index];
            // Get the time quantum for this priority level.
            
            let execution_time = quantum.min(process.remaining_time);
            // Determine how much time to execute: either the quantum or the remaining process time,
            // whichever is smaller, to avoid over-executing the process.
    
            // Update the process's remaining execution time.
            process.remaining_time -= execution_time;
            
            // Track the total time the process has been executed so far.
            process.total_executed_time += execution_time;
            
            // Update the system's current time based on the execution time of this process.
            self.current_time += execution_time;
    
            // Check if the process has completed.
            if process.remaining_time > 0 {
                // If the process hasn't completed, move it to a lower-priority queue.
                
                let new_priority = (process.priority + 1).min(self.num_levels - 1);
                // Calculate the new priority, ensuring it doesn't exceed the lowest priority queue.
                
                process.priority = new_priority;
                // Update the process's priority to the lower level.
    
                self.queues[new_priority].push(process);
                // Add the process back into the lower-priority queue.
            } else {
                // If the process has completed, no further action is needed.
                // The process is not added back into any queue since it's finished.
            }
        }
        // If no process was available in the queue, the function simply exits without doing anything.
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0
        // This function performs a priority boost by moving all processes 
        // from lower-priority queues to the highest-priority queue (queue 0).
        // It resets the priority of all processes to 0.

        let mut boosted_processes = Vec::new();
        // Create a temporary vector to hold all the processes that will be boosted.
        
        // Collect all processes from lower priority queues (queues 1 and below).
        for queue in &mut self.queues[1..] {
            boosted_processes.append(queue);
            // Move all the processes from each lower queue into the `boosted_processes` vector.
        }

        // Reset the priority of all the collected processes.
        for process in boosted_processes.iter_mut() {
            process.priority = 0;
            // Set the priority of each process to 0, which is the highest priority.
        }

        // Add all the boosted processes into the highest-priority queue (queue 0).
        self.queues[0].append(&mut boosted_processes);
        // Move the boosted processes into queue 0, leaving the temporary vector empty.
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}