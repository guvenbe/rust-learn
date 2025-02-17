#[derive(Copy, Clone, Debug)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl  ProtectedLocation{
    fn required_access_level(&self) ->u16{
       match self {
           Self::All =>1000,
           Self::Office => 800,
           Self::Warehouse => 500,
       } 
    }
}

#[derive(Debug)]
struct Database;

impl Database{
    fn connect() ->Result<Self, String>{
        Ok(Database)
    }
    fn find_employee(&self, name: &str)-> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee{
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee{
                name: "Anita".to_string(),
            }),
            "Catherine" => Ok(Employee{
                name: "Catherine".to_string(),
            }),
            _=> Err(String::from("employee not found"))
        }    
    }
    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String>{
        match employee.name.as_str() {  
            "Anita" => Ok(KeyCard{access_level: 1000}),
            "Brody" => Ok(KeyCard{access_level: 500}),
            other => Err(format!("{other} doesn't have a key card"))
        }
    }
}

#[derive(Clone, Debug)]
struct Employee{
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Clone, Debug)]
enum AuthorizationStatus{
    Allow,
    Deny
}
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}
impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(
                Self {
                    age,
                    name: name.to_string(),
                }
            )
        } else {
            Err("Age must be at least 21")
        }
    }
}
fn authorize(employee_name: &str,
             location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    let db = Database::connect()?;
    let employee = db.find_employee(employee_name)?;
    let keycard = db.get_keycard(&employee)?;

    if keycard.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}
fn main() {
    let adult1 = Adult::new(21, "Bob");
    match adult1 {
        Ok(a) => println!("Adult:{:?}", a),
        Err(e) => println!("Error{e}"),
    }
    let child = Adult::new(11, "Jane");
    match child {
        Ok(a) => println!("Adult:{:?}", a),
        Err(e) => println!("Error{e}"),
    }
    let  anita_authorized= authorize("Anita", ProtectedLocation::Warehouse);
    let  brody_authorized= authorize("Brody", ProtectedLocation::Office);
    let  catherine_authorized= authorize("Catherine", ProtectedLocation::Warehouse);
    
    println!("{anita_authorized:?}");
    println!("{brody_authorized:?}");
    println!("{catherine_authorized:?}");
}