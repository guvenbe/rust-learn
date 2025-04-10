struct File<'a>(&'a str);
impl <'a> File<'a> {
    fn read_bytes(&self)->Vec<u8>{
        vec![1,2,4]
    }
    fn delete(self){
        
    }
}
fn main() {
    let file = File("data.txt");
    let data = file.read_bytes();
    file.delete();
    //Can't read since ownership moved
    // let read_again = file.read_bytes()1
}