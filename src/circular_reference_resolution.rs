pub fn run() {
    struct Student {
        name: String
    }

    struct Course {
        name: String
    }

    struct Enrollment<'a> {
        student: &'a Student,
        course: &'a Course
    }

    struct Platform<'a> {
        enrollments: Vec<Enrollment<'a>>
    }

    impl Student {
        fn new(name: &str) -> Student {
            Student{name: name.into()}
        }
        fn courses(&self, platform: Platform) -> Vec<String> {
            platform.enrollments.iter()
                .filter(|&e| e.student.name == self.name)
                .map(|e| e.course.name.clone())
                .collect()
        }
    }
    impl Course {
        fn new(name: &str) -> Course {
            Course{name: name.into()}
        }
    }
    impl<'a> Enrollment<'a> {
        fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
            Enrollment {student, course}
        }
    }

    impl<'a> Platform<'a> {
        fn new() -> Platform<'a> {
            Platform { enrollments: Vec:: new() }
        }
        fn enroll(&mut self, student: &'a Student, course: &'a Course) {
            self.enrollments.push(Enrollment::new(student, course))
        }
    }

    let john = Student::new("John");
    let jack = Student::new("Jack");
    let math = Course::new("Math");
    let biology = Course::new("Biology");

    let mut platform = Platform::new();
    platform.enroll(&john, &biology);
    platform.enroll(&jack, &math);
    platform.enroll(&john, &math);

    for enrollment in platform.enrollments.iter() {
        println!("{} {}", enrollment.student.name, enrollment.course.name);
    }

    for course in john.courses(platform).iter() {
        println!("{}", course)
    }
}