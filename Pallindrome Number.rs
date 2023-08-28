use std::io;

fn main ()
{
  let mut input = String::new();
  println!("Enter Number:");
  io::stdin ().read_line (&mut input).expect ("Something went wrong");

  let mut input : i32 = input.trim ().parse ().unwrap ();
  if isPallindrome(input) == true {
      println ! ("{} is a Pallindrome number", input);
    }
  else {
      println ! ("{} is not a Pallindrome number", input);
    }
}

fn isPallindrome (num:i32) -> bool {
  let mut newNum : i32 = 0;
  let mut remainder : i32 = 0;
  let mut sub : i32 = num;

  while sub > 0 {
      remainder = sub % 10;
      newNum = (newNum * 10) + remainder;
      sub /= 10;
    }

  if newNum == num {
      return true;
    }
  else {
      return false;
    }
}
