fn main() {
    for i in (0..100).rev() {
        if i > 1 {
            println!("{i} bottles of beer on the wall, {i} bottles of beer\nTake one down and pass it around, {} bottles of beer on the wall",i-1);
        }
        else if i == 1 {
            println!("1 bottle of beer on the wall, 1 bottle of beer.\nTake one down and pass it around, no more bottles of beer on the wall");
        }
        else if i == 0 {
            println!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.")
        }
    }
}