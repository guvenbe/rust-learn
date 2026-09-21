use crate::{ParseError, ValidationError, NonEmptyString, PositiveQuantity, Money, Currency, CustomerId, OrderId};

/// Borrowed view: shows lifetimes. No allocations here.
#[derive(Debug, Clone, Copy)]
pub struct ParsedOrder<'a> {
    pub id: &'a str,
    pub customer_id: &'a str,
    pub currency: &'a str,
    pub lines: &'a str, // comma-separated SKU:qty:unit_price
}

#[derive(Debug, Clone, Copy)]
pub struct ValidOrder<'a> {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub currency: Currency,
    pub lines: &'a str, // still borrowed; we will parse items later step-by-step
}

pub fn parse_order(input: &str) -> Result<ParsedOrder<'_>, ParseError> {
    // very simple format: "id=1;customer=10;currency=BRL;lines=ABC:2:10,DEF:1:15"
    let mut id = None; let mut customer = None; let mut currency = None; let mut lines = None;
    for part in input.split(';') {
        let (k, v) = part.split_once('=').ok_or(ParseError::InvalidFormat)?;
        let k = k.trim(); let v = v.trim();
        match k {
            "id" => id = Some(v),
            "customer" => customer = Some(v),
            "currency" => currency = Some(v),
            "lines" => lines = Some(v),
            _ => {}
        }
    }
    Ok(ParsedOrder {
        id: id.ok_or(ParseError::MissingField("id"))?,
        customer_id: customer.ok_or(ParseError::MissingField("customer"))?,
        currency: currency.ok_or(ParseError::MissingField("currency"))?,
        lines: lines.ok_or(ParseError::MissingField("lines"))?,
    })
}

pub fn validate_order(p: ParsedOrder<'_>) -> Result<ValidOrder<'_>, ValidationError> {
    let id_num: u64 = p.id.parse().map_err(|_| ValidationError::Empty("id"))?;
    let cust_num: u64 = p.customer_id.parse().map_err(|_| ValidationError::Empty("customer"))?;
    let currency = match p.currency { "BRL" => Currency::BRL, "USD" => Currency::USD, "EUR" => Currency::EUR, _ => return Err(ValidationError::Empty("currency")) };
    Ok(ValidOrder { id: OrderId(id_num), customer_id: CustomerId(cust_num), currency, lines: p.lines })
}

#[derive(Debug, Clone)]
pub struct OrderLineOwned {
    pub sku: NonEmptyString,
    pub qty: PositiveQuantity,
    pub unit: Money,
}

pub fn parse_lines(lines: &str, currency: Currency) -> Result<Vec<OrderLineOwned>, ParseError> {
    // lines format: each item SKU:qty:unit_price_units (units, not cents)
    let mut out = Vec::new();
    if lines.trim().is_empty() { return Ok(out); }
    for raw in lines.split(',') {
        let (sku, rest) = raw.split_once(':').ok_or(ParseError::InvalidFormat)?;
        let (qty, price) = rest.split_once(':').ok_or(ParseError::InvalidFormat)?;
        let sku_ne = NonEmptyString::new(sku, "sku").map_err(|_| ParseError::InvalidFormat)?;
        let qty_num: u32 = qty.parse().map_err(|_| ParseError::InvalidNumber("qty"))?;
        let qty_pos = PositiveQuantity::new(qty_num).map_err(|_| ParseError::InvalidNumber("qty"))?;
        let price_units: i64 = price.parse().map_err(|_| ParseError::InvalidNumber("unit_price"))?;
        let unit = Money::from_units(price_units, currency);
        out.push(OrderLineOwned { sku: sku_ne, qty: qty_pos, unit });
    }
    Ok(out)
}
