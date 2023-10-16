use
  std::io;

fn
main ()
{
  let mut
    value = String::new ();
  let mut
    Fahrenheit = String::new ();
  let mut
    Celsius = String::new ();

  println ! ("Choose Conversion");
  println ! ("1) Celsius to Fahrenheit");
  println ! ("2) Fahrenheit to Celsius");
  println ! ("What do you choose ?");
  io::stdin ().read_line (&mut value).expect ("Something went wrong");

  let
    choice:
    i32 = value.trim ().parse ().unwrap ();

  if choice
    == 1
    {
      io::stdin ().read_line (&mut Celsius).
	expect ("Something went wrong when inputting celsius");
      let
	val:
	f32 = Celsius.trim ().parse ().unwrap ();
    println ! ("Fahrenheit = {}", (val * (9f 32 / 5f 32) + 32f 32))}
  else if choice
    == 2
    {
      io::stdin ().read_line (&mut Fahrenheit).
	expect ("Something went wrong when inputting fahrenheit");
      let
	val:
	f32 = Fahrenheit.trim ().parse ().unwrap ();
    println ! ("Celsius = {}", (val - 32f 32) * (5f 32 / 9f 32))}

}