//! Integration tests for `#[derive(DataDefinition)]`, exercising every
//! supported field category (scalars, unit-suffix inference, fixed
//! strings, the general-purpose data types, and the two BCD types) with a
//! real `encode`/`decode`/`SCHEMA` round trip — no live sim connection
//! needed, since this only tests the compile-time codegen and its
//! generated methods.

use simconnect::proto::bcd::{Bco16, FrequencyBcd16};
use simconnect::proto::data::{LatLonAlt, MarkerState, Waypoint, Xyz};
use simconnect::proto::enums::DataType;
use simconnect::proto::strings::String256;
use simconnect::DataDefinition;

#[derive(DataDefinition, Debug, PartialEq)]
struct Everything {
    // `__1__mhz`: index and unit segments are classified by content, not
    // position, and can be peeled off in either order — this infers
    // `units = "MHz"` *and* derives the datum name as
    // `"COM ACTIVE FREQUENCY:1"` (colon, matching the real simvar), not
    // `"COM ACTIVE FREQUENCY 1"` (space).
    com_active_frequency__1__mhz: f64,
    com_active_frequency__2__mhz: f64,
    // Order doesn't matter: unit segment first, then index.
    com_standby_frequency__mhz__1: f64,
    on: bool,
    count: i32,
    big_count: i64,
    ratio: f32,
    title: String256,
    home: Waypoint,
    position: LatLonAlt,
    offset: Xyz,
    marker: MarkerState,
    #[simconnect(units = "BCO16")]
    squawk: Bco16,
    #[simconnect(units = "Frequency BCD16")]
    legacy_freq: FrequencyBcd16,
}

fn sample() -> Everything {
    Everything {
        com_active_frequency__1__mhz: 118.0,
        com_active_frequency__2__mhz: 121.5,
        com_standby_frequency__mhz__1: 118.5,
        on: true,
        count: -42,
        big_count: 9_000_000_000,
        ratio: 0.5,
        title: String256("Cessna 172".to_string()),
        home: Waypoint {
            latitude: 47.44,
            longitude: -122.30,
            altitude: 433.0,
            flags: 0,
            speed: 0.0,
            throttle: 0.0,
        },
        position: LatLonAlt {
            latitude: 1.0,
            longitude: 2.0,
            altitude: 3.0,
        },
        offset: Xyz {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        marker: MarkerState {
            name: "M1".to_string(),
            state: true,
        },
        squawk: Bco16::from_octal_digits(1200),
        legacy_freq: FrequencyBcd16::from_khz(118_000),
    }
}

#[test]
fn schema_has_one_entry_per_field_in_declaration_order() {
    let schema = Everything::SCHEMA;
    assert_eq!(schema.len(), 14);
    assert_eq!(schema[0].datum_name, "COM ACTIVE FREQUENCY:1");
    assert_eq!(schema[0].units_name, Some("MHz"));
    assert_eq!(schema[0].data_type, DataType::Float64);
    assert_eq!(schema[1].datum_name, "COM ACTIVE FREQUENCY:2");
    assert_eq!(schema[2].datum_name, "COM STANDBY FREQUENCY:1");
    assert_eq!(schema[2].units_name, Some("MHz"));
    assert_eq!(schema[3].datum_name, "ON");
    assert_eq!(schema[3].data_type, DataType::Int32);
    assert_eq!(schema[7].datum_name, "TITLE");
    assert_eq!(schema[7].data_type, DataType::String256);
    assert_eq!(schema[7].units_name, None);
    assert_eq!(schema[8].data_type, DataType::Waypoint);
    assert_eq!(schema[12].datum_name, "SQUAWK");
    assert_eq!(schema[12].units_name, Some("BCO16"));
    assert_eq!(schema[12].data_type, DataType::Int32);
    assert_eq!(schema[13].units_name, Some("Frequency BCD16"));
}

#[test]
fn encode_decode_round_trips() {
    let value = sample();
    let bytes = value.encode().expect("encode");
    let decoded = Everything::decode(&bytes).expect("decode");
    assert_eq!(decoded, value);
}
