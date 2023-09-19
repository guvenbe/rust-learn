trait  UIComponent {
    fn render(&self){
        println!("Rendering component..")
    }
}

struct Button {
    text: String
}

impl UIComponent for Button{}

struct Container {
    name: String,
    child: Box<Container>, //since we don't know size we need to put this behind some pointer
}

fn main() {
    let button_a = Button{ //Button is stored on the stack
        text: "button_a".to_owned(),
    };

    let button_b = Box::new(Button{ //stored on the heap
        text: "button_b".to_owned(),
    });

    let  button_c = button_a; //entire data is copied to give 0wnership
    let  button_d = button_b; //only the box smart pointer is copied (smaller amount of data)

    let componenets: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d
    ];
}