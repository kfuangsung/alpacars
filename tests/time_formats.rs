//! Dates and times are `jiff` types. These pin the shapes Alpaca actually sends and expects,
//! so the switch from chrono (0.9.0) can't silently change what parses or what goes on the
//! wire.

use alpacars::broker::requests::AgreementRequest;
use alpacars::trading::models::{Clock, Order};
use jiff::{civil, Timestamp};
use serde_json::json;

#[test]
fn test_timestamps_with_offsets_and_nanoseconds_parse() {
    // getclock-1 example: offsets other than Z.
    let clock: Clock = serde_json::from_value(json!({
        "timestamp": "2026-11-20T09:30:00.123456789-05:00",
        "is_open": true,
        "next_open": "2026-11-21T09:30:00-05:00",
        "next_close": "2026-11-20T16:00:00-05:00"
    }))
    .unwrap();
    assert_eq!(
        clock.timestamp.to_string(),
        "2026-11-20T14:30:00.123456789Z"
    );
    assert_eq!(clock.next_close.to_string(), "2026-11-20T21:00:00Z");

    // Nanosecond precision on orders, and the zero time some Broker fields return.
    let order: Order = serde_json::from_value(json!({
        "id": "83f37e9f-6b1f-49ed-8fc6-3e6af716323f",
        "client_order_id": "646b1fe6-b212-4f54-94c6-429e7bcdee04",
        "created_at": "2024-12-10T16:15:53.677230742Z",
        "order_class": "simple",
        "time_in_force": "day",
        "status": "new",
        "extended_hours": false
    }))
    .unwrap();
    assert_eq!(order.created_at.unwrap().subsec_nanosecond(), 677_230_742);
    let zero: Timestamp = serde_json::from_value(json!("0001-01-01T00:00:00Z")).unwrap();
    assert_eq!(zero.to_string(), "0001-01-01T00:00:00Z");
}

#[test]
fn test_request_dates_and_timestamps_serialize_as_before() {
    let agreement = AgreementRequest {
        agreement: alpacars::broker::enums::AgreementType::Customer,
        signed_at: "2024-01-15T10:30:00Z".parse().unwrap(),
        ip_address: "127.0.0.1".into(),
        revision: None,
    };
    assert_eq!(
        serde_json::to_value(&agreement).unwrap()["signed_at"],
        json!("2024-01-15T10:30:00Z")
    );
    assert_eq!(
        serde_json::to_value(civil::date(2025, 1, 2)).unwrap(),
        json!("2025-01-02")
    );
}
