// Call Center: Imagine you have a call center with three levels of
// employees: respondent, manager, and director. An incoming telephone
// call must be first allocated to a respondent who is free. If the
// respondent can't handle the call, he or she must escalate the call
// to a manager. If the manager is not free or not able to handle it,
// then the call should be escalated to a director. Design the classes
// and data structures for this problem. Implement a method
// dispatchCall() which assigns a call to the first available
// employee.

#[derive(Debug, PartialEq, Eq)]
enum Role {
    Respondent,
    Manager,
    Director,
}

#[derive(Debug, PartialEq, Eq)]
struct Employee {
    is_available: bool,
    role: Role,
    name: String,
}

impl Employee {
    fn set_busy(&mut self) {
        self.is_available = false;
    }
}

struct CallCenter {
    employees: Vec<Employee>,
}

impl CallCenter {
    fn _find_available_employee_idx(&self, role: Role) -> Option<usize> {
        self.employees.iter().enumerate().find_map(|(i, e)| {
            if e.role == role && e.is_available {
                Some(i)
            } else {
                None
            }
        })
    }

    fn dispatch_call(&mut self) -> Option<&Employee> {
        if let Some(employee_idx) = self
            ._find_available_employee_idx(Role::Respondent)
            .or_else(|| self._find_available_employee_idx(Role::Manager))
            .or_else(|| self._find_available_employee_idx(Role::Director))
        {
            let employee = &mut self.employees[employee_idx];
            employee.set_busy();
            Some(&*employee)
        } else {
            None
        }
    }
}

#[test]
fn test_employees() {
    let mut employees = vec![Employee {
        role: Role::Respondent,
        is_available: true,
        name: String::from("joe"),
    }];
    employees.push(Employee {
        role: Role::Manager,
        is_available: false,
        name: String::from("manager Bill"),
    });
    assert_eq!(
        employees.iter().find(|e| e.name == "joe"),
        Some(&employees[0])
    );
}

#[test]
fn test_call_center_none() {
    let employees = vec![
        Employee {
            role: Role::Director,
            is_available: false,
            name: String::from("Jane"),
        },
        Employee {
            role: Role::Respondent,
            is_available: false,
            name: String::from("Joe"),
        },
    ];
    let mut call_center = CallCenter { employees };
    assert_eq!(call_center.dispatch_call(), None);
}

#[test]
fn test_call_center_single() {
    let employees = vec![Employee {
        role: Role::Director,
        is_available: true,
        name: String::from("Jane"),
    }];
    let mut call_center = CallCenter { employees };
    assert_eq!(
        call_center.dispatch_call().unwrap().name,
        String::from("Jane")
    );
}

#[test]
fn test_call_center_multiple() {
    let employees = vec![
        Employee {
            role: Role::Director,
            is_available: true,
            name: String::from("Jane"),
        },
        Employee {
            role: Role::Respondent,
            is_available: true,
            name: String::from("Jerry"),
        },
    ];
    let mut call_center = CallCenter { employees };
    assert_eq!(
        call_center.dispatch_call().unwrap().name,
        String::from("Jerry")
    );
}
fn main() {
    let mut call_center = CallCenter {
        employees: vec![Employee {
            role: Role::Director,
            is_available: false,
            name: String::from("Mr. Director"),
        }],
    };
    call_center.dispatch_call();
}
