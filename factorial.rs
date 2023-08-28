use
  std::io;

fn
main ()
{
  let mut num: String = "".to_string ();
  let mut factorial: i32 = 1;
  println ! ("Enter Number:");
  io::stdin ().read_line (&mut num).expect ("Something went wrong");

  let mut num: i32 = num.trim ().parse ().unwrap ();
  
  for i in 1..=num {
      factorial *= i;
  }

  println ! ("Factorial = {}", factorial);
}
