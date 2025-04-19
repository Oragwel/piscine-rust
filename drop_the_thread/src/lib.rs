use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        self.states.borrow_mut().push(false);
        (pid, Thread::new_thread(pid, c, self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{id} is already dropped");
        } else {
            states[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread {
    pub pid: usize,
    pub cmd: String,
    pub parent: *const Workers, // raw pointer because Rc doesn't play nicely with lifetimes
}

impl Thread {
    pub fn new_thread(p: usize, c: String, t: &Workers) -> Thread {
        Thread {
            pid: p,
            cmd: c,
            parent: t as *const Workers,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            if let Some(parent) = self.parent.as_ref() {
                parent.add_drop(self.pid);
            }
        }
    }
}
