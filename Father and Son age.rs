use std::io;

fn main(){
   let mut father_age = String::new();
   let mut son_age = String::new();

   println!("Enter Father's age:");
   io::stdin().read_line(&mut father_age).expect("Something went wrong while processing input for father's age");
   father_age.pop();

   println!("Enter Son's age:");
   io::stdin().read_line(&mut son_age).expect("Something went wrong while processing input for son's age");
   son_age.pop();

   let father_age : u32 = father_age.trim().parse().unwrap();
   let son_age : u32 = son_age.trim().parse().unwrap();
   
   if son_age == father_age { println!("father and son cannot have the same age"); return; }

   if son_age * 2 == father_age { println!("0"); return; }

   for i in 1..son_age {
	if i * 2 == ( father_age - son_age + i ) { println!("{0} years ago",son_age-i); return; } 
   }

   let mut sub_age1 = father_age;
   let mut sub_age2 = son_age;

   while sub_age2 * 2 != sub_age1 {
	sub_age2 += 1;
	sub_age1 += 1;
   }

   println!("{} years from now",sub_age1 - father_age);
}