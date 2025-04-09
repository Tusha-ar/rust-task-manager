use std::{cell::{Cell, RefCell}, rc::Rc};


struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct TaskManager {
    tasks: Rc<RefCell<Vec<Task>>>,
    history: RefCell<Vec<Box<dyn FnOnce()>>>, // to undo actions
    next_id: Cell<u32>
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { tasks: 
            Rc::new(RefCell::new(vec![])), history: RefCell::new(vec![]),
            next_id: Cell::new(0)
        }
    }
    fn add_task(&self, title: String) {
        let id = self.next_id.get() + 1;
        self.next_id.set(id);
        self.tasks.borrow_mut().push(Task {
            done: false,
            title: title,
            id: self.next_id.get()
        });

        let task_ref = self.tasks.clone();
        self.history.borrow_mut().push(Box::new(move || {
            task_ref.borrow_mut().retain(|t| t.id != id);
        }));
    }
    fn mark_done(&self, id: u32) {
        let mut tasks = self.tasks.borrow_mut();
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            let prev_done = task.done;
            task.done = true;

            let task_ref = self.tasks.clone();
            self.history.borrow_mut().push(Box::new(move || {
                if let Some(t) = task_ref.borrow_mut().iter_mut().find(|t| t.id == id) {
                    t.done = prev_done;
                }
            }));
        } 
    }
    fn list_tasks(&self) {
        println!("##### ALL TASKS ######");
        for task in self.tasks.borrow().iter() {
            println!(
                "[{}] {} (ID: {})",
                if task.done { "✔" } else { " " },
                task.title,
                task.id
            );
        }
        println!("######################");
    }
    fn delete_task(&self, id:u32) {
        let mut tasks = self.tasks.borrow_mut();
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            let deleted_task = tasks.remove(pos);
            let task_ref = self.tasks.clone();
            self.history.borrow_mut().push(Box::new(move || {
                task_ref.borrow_mut().push(deleted_task);
            }));
        }
    }
    fn undo(&self) {
        if let Some(undo_action) = self.history.borrow_mut().pop() {
            undo_action()
        }
    }
}





fn main() {
    let t = TaskManager::new();
    t.add_task(String::from("Go to the gym"));
    t.add_task(String::from("Wash clothes"));
    t.add_task(String::from("work"));
    t.mark_done(1);
    t.list_tasks();
    t.delete_task(1);
    t.add_task(String::from("Sample Task"));
    t.list_tasks();
    t.undo();
    t.list_tasks();
}
