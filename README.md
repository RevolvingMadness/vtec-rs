# vtec-rs

A rust crate for parsing P-VTEC and H-VTEC strings found in various NWS text products (and probably other places too).

**P-VTEC Example**

```rs
use std::str::FromStr;

use vtec::primary::PrimaryVtec;

fn main() {
    let vtec = PrimaryVtec::from_str("/O.NEW.KGYX.SV.W.0009.260618T1839Z-260618T1945Z/").unwrap();

    println!("{:#?}", vtec);
}

/*
PrimaryVtec {
    identifier: Operational,
    action: New,
    office_id: "KGYX",
    phenomenon: SevereThunderstorm,
    significance: Warning,
    tracking_number: 9,
    start_date: Some(
        2026-06-18T18:39:00Z,
    ),
    end_date: Some(
        2026-06-18T19:45:00Z,
    ),
}
*/
```

**H-VTEC Example**

```rs
use std::str::FromStr;

use vtec::hydrologic::HydrologicVtec;

fn main() {
    let vtec =
        HydrologicVtec::from_str("/MONL1.1.ER.260618T0404Z.260620T0000Z.260622T0712Z.NO/").unwrap();

    println!("{:#?}", vtec);
}

/*
HydrologicVtec {
    site_identifier: "MONL1",
    flood_severity: Minor,
    immediate_cause: ExcessiveRainfall,
    beginning_date: Some(
        2026-06-18T04:04:00Z,
    ),
    crest_date: Some(
        2026-06-20T00:00:00Z,
    ),
    ending_date: Some(
        2026-06-22T07:12:00Z,
    ),
    flood_record_status: NotExpected,
}
*/
```
