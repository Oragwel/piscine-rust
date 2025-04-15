#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(mut current_worker) = self.grade.take() {
            self.grade = current_worker.next.take();
            Some(current_worker.name)
        } else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade
            .as_ref()
            .map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}
