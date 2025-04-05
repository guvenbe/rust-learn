 trait Game {
     fn name(&self) ->String;
 }
 #[derive(Debug)]
 enum BoardGame {
     Chess,
     Monopoly,
 }
 impl Game for BoardGame {
     fn name(&self) -> String {
         self.name()
     }
 }

 #[derive(Debug)]
 enum VideoGame {
     PlayStation,
     Xbox,
 }
 impl Game for VideoGame{
     fn name(&self) -> String {
         self.name()
     }
 }
 
 struct PlayRoom<T: Game>{
     game: T,
 }
 
 impl PlayRoom<BoardGame> {
     pub fn cleanup(&self){
         
     }
 }
 
 
fn main() {
    let video_room = PlayRoom {
        game: VideoGame::Xbox,
    };
    let board_room = PlayRoom {
        game: BoardGame::Monopoly,
    };
    board_room.cleanup();
   println!("Game:{:?}",video_room.game);
    
}