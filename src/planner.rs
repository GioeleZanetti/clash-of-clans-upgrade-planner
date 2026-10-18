use std::collections::HashMap;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub duration: usize,
    
}

impl Task {
    pub fn new(name: String, duration: usize) -> Self {
        Self {name, duration}
    }
}

pub struct Planner {
    pub workers: usize,
    workloads: Vec<usize>,
    tasks: Vec<Task>,
    tasks_prefix: Vec<usize>,
    curr_state: Vec<Vec<usize>>, 
    min_diff: usize,
    min_state: Vec<Vec<usize>>,

    dependencies: HashMap<usize, Vec<usize>>,
    tasks_end: HashMap<usize, usize>,
}

impl Planner {
    pub fn new(workers: usize, tasks: Vec<Task>, dependencies: HashMap<usize, Vec<usize>>) -> Self {
        Planner { 
            workers, 
            workloads: vec![0; workers],
            tasks_prefix: tasks.iter().scan(0, |sum, i| {*sum += i.duration; Some(*sum)}).collect(),
            tasks, 
            curr_state: std::iter::repeat_with(Vec::new).take(workers).collect(),
            min_diff: !0usize,
            min_state: std::iter::repeat_with(Vec::new).take(workers).collect(),
            dependencies,
            tasks_end: HashMap::new(),
        }
    }

    pub fn execute(&mut self) -> Option<(Vec<Vec<Task>>, usize)> {
        self.plan(0);
        let tasks: Vec<Vec<Task>> = self.min_state.iter().map(|state| {
            state.iter().map(|idx| {
                self.tasks[*idx].clone()
            }).collect()
        }).collect();
        Some((tasks, self.min_diff))
    }

    fn plan(&mut self, task_idx: usize) {
        if task_idx == self.tasks.len() { 
            let diff = self.get_max_diff();
            if diff < self.min_diff { 
                self.min_diff = diff; 
                self.min_state = self.curr_state.clone();
            } 
            return;
        } 
        
        for i in 0..self.workers {
            let mut skip = false;
            if let Some(dependencies) = self.dependencies.get(&task_idx) {
                for dependency in dependencies {
                    let worker_end = &self.workloads[i];
                    let dependency_end = self.tasks_end.get(dependency).unwrap_or(&0);
                    if worker_end < dependency_end { skip = true; break; }
                }
            } 
            if self.can_prune(task_idx) || skip { continue; }
            
            let duration = self.tasks[task_idx].duration;
            self.workloads[i] += duration;
            self.tasks_end.insert(task_idx, self.workloads[i]);
            self.curr_state[i].push(task_idx);
            
            self.plan(task_idx + 1);
            
            self.workloads[i] -= duration;
            self.tasks_end.remove(&task_idx);
            self.curr_state[i].pop();
            
            if self.workloads[i] == 0 { break; }
        }
    }

    fn can_prune(&self, task_idx: usize) -> bool {
        let max = *self.workloads.iter().max().unwrap_or(&0);
        let min = *self.workloads.iter().min().unwrap_or(&0);
        
        let total_prefix = *self.tasks_prefix.last().unwrap_or(&0);
        let current_prefix = *self.tasks_prefix.get(task_idx).unwrap_or(&0);
        
        let remaining_tasks = total_prefix - current_prefix; 
        
        max > self.min_diff.saturating_add(min).saturating_add(remaining_tasks)
    }
    
    fn get_max_diff(&self) -> usize {
        let max = self.workloads.iter().max().unwrap_or(&0);
        let min = self.workloads.iter().min().unwrap_or(&0);
        return max - min;
    }
}
