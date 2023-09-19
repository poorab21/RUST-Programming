use std::io;

struct LinkedList {
    head : Option<Box<Node>>
}

struct Node {
    name : String , 
    age : i32 ,
    next : Option<Box<Node>>
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head : None
        }
    }
    fn insert( &mut self, name : String , age : i32 ) {
        if let None = self.head {
            self.head = Some(Box::new(Node {
                name, 
                age ,
                next : None
            }));
            return;
        }
        
        let mut current = &mut self.head;
        
        while let Some(node) = current {
            if let None = node.next { 
                node.next = Some(Box::new(Node {
            name ,
            age , 
            next : None
        })); 
                break; 
            }
            current = &mut node.next;
        }
    }
    fn display(&self) {
        let mut current = &self.head;
        
        while let Some(node) = current {
            println!("Name = {} , age = {}",node.name,node.age);
            current = &node.next;
        }
    }
}

fn main() {
    let mut linked_list1 = LinkedList::new();
    linked_list1.insert(String::from("john"),6);
    linked_list1.insert(String::from("Ajay"),9);
    linked_list1.insert(String::from("Poorab Gangwani"),12);
    linked_list1.display();
}