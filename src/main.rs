use std::io;

fn main() {    
    loop {
        println!("Enter if the type of your measure (C: Celsius, F: fahrenheit):");

        let mut type_of_measure = String::new();
    
        io::stdin().read_line(&mut type_of_measure).expect("It is not a valid character");
    
        let chosed_type : String = match type_of_measure.trim().parse() { //the parse method returns a Result type
            Ok(num) => num,
            Err(_) => continue, //the underscores means that we want to match all error types
        };
        if validate_correct_measure(&chosed_type) {
            let m_type = if chosed_type == "C"{
                "Celsius to fahrenheit"                
            }
            else {
                "fahrenheit to Celsius"
            } ;
            println!("You chose {}", m_type);            
            break;       
        }
        else {
            println!("Please enter C or F");   
            continue;         
        }        
    }

    loop {
        let mut m_input = String::new();
        println!("Please, enter a value to convert");
        io::stdin().read_line(&mut m_input).expect("It is not a valid value.");
        let chosed_value:f32 = match m_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        let result:f32 = calculate_fahrenheit_2_celsius(chosed_value);
        println!("Result: {}", result);
        break;
    }
}

fn validate_correct_measure(measure:&String) -> bool{
    measure.eq("C") || measure.eq("F")
}

fn calculate_fahrenheit_2_celsius(m_value:f32) -> f32{
    return ((m_value - 32) *5) /9;
}