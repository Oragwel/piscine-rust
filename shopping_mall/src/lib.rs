pub mod mall;

pub use mall::{Mall, Guard, Floor, Store, Employee};

use std::collections::HashMap;


pub fn biggest_store(mall: &Mall) -> Option<(&String, &Store)> {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<&Employee> {
    let all_employees: Vec<&Employee> = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.values())
        .collect();

    if let Some(max_salary) = all_employees.iter().map(|e| e.salary).max_by(|a, b| a.partial_cmp(b).unwrap()) {
        all_employees
            .into_iter()
            .filter(|e| e.salary == max_salary)
            .collect()
    } else {
        vec![]
    }
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employee_count = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum::<usize>();
    employee_count + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, mut guard_pool: HashMap<String, Guard>) {
    let total_area: u64 = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_area as f64 / 200.0).ceil() as usize;
    let current_guards = mall.guards.len();

    let guards_needed = required_guards.saturating_sub(current_guards);

    for _ in 0..guards_needed {
        if let Some((name, guard)) = guard_pool.iter().next().map(|(k, v)| (k.clone(), v.clone())) {
            mall.hire_guard(name.clone(), guard);
            guard_pool.remove(&name);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for store in mall.floors
        .values_mut()
        .flat_map(|floor| floor.stores.values_mut()) {
        for employee in store.employees.values_mut() {
            let hours = employee.working_hours.1 - employee.working_hours.0;
            if hours >= 10 {
                employee.salary *= 1.10;
            } else {
                employee.salary *= 0.90;
            }
        }
    }
}
