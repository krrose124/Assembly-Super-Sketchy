use std::collections::HashMap;

static nametable: Vec<String> = Vec::new();
struct Tables {


intmap: HashMap<String, i32>,
stringmap: HashMap<String, Vec<char>>,
arraymap: HashMap<String, Vec<i32>>,
}

pub struct ErrorMessage {
    pub message: String,
    pub isError: bool,
}


fn check_names(name: &String) -> bool {
    for each in nametable.iter(){
        if String::from(each) == String::from(name){
            return true;
        }
        else{ return false; }
    }
    return true;
}
fn var_error_builder(error: bool) -> ErrorMessage{
    let builder = ErrorMessage{message:"Var name is already in use".to_string(), isError:error};
    return builder;
}

impl Tables{
    fn init(&mut self){
        self.intmap = HashMap::new();
        self.stringmap = HashMap::new();
        self.arraymap = HashMap::new();
    }
    fn add_array(&mut self, name: String, array: Vec<i32>) -> ErrorMessage {
    if check_names(&name){
        return var_error_builder(true);
    }
    self.arraymap.insert(name, array);
    return var_error_builder(false);
    }
    fn add_str(&mut self, name: String, array: Vec<char>) -> ErrorMessage {
    if check_names(&name){
        return var_error_builder(true);
    }
    self.stringmap.insert(name, array);
    return var_error_builder(false);
    }
    fn add_int(&mut self, name: String, integer: i32) -> ErrorMessage {
    if check_names(&name){
        return var_error_builder(true);
    }
    self.intmap.insert(name, integer);
    return var_error_builder(false);
    }
}
