#[derive(Debug)]
pub enum Status {
    InProgress ,
    Cancelled ,
    Completed
}

#[derive(Debug)]
pub struct Task {
    id : u32 , 
    title : String ,
    description : String ,
    status : Status 
}

impl Task {
    pub fn new( id : u32 , title : String , description : String , status : Status ) -> Self {
        Self {
            id , 
            title ,
            description ,
            status 
        }
    }

    pub fn set_status( &mut self , status : Status ) {
        self.status = status;
    }

    pub fn set_title( &mut self , title : String ) {
        self.title = title;
    }

    pub fn set_description( &mut self , description : String ) {
        self.description = description;
    }

    pub fn title( &self ) -> &str {
        &self.title
    }

    pub fn description( &self ) -> &str {
        &self.description
    }

    pub fn status( &self ) -> &Status {
        &self.status
    }

    pub fn id( &self ) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_task_creation() {
        let task = Task::new( 
            1 , 
            "Theatre Visitation".to_string() , 
            "Go watch the new movie tonight".to_string() , 
            Status::InProgress 
        );

        match ( task.title() , task.description() , task.status() ) {
            ( "Theatre Visitation" , "Go watch the new movie tonight" , Status::InProgress ) => {
                assert!( true )
            },
            ( _ , _ , _ ) => { assert!(false) }
        }
    }

    #[test]
    fn check_title_update() {
        let mut task = Task::new(
            1 ,
            "Task Title".to_string() ,
            "Task Description".to_string() ,
            Status::InProgress
         );

         task.set_title("Electronic Device Purchase".to_string());

         assert_eq!( task.title() , "Electronic Device Purchase" );
    }

    #[test]
    fn check_description_update() {
        let mut task = Task::new(
            1 ,
            "Task Title".to_string() ,
            "Task Description".to_string() ,
            Status::InProgress
        );

        task.set_description("Go to computer shop and buy new apple computer".to_string());

        assert_eq!( task.description() , "Go to computer shop and buy new apple computer" );
    }

    #[test]
    fn check_status_update() -> Result< () , Box< dyn std::error::Error> > {
        let mut task = Task::new(
            1 ,
            "Task Title".to_string() ,
            "Task Description".to_string() ,
            Status::InProgress
        );

        task.set_status( Status::Cancelled );

        match task.status() {
            Status::Cancelled => Ok(()) ,
            _ => Err("set_status not updating status properly".into())
        }
    }
}