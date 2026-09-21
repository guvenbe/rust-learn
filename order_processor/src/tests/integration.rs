use clean_code_rust_way::*;

#[test]
fn end_to_end_place_order() {
    let repo = InMemoryOrderRepo::new();
    let notifier = StdoutNotifier;
    let logger = StdoutLogger;
    let mut svc = OrderService::new(repo, notifier, logger);

    let input = "id=7;customer=21;currency=USD;lines=SKU1:2:5,SKU2:1:7";
    let order = svc.place_order(input).unwrap();
    assert_eq!(order.id.0, 7);
    assert_eq!(order.customer_id.0, 21);
    assert_eq!(order.lines.len(), 2);

    let total = order.total().unwrap();
    // (2 * $5) + (1 * $7) = $17 -> 1700 cents
    assert_eq!(total.cents, 1700);
}
