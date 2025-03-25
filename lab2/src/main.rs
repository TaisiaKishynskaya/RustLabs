fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            transposed[i][j] = matrix[j][i];
        }
    }
    transposed
}

fn print_matrix(matrix: [[i32; 3]; 3]) {
    for row in matrix.iter() {
        for &elem in row.iter() {
            print!("{:4}", elem);
        }
        println!()
    }
}

//--------------------------------------------------------------------------

struct Phone {
    name: String,
    diagonal: u32,
    processor: u32,
    ram: f32,
}

impl Phone {
    fn get_ram(&self) -> f32 {
        self.ram
    }
    fn set_diagonal(&mut self, new_diagonal: u32) {
        self.diagonal = new_diagonal;
    }
    fn increase_processor_speed(&mut self, amount: u32) {
        self.processor += amount;
    }
    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn print_characteristics(&self) {
        println!(
            "Name: {}\nDiagonal: {}\nProcessor: {}\nRAM: {}",
            self.name, self.diagonal, self.processor, self.ram
        );
    }
}

//--------------------------------------------------------------------------

fn main() {
    println!("Task 1");

    let matrix = [[101, 102, 103],
                            [201, 202, 203],
                            [301, 302, 303]];

    println!("Initial matrix:");
    print_matrix(matrix);

    println!("Transposed matrix");
    print_matrix(transpose(matrix));

    println!("\nTask 2. Variant 7");
    let mut phone = Phone {
        name: String::from("Samsung"),
        diagonal: 6,
        processor: 1800,
        ram: 8.0,
    };

    phone.print_characteristics();

    println!("RAM size: {} GB", phone.get_ram());

    phone.set_diagonal(7);
    println!("New diagonal: {} inches", phone.diagonal);

    let amount = 200;
    phone.increase_processor_speed(amount);
    println!("New processor speed: {} MHz", phone.processor);

    phone.set_name(String::from("Samsung Ultra"));
    phone.print_characteristics();

    println!("\nTask 3");
    let mut phones = [
        Phone {
            name: String::from("Samsung"),
            diagonal: 6,
            processor: 2500,
            ram: 12.0,
        },
        Phone {
            name: String::from("Apple"),
            diagonal: 6,
            processor: 3000,
            ram: 16.0,
        },
        Phone {
            name: String::from("Realme"),
            diagonal: 5,
            processor: 2200,
            ram: 8.0,
        },
    ];

    phones.sort_by_key(|phone| phone.diagonal);

    for phone in &phones {
        phone.print_characteristics();
        println!();
    }
}
