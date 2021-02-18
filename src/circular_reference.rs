use std::cell::RefCell;
use std::rc::Rc;

// Circular reference: two objects refer one another
// Can be implemented using Rc RefCell, instead of lifetime ('a)
// Although, that sacrifices on compiler checks
// So, instead of having a many to many structure we can do something similar to database normalization
// Student, Course, Vec<Enrollment{student, course}>
pub fn run() {

    // Say, Student and Course have a many to many relationship
    struct Student {
        name: String,
        courses: Vec<Rc<RefCell<Course>>>
    }

    struct Course {
        name: String,
        students: Vec<Rc<RefCell<Student>>>
    }

    impl Student {
        fn new(name: &str) -> Student {
            Student {name: name.into(), courses: vec![] }
        }
    }
    impl Course {
        fn new(name: &str) -> Course {
            Course {name: name.into(), students: vec![] }
        }
        fn add(course:Rc<RefCell<Course>>, student:Rc<RefCell<Student>>) {
            student.borrow_mut().courses.push(course.clone());
            course.borrow_mut().students.push(student.clone());
            // Borrowing problem can solved using RefCell
        }
    }

    let john = Rc::new(RefCell::new(Student::new("John")));
    let course = Rc::new(RefCell::new(Course::new("Biology")));

    Course::add(course,john); // Lifetime issue can be solved using Rc

}