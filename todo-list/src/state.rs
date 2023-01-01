use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const TODOLIST: Item<TodoList> = Item::new("todolist");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    pub owner: Addr,
}

impl TodoList {
    pub fn init(addr: Addr, name: String, description: String) -> TodoList {
        let mut tasks = Vec::new();
        let task = Task::new(name, description);
        tasks.push(task);
        TodoList { tasks, owner: addr }
    }

    pub fn add_task(&mut self, name: String, description: String) {
        let task = Task::new(name, description);

        self.tasks.push(task)
    }

    pub fn delete_task(&mut self, name: String) {
        if let Some(pos) = self.tasks.iter().position(|x| *x.name == name) {
            self.tasks.remove(pos);
        }
    }

    pub fn reset(&mut self) {
        self.tasks.clear();
    }

    pub fn update_task(&mut self, name: String, description: String) {
        if let Some(pos) = self.tasks.iter().position(|x| *x.name == name) {
            self.tasks[pos].update(name, description);
        }
    }

    pub fn completed_task(&mut self, name: String, completed: bool) {
        if let Some(pos) = self.tasks.iter().position(|x| *x.name == name) {
            self.tasks[pos].done(completed);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Task {
    name: String,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            completed: false,
        }
    }

    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    pub fn done(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
