﻿use rand;
use std::fmt;
use std::fmt::Display;

use rand::RngCore;
use smoltcp::wire;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill.bytes(&mut octets);
        octets[0] |= 0b_0000_0010; // Set 1 for the local address bits
        octets[0] &= 0b_1111_1100; // Set 0 for the uni cast bits
        MacAddress { 0: octets }
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress { 0: self.0 }
    }
}
