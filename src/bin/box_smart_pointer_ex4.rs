trait UIComponent {
    fn render(&self) {
        println!("Rendering component");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Conainer {
    name: String,
    child: Box<Conainer>,
}

impl UIComponent for Conainer<F8> {}
fn main() {
    //Sored on the stack
    let button_a = Button {
        text: "button_a".to_owned(),
    };
    //Stored on the heap with Box pointer
    let button_b = Box::new(Button {
        text: "button_b".to_owned(),
    });
    //All of button is coppied
    let button_c = button_a;
    //Only the Box pointer is copied
    let button_d = button_b;

    let component: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
