use order_processor::*;

fn main() {
    println!("== Order processor ==\n");

    // 1) Small, specific functions: parse -> validate -> transform happens inside the service
    let input = "id=1;customer=10;currency=BRL;lines=ABC-123:2:10,XYZ-999:1:15";

    let repo = InMemoryOrderRepo::new();
    let notifier = StdoutNotifier;
    let logger = StdoutLogger;
    let mut svc = OrderService::new(repo, notifier, logger);

    let order = svc.place_order(input).expect("place_order ok");
    println!("placed order id={} for customer={}", order.id.0, order.customer_id.0);

    let total = order.total().expect("total ok");
    println!("order total = {:?} {} cents", order.currency, total.cents);

    // 2) Deterministic cleanup demonstration (RAII / Drop)
    println!("\n-- deterministic cleanup demo (mirrors CPython vs PyPy lesson) ");
    export_demo();

    println!("\n== end ==");
}
