use std::io;

fn main () {
  let mut value = String::new ();
  let mut twinPrime = 0;

  println ! ("Enter Number:");
  io::stdin ().read_line (&mut value).expect ("Something went wrong when taking input");
  value.pop ();

  let value : i32 = value.trim ().parse ().unwrap ();

  for i in 2..=value {
      if isPrime(i) && isPrime (i - 2) && isPrime (i + 2) {
	  twinPrime = setTwinPrime (i + 2, twinPrime, value);
	}
      else if isPrime(i) && !isPrime (i - 2) && isPrime (i + 2) {
	  twinPrime = setTwinPrime (i + 2, twinPrime, value);
	}
      else if isPrime(i) && isPrime (i - 2) && !isPrime (i + 2) {
	  twinPrime = setTwinPrime (i, twinPrime, value);
	}
    }
    
    if twinPrime == 0 {
        println!("Twin Prime = None");
    }
    else {
        println!("Twin Prime = {}",twinPrime);
    }
}

fn isPrime (number:i32) -> bool {
  if number == 1 || number == 0 { return false } 
  
  for i in 2..number {
    if number % i == 0 {
	  return false;
	}
  }
  return true;
}

fn setTwinPrime (prime:i32, currentPrime:i32, number:i32) -> i32 {
  if prime > currentPrime && prime <= number {
      return prime;
  }
  else {
      return currentPrime;
  }
}
