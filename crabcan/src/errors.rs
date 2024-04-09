use std::fmt;
use std::process::exit;

// Allows to display a variant with the format {:?}
#[derive(Debug)]
// Containers all possible errors in our tool
pub enum Errcode {
    ArgumentInvalid(&'static str),
    NotSupported(u8),
    ContainerError(u8),
}

impl Errcode {
    // Translate an Errcode::X into a number to return (the Unix way)
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be considered an error
    }
}

#[allow(unreachable_patterns)]
// trait Display, allows Errorcode enum to be display by:
//      println!("{}", error);
// in this case, it calls the function "fmt", which is implemented below
impl fmt::Display for Errcode {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behavior to have for each variant
        match &self {
            _ => write!(f, "{:?}", self)
        }
    }

}

// Get the result from a function, and exist the process with corresponding error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }, 
        
        // If there's an error, print an error message and return the corresponding error code
        Err(e) =>  {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);        
        }
    }
}