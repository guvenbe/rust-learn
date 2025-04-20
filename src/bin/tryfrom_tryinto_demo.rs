use std::convert::TryFrom;
use thiserror::Error;
use crate::ApiError::Network;

enum NonZeroError {
    IsZero,
}

struct NonZero(i32);

impl TryFrom<i32> for NonZero {
    type Error = NonZeroError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value == 0 {
            Err(NonZeroError::IsZero)
        } else {
            Ok(NonZero(value))
        }
    }
}

enum KeyPress {
   Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

enum InputEvent {
    key(u16, KeyPress),
    Mouse
}

impl From<KeyEvent> for InputEvent{
    fn from(ev: KeyEvent) -> Self{
        InputEvent::key(ev.keycode, ev.state)
    }
}
#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}
#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying database")]
    QueryFailure,
}
// impl From<NetworkError> for ApiError{
//     fn from(err: NetworkError) -> Self {
//         Self::Network(err)
//     }
// }
// impl From<DatabaseError> for ApiError{
//     fn from(err: DatabaseError) -> Self {
//         Self::Database(err)
//     }
// }

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::Timeout)?
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(#[from]NetworkError),
    #[error("database error: {0}")]
    Database(#[from]DatabaseError),
}
fn main() {
    match NonZero::try_from(9){
        Ok(nonzero) => println!("not zero"),
        Err(e) => println!("is zero!"),
    }
    let whoops: Result<NonZero, _> =0_i32.try_into();
    match whoops {
        Ok(nonzero) => println!("not zero"),
        Err(e) => println!("is zero"),
    }
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };
    
    let input_ev = InputEvent::from(key_ev);
    // let input_ev: InputEvent = key_ev.into();
}