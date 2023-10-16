use calculib::operations::{ multiply , divide , add , subtract , modulo };

#[test]
fn operation_selection() {
    let operator = '*';
    let operand1 = 44 ;
    let operand2 = 34 ;

    if operator == '+' {
        println!("Sum of {operand1} and {operand2} = {}", add(operand1,operand2));
    }
    else if operator == '-' {
        println!("Difference of {operand1} and {operand2} = {}",subtract(operand1,operand2));
    }
    else if operator == '/' {
        println!("Quotient of {operand1} divided by {operand2} = {}",divide(operand1,operand2).unwrap());
    }
    else if operator == '%' {
        println!("Modulo of {operand1} and {operand2} = {}",modulo(operand1,operand2));    
    }
    else if operator == '*' {
        println!("Product of {operand1} and {operand2} = {}",multiply(operand1,operand2))
    }
    else { panic!("Enter valid operator"); }
}