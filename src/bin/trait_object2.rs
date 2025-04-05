
trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl  Sale for FullSale{
    fn amount(&self) -> f64 {
        self.0
    }
}
struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon{
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}
struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo{
    fn amount(&self) -> f64 {
         self.0 * 0.9
    }
}

fn calculate_revanue(sales: &Vec<Box<dyn Sale>>) -> f64{
    sales.iter().map(|sale| sale.amount()).sum()
}
fn main() {
   let price = 20.0;
    let regular = Box::new(FullSale(price));
    let coupon = Box::new(OneDollarOffCoupon(price));
    let promot = Box::new(TenPercentOffPromo(price));
    
    let sales:Vec<Box<dyn Sale>> = vec![regular, coupon, promot];
    let revanue = calculate_revanue(&sales);
    println!("total_revanue = {}", revanue);
    
}