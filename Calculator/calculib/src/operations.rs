use num::{ Num , Zero };

pub fn add<T: std::ops::Add<Output = T> + Num + Into<f64>>( num1 : T , num2 : T ) -> f64 {
    let num1 : f64 = num1.into();
    let num2 : f64 = num2.into();
    
    (num1 as f64) + (num2 as f64)
}

pub fn subtract<T: std::ops::Sub<Output = T> + Num + Into<f64>>( num1 : T , num2 : T ) -> f64 {
    let num1 : f64 = num1.into();
    let num2 : f64 = num2.into();

    (num1 as f64) - (num2 as f64)
}

pub fn divide<T: std::ops::Div<Output = T > + Num + Into<f64>>( num1 : T , num2 : T ) -> Result< f64 , Box< dyn std::error::Error> > {
    let num1 : f64 = num1.into();
    let num2 : f64 = num2.into();

    if num2 == f64::zero() { return Err("Division by Zero error".into()); }

    Ok((num1 as f64) / (num2 as f64))
}

pub fn multiply<T: std::ops::Mul<Output = T> + Num + Into<f64>>( num1 : T , num2 : T ) -> f64 {
    let num1 : f64 = num1.into();
    let num2 : f64 = num2.into();
    
    (num1 as f64) * (num2 as f64)
}

pub fn modulo( num1 : u32 , num2 : u32 ) -> u32 {
    num1 % num2
}

#[ cfg ( test ) ]
mod tests {

    use super::*;

    #[test]
    fn check_modulo() {
        let num1 = 33;
        let num2 = 10;
        let modulo = modulo( num1 , num2 );

        assert_eq!( modulo , 3 );
    }

    #[test]
    fn check_product() -> Result< () , Box< dyn std::error::Error > > {
        let num1 = 1.45;
        let num2 = 6.6;
        let product = format!("{:.2}",multiply( num1 , num2 )).trim().parse::<f32>().unwrap();

        if product == 9.57 { Ok(()) }
        else { Err(format!("multiply() incorrectly determined {} as product of {} and {}",product,num1,num2).into()) }
    }

    #[test]
    fn check_sum() {
        let num1 = 54.41;
        let num2 = 24.45;
        let sum = format!("{:.2}",add( num1 , num2 )).trim().parse::<f32>().unwrap();

        assert_eq!( sum , 78.86 , " add() incorrectly determined {sum} as the sum of {num1} and {num2} ");
    }

    #[test]
    fn check_quotient() {
        let num1 = 381;
        let num2 = 112;
        let quotient = format!("{:.2}",divide( num1 , num2 ).unwrap()).trim().parse::<f32>().unwrap();

        assert_eq!( quotient , 3.4 , "quotient() incorrectly determined {quotient} as the quotient when {num1} gets divided by {num2}", );
    }

    #[test]
    #[should_panic( expected = "Division by Zero" )]
    fn check_divide_by_zero() {
        let num1 = 55;
        let num2 = 0;
        
        divide( num1 , num2 ).unwrap();
    }

    #[test]
    fn check_difference() {
        let num1 = 45.13;
        let num2 = 12.21;
        let difference = format!("{:.2}",subtract( num1 , num2 )).trim().parse::<f32>().unwrap();

        assert_eq!( difference , 32.92 , "subtract() incorrectly determined {difference} as the difference of {num1} and {num2}" ); 
    }
}