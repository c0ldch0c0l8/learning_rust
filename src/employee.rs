pub struct Employee {
    name: String,
    department: Department
}

#[derive(Debug)]
pub enum Department {
    Sales,
    IT,
    CustomerService
}

impl Employee {
    pub fn new(name: &str, department: Department) -> Employee {
        Employee {
            name: String::from(name),
            department
        }
    }
}

pub fn quick_sort_alphabetically(employees: &Vec<Employee>) {
    if employees.len() < 2 {
        return;
    } else {
        // sort vec lol, and then out of this func do some loop input loop
    }
}