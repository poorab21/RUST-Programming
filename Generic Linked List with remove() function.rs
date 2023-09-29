struct Node<T: std::fmt::Display + Clone> {
    data : T ,
    next : Option<Box<Node<T>>>
}

struct Linked_List<T: std::fmt::Display + Clone> {
    head : Option<Box<Node<T>>>
}

impl<T: std::fmt::Display + Clone> Linked_List<T> {
    fn new() -> Self {
        Self {
            head: None
        }
    }
    
    fn push( &mut self , data : T ) {
        if let None = &self.head {
            self.head = Some(
                Box::new(
                    Node {
                        data ,
                        next : None
                    }
                )
            );
        }
        else {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if let None = node.next {
                    node.next = Some(
                        Box::new(
                            Node {
                                data ,
                                next : None
                            }
                        )
                    );
                    break;
                }
                
                current = &mut node.next;
            }
        }
    }
    
    fn display( &self ) {
        let mut current = &self.head;
        
        while let Some(node) = current {
            print!("{} ",node.data);
            current = &node.next;
        }
        println!("");
    }
    
    fn size( &self ) -> usize {
        let mut current = &self.head;
        let mut count: usize = 0;
        
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        
        count
    }
    
    fn remove( &mut self , index : u32 ) -> Option<T> {
        
        if self.size() < index as usize {
            None
        }
        else if index == 1 {
            let mut temp = self.head.as_mut().take();
            let val = temp.as_mut().unwrap().data.clone();
            self.head = temp.as_mut().unwrap().next.take();
            Some(val)
        }
        else {
            let mut current = &mut self.head;
            let mut count = 1;
            
            while count < index - 1 {
                current = &mut current.as_mut().unwrap().next;
                count += 1;
            }
            
            let mut current = current.as_mut().unwrap();
            let value = current.next.as_mut().unwrap().data.clone();
            current.next = current.next.as_mut().unwrap().next.take();
            Some(value)
        }
    }
}

fn main() {
    let mut l1 = Linked_List::new();
    l1.push(6);
    l1.push(17);
    l1.push(18);
    l1.push(28);
    l1.display();
    println!("{} removed",l1.remove(4).unwrap());
    println!("{} removed",l1.remove(1).unwrap());
    println!("{} removed",l1.remove(2).unwrap());
    println!("{} removed",l1.remove(1).unwrap());
    l1.display();
    l1.push(71);
    l1.push(9);
    l1.display();
    println!("{} removed",l1.remove(1).unwrap());
    println!("{} removed",l1.remove(1).unwrap());
    if let None = l1.remove(1) {
        println!("List is empty");
    }
    l1.push(98);
    l1.push(100);
    l1.display();
}