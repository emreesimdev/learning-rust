// Adding Debug to print structs easily later
#[derive(Debug)]

struct Student {
    name: String,
    grade: i32,
}

#[derive(Debug)]

struct Classroom {
    // This struct holds a list of Students
    students: Vec<Student>,
}

impl Classroom {
    // Constructor: Creates a new, empty classroom
    fn new() -> Classroom {
        Classroom {
            students: Vec::new(),
        }
    }

    // This function takes a name and a grade, creates a Student, and adds it to the list
    fn add_student(&mut self, name: String, grade: i32) {
        let new_student = Student {
            name: name,
            grade: grade,
        };

        // Pushing the new student into the vector
        self.students.push(new_student);
    }
}

fn main() {
    // We need 'mut' because we will add students to it
    let mut classroom = Classroom::new();

    // Adding some test students
    classroom.add_student(String::from("Emre"), 73);
    classroom.add_student(String::from("Hans"), 51);
    classroom.add_student(String::from("Lily"), 95);

    // Printing the whole class structure
    println!("Students and their grades: {:?}", classroom);
}