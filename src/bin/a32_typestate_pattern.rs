#[derive(Copy, Clone)]
struct LuggageId(usize);
struct Luggage(LuggageId);

struct CheckIn(LuggageId);
struct OnLoad(LuggageId);
struct OffLoad(LuggageId);
struct AwaitingPickup(LuggageId);
struct EndCustody(LuggageId);

impl Luggage{
    fn new(id: LuggageId) -> Self{
        Luggage(id)
    }
    fn check_in(self) -> CheckIn{
      CheckIn(self.0)
    }
}
impl CheckIn{
    fn onload(self) -> OnLoad {
        OnLoad(self.0)
    }
}
impl OnLoad{
    fn offload(self) -> OffLoad{
        OffLoad(self.0)
    }
}
impl OffLoad{
    fn carousel(self) -> AwaitingPickup{
        AwaitingPickup(self.0)
    }
}
impl AwaitingPickup{
    fn pickup(self) -> (Luggage, EndCustody){
        (Luggage(self.0), EndCustody(self.0))
    }
}
fn main() {
    let luggageId = LuggageId(1);
    let luggage = Luggage::new(luggageId).check_in().onload().offload().carousel();    
    let (luggage, _) = luggage.pickup();
}