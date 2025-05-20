use num_traits::CheckedAdd;

fn main() {
    let a = 1;
    let b = 2;
    
    
    #[derive(Debug)]
    enum RoomType {
        Bedroom,
        Kitchen,
    }

    #[derive(Debug)]
    struct  Room {
        dimensions: (usize, usize),
        kind: RoomType,
    }
    
    let  kitchen = Room {
        dimensions: (20, 20),
        kind: RoomType::Kitchen,
    };
    dbg!(&kitchen);
    
    let h = "Hello";
    let w = " World";
    
    let greet  = format!("{}, {}!", h, w);
    println!("{}", greet);
    
    
    let msg = include_str!("msg.txt");
    println!("{}", msg);
    
    let bytes = include_bytes!("msg.txt");
    println!("{:?}", bytes);
    
    let config_1 = option_env!("CONFIG_1").unwrap_or("default_value");
    
    let number = 12;
    let max_5 = {
        if number > 5 {
            5
        } else {
            number
        }
    };
    
    match max_5 {
        n @ 0 ..=5 => println!("n= {}", n),
        _ => unreachable!("n > 5, this is a bug")
    }
   
    
    assert!(a==b , "{} ne {}", a ,b);
    assert_eq!(a,b, "values should be equal");
    assert_ne!(a,b, "values should not  be equal");
    debug_assert!(a==b , "{} ne {}", a ,b);
    debug_assert_eq!(a,b, "values should be equal");
    debug_assert_ne!(a,b, "values should not  be equal");
    
} 