#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Deserialize;
use std::collections::HashMap;
use wmi::{COMLibrary, Variant, WMIConnection, WMIDateTime};
// use wmi::{COMLibrary, Variant, WMIConnection};
// use wmi::COMLibrary;
// use wmi::Variant;
// use wmi::WMIConnection;
// use wmi::WMIDateTime;
// use wmi::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remote_machine_ip = "xxxxx";
    let username = "xxxxx";
    let password = "xxxxx";

    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT * FROM Win32_OperatingSystem")?;

    for os in results {
        println!("{:#?}", os);
    }

    #[derive(Deserialize, Debug)]
    struct Win32_OperatingSystem {
        Caption: String,
        Name: String,
        CurrentTimeZone: i16,
        Debug: bool,
        EncryptionLevel: u32,
        ForegroundApplicationBoost: u8,
        LastBootUpTime: WMIDateTime,
    }

    let results: Vec<Win32_OperatingSystem> = wmi_con.query()?;

    for os in results {
        println!("{:#?}", os);
    }

    Ok(())
}
