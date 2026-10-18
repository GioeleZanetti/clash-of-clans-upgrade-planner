use std::collections::HashMap;

use planner::{Planner, Task};

mod planner;

fn main() {
    /*let tasks: Vec<Task> = vec![
        Task::new("Army Camp #1 lvl 11".to_string(), 1 * 24 + 16),
        Task::new("Eagle Artillery lvl 4".to_string(), 1 * 24 + 9),
        Task::new("Army Camp #2 lvl 11".to_string(), 4 * 24),
        Task::new("Hero Hall lvl 8".to_string(), 5 * 24 + 4),
        Task::new("Army camp #4 lvl 11".to_string(), 4 * 24),
        Task::new("Army camp #3 lvl 11".to_string(), 4 * 24),
        Task::new("Lab lvl 12".to_string(), 5 * 24 + 14),
        Task::new("Blacksmith lvl 7".to_string(), 4 * 24),
        Task::new("Town Hall lvl 15".to_string(), 6 * 24 + 9),
        Task::new("Eagle Artillery lvl 5".to_string(), 6 * 24 + 9),
        Task::new("Clan Castle lvl 10".to_string(), 5 * 24 + 14),
        Task::new("Pet House lvl 2".to_string(), 1 * 24 + 14),
        Task::new("Pet House lvl 3".to_string(), 2 * 24 + 9),
        Task::new("Pet House lvl 4".to_string(), 2 * 24 + 19),
    ];

    let dependencies: HashMap<usize, Vec<usize>> = vec![
        (9, vec![1]),
        (12, vec![11]),
        (13, vec![12]),
    ].into_iter().collect();*/

    let tasks: Vec<Task> = vec![
        Task::new("a".to_string(), 1),
        Task::new("b".to_string(), 1),
        Task::new("c".to_string(), 1),
        Task::new("d".to_string(), 1),
        Task::new("e".to_string(), 1),
        Task::new("f".to_string(), 1),
        Task::new("g".to_string(), 1),
        Task::new("h".to_string(), 1),
        Task::new("i".to_string(), 1),
        Task::new("j".to_string(), 1),
        Task::new("k".to_string(), 1),
        Task::new("l".to_string(), 1),
        Task::new("m".to_string(), 1),
        Task::new("n".to_string(), 1),
    ];

    let dependencies: HashMap<usize, Vec<usize>> = vec![
        (1, vec![0]),
        (2, vec![1]),
        (3, vec![2]),
        (4, vec![3]),
        (5, vec![4]),
        (6, vec![5]),
        (7, vec![6]),
        (8, vec![7]),
        (9, vec![8]),
        (10, vec![9]),
        (11, vec![10]),
    ].into_iter().collect();
    let workers = 4;
    let mut p = Planner::new(workers, tasks, dependencies);

    if let Some((tasks, diff)) = p.execute() {
        println!("The min diff is {} and is achived with:", diff);
        for i in 0..workers {
            println!("Worker {}: ({}h) {} ", 
                i + 1, 
                tasks[i].iter().fold(0, |sum: usize, t: &Task| sum + t.duration),
                tasks[i].iter().map(|t: &Task| t.name.clone()).collect::<Vec<String>>().join(", "), 
            );
        }
    }
}


