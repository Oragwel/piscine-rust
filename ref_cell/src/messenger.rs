// ref_cell/src/messenger.rs

use std::cell::RefCell;
use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Tracker { logger, max }
    }

    pub fn peek(&self, value: &Rc<RefCell<usize>>) {
    let count = Rc::strong_count(value);
    let percentage = ((count as f64 / self.max as f64) * 100.0).round();
    self.logger.info(&format!(
        "you are using up to {:.0}% of your quota",
        percentage
    ));
}

pub fn set_value(&self, value: &Rc<RefCell<usize>>) {
    let count = Rc::strong_count(value);
    let percentage = ((count as f64 / self.max as f64) * 100.0).round();

    if percentage >= 100.0 {
        self.logger.error("you are over your quota!");
    } else if percentage >= 70.0 {
        self.logger.warning(&format!(
            "you have used up over {:.0}% of your quota! Proceeds with precaution",
            percentage
        ));
    }
}
}

