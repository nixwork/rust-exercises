use std::collections::HashMap;

// 8.3 exercise
fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    println!("Type a command (e.g. \"Add Sally to Engineering\")");
    println!("Type \"list\" to print all the staff and exit");

    loop {
        let mut command = String::new();

        std::io::stdin().read_line(&mut command).expect("Read failed");

        if command.trim_end() == "list" {
            break;
        }

        let words: Vec<_> = command.split_whitespace().collect();
        let department = String::from(words[3]);
        let name = String::from(words[1]);
        if words[0] == "Add" && words[2] == "to" {
            println!("Adding {} to {}", name, department);
            let names = company.entry(department).or_insert(Vec::new());
            names.push(name);
        }
        else {
            println!("Invalid command");
        }
    }

    for (department, staff) in &company {
        println!("{} - {:?}", department, staff);
    }
}
