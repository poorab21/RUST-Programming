use crate::task::{ Task , Status };

#[derive(Debug)]
pub struct List {
    storage : Vec<Task>
}

impl List {
    pub fn new() -> Self {
        Self {
            storage : Vec::new()
        }
    }

    pub fn push( &mut self , task : Task ) {
        self.storage.push(task);
    }

    pub fn pop( &mut self ) -> Task {
        self.storage.pop().expect("Called pop() function on an empty List")
    }

    pub fn insert( &mut self , index : usize , task : Task ) {
        if self.storage.len() - 1 >= index {
            self.storage.insert( index , task )
        }
        else {
            panic!("Index out of bounds");
        }
    }

    pub fn remove( &mut self , index : usize ) -> Task {
        if self.storage.len() - 1 >= index {
            self.storage.remove(index)
        }
        else {
            panic!("Index out of bounds")
        }
    }

    pub fn remove_with_id( &mut self , id : u32 ) -> Result<Task , Box<dyn std::error::Error>> {
        let mut index : i32 = -1;

        for i in 0..self.storage.len() {
            if self.storage.get(i).unwrap().id() == id { index = i as i32; break; }
        }

        if index == -1 {  return Err("Task with specified ID does not exist".into()) }
        Ok(self.storage.remove(index as usize))
    }

    pub fn tasks( &self ) -> &Vec<Task> {
        &self.storage
    }

    pub fn size( &self ) -> usize {
        self.storage.len()
    }

    pub fn get( &self , index : usize ) -> Option<&Task> {
        self.storage.get( index )
    }
}