// Call Center: Imagine you have a call center with three levels of
// employees: respondent, manager, and director. An incoming telephone
// call must be first allocated to a respondent who is free. If the
// respondent can't handle the call, he or she must escalate the call
// to a manager. If the manager is not free or not able to handle it,
// then the call should be escalated to a director. Design the classes
// and data structures for this problem. Implement a method
// dispatchCall() which assigns a call to the first available
// employee.

use core::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Respondent,
    Manager,
    Director,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Employee {
    rank: Rank,
    name: String,
    is_available: bool,
}

impl Employee {
    fn set_busy(&mut self) {
        self.is_available = false;
    }
}

// Without this, Employee ordering would be determined by the order of
// the comparson of attributes in the struct, in whatever order they
// are defined.
impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

struct Call {
    from: String,
    minimum_rank: Rank,
}

struct CallCenter {
    employees: Vec<Employee>,
}

impl CallCenter {
    fn new(mut employees: Vec<Employee>) -> Self {
        employees.sort(); // sort from lowest to highest Rank.
        CallCenter { employees }
    }
    fn _find_available_employee_idx(&self, call: Call) -> Option<usize> {
        // Finds the first available Employee with Rank above or equal
        // to the Call's minimum_rank. Depens on self.employees being
        // sorted already!
        self.employees.iter().enumerate().find_map(|(i, e)| {
            if e.rank >= call.minimum_rank && e.is_available {
                Some(i)
            } else {
                None
            }
        })
    }

    fn dispatch_call(&mut self, call: Call) -> Option<&Employee> {
        if let Some(employee_idx) = self._find_available_employee_idx(call)
        // .or_else(|| self._find_available_employee_idx(Rank::Manager))
        // .or_else(|| self._find_available_employee_idx(Rank::Director))
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
fn test_rank_ordering() {
    assert_eq!(Rank::Manager > Rank::Director, false);
    assert_eq!(Rank::Manager > Rank::Respondent, true);
}

#[test]
fn test_employee_ordering() {
    // assert that Rank is the only determination for ordering, and
    // that swapping names does not affect ordering.
    assert_eq!(
        Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("joe")
        } > Employee {
            rank: Rank::Director,
            is_available: false,
            name: String::from("kyle")
        },
        false
    );
    assert_eq!(
        Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("kyle")
        } > Employee {
            rank: Rank::Director,
            is_available: false,
            name: String::from("joe")
        },
        false
    );
    assert_eq!(
        Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("kyle")
        } < Employee {
            rank: Rank::Director,
            is_available: false,
            name: String::from("joe")
        },
        true
    );

    assert_eq!(
        Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("joe")
        } < Employee {
            rank: Rank::Director,
            is_available: false,
            name: String::from("kyle")
        },
        true
    );
    // assert that 2 employess with the same rank are neither greater
    // than nor less than each other
    assert_eq!(
        Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("kyle")
        } < Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("joe")
        },
        false
    );
    assert_eq!(
        Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("kyle")
        } > Employee {
            rank: Rank::Manager,
            is_available: false,
            name: String::from("joe")
        },
        false
    );
}

#[test]
fn test_employees() {
    let mut employees = vec![Employee {
        rank: Rank::Respondent,
        is_available: true,
        name: String::from("joe"),
    }];
    employees.push(Employee {
        rank: Rank::Manager,
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
            rank: Rank::Director,
            is_available: false,
            name: String::from("Jane"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: false,
            name: String::from("Joe"),
        },
    ];
    let mut call_center = CallCenter { employees };
    assert_eq!(
        call_center.dispatch_call(Call {
            from: String::from("somebody"),
            minimum_rank: Rank::Respondent,
        }),
        None
    );
}

#[test]
fn test_call_center_single() {
    let employees = vec![Employee {
        rank: Rank::Director,
        is_available: true,
        name: String::from("Jane"),
    }];
    let mut call_center = CallCenter { employees };
    assert_eq!(
        call_center
            .dispatch_call(Call {
                from: String::from("someone"),
                minimum_rank: Rank::Respondent,
            })
            .unwrap()
            .name,
        String::from("Jane")
    );
}

#[test]
fn test_call_center_respondent() {
    let employees = vec![
        Employee {
            rank: Rank::Director,
            is_available: true,
            name: String::from("Diane Director"),
        },
        Employee {
            rank: Rank::Manager,
            is_available: true,
            name: String::from("Marty Manager"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: false,
            name: String::from("Joe"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: true,
            name: String::from("Jane"),
        },
    ];
    let mut call_center = CallCenter::new(employees);
    assert_eq!(
        call_center
            .dispatch_call(Call {
                from: String::from("Luke"),
                minimum_rank: Rank::Respondent
            })
            .unwrap()
            .name,
        String::from("Jane")
    );
}

#[test]
fn test_call_center_director() {
    let employees = vec![
        Employee {
            rank: Rank::Director,
            is_available: true,
            name: String::from("Diane Director"),
        },
        Employee {
            rank: Rank::Manager,
            is_available: true,
            name: String::from("Marty Manager"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: false,
            name: String::from("Joe"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: true,
            name: String::from("Jane"),
        },
    ];
    let mut call_center = CallCenter::new(employees);
    assert_eq!(
        call_center
            .dispatch_call(Call {
                from: String::from("Obama"),
                minimum_rank: Rank::Director
            })
            .unwrap()
            .name,
        String::from("Diane Director")
    );
}

#[test]
fn test_call_center_manager() {
    let employees = vec![
        Employee {
            rank: Rank::Director,
            is_available: true,
            name: String::from("Diane Director"),
        },
        Employee {
            rank: Rank::Manager,
            is_available: true,
            name: String::from("Marty Manager"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: false,
            name: String::from("Joe"),
        },
        Employee {
            rank: Rank::Respondent,
            is_available: true,
            name: String::from("Jane"),
        },
    ];
    let mut call_center = CallCenter::new(employees);
    assert_eq!(
        call_center
            .dispatch_call(Call {
                from: String::from("Middle Manager"),
                minimum_rank: Rank::Manager
            })
            .unwrap()
            .name,
        String::from("Marty Manager")
    );
}
fn main() {
    let mut call_center = CallCenter::new(vec![Employee {
        rank: Rank::Director,
        is_available: false,
        name: String::from("Mr. Director"),
    }]);
    call_center.dispatch_call(Call {
        from: String::from("Obama"),
        minimum_rank: Rank::Respondent,
    });
}
