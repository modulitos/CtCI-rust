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
    fn new(role: Role, is_available: bool, name: String) -> Self {
        Employee {
            is_available,
            role,
            name,
        }
    }
}

struct CallCenter {
    employees: Vec<Employee>,
}

impl CallCenter {
    fn dispatch_call() {}
}

// type Call = ();

// fn dispatch_call(call: Call) {}

#[test]
fn test_employees() {
    let mut employees = vec![Employee::new(Role::Respondent, true, String::from("joe"))];
    employees.push(Employee::new(
        Role::Manager,
        false,
        String::from("manager Bill"),
    ));
    assert_eq!(
        employees.iter().find(|e| e.name == "joe"),
        Some(&employees[0])
    );
}
