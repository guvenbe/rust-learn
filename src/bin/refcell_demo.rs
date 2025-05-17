use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct PackageCounter(u32);

type  SharedPackageCounter =Rc<RefCell<PackageCounter>>;
#[derive(Debug)]
struct Truck(SharedPackageCounter);

#[derive(Debug)]
struct Dispatch(SharedPackageCounter);

fn main() {
    //counter > 0: exist in memory
    //counter ==0: deleted
    
    let counter = Rc::new(RefCell::new(PackageCounter(5)));
    
    //total: 2
    let truck = Truck(Rc::clone(&counter)); // +1
    let dispatch = Dispatch(counter);
    
    println!("before:"); 
    println!("{truck:?}");
    println!("{dispatch:?}");
    
    //truck unloads 1 package
    {
        let mut counter = truck.0.borrow_mut();
        counter.0 -= 1;
    }

    println!("\nafter:");
    println!("{truck:?}");
    println!("{dispatch:?}");
    
    let counter = dispatch.0.borrow();
    println!("\n{counter:?}")
    

}