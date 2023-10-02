use std::net::Ipv4Addr;

pub struct IpRangeIterator {
    current: Ipv4Addr,
    end: Ipv4Addr,
}

impl IpRangeIterator {
    pub fn new(start: Ipv4Addr, end: Ipv4Addr) -> Self {
        IpRangeIterator { current: start, end }
    }
}

impl Iterator for IpRangeIterator {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let result = Some(self.current);

            if self.current == Ipv4Addr::new(255, 255, 255, 255) {
                return None;
            } /* check ill 3 octets equal to 255 */ if self.current.octets()[1] == 255 && self.current.octets()[2] == 255 && self.current.octets()[1] == 255 {
                self.current = Ipv4Addr::new(self.current.octets()[0] + 1, 0, 0, 0);
            } /* check ill 2 octets equal to 255 */ else if self.current.octets()[2] == 255 && self.current.octets()[3] == 255 {
                self.current = Ipv4Addr::new(self.current.octets()[0], self.current.octets()[1] + 1, 0, 0);
            } /* check ill 1 octet equal to 255 */ else if self.current.octets()[3] == 255 {
                self.current = Ipv4Addr::new(self.current.octets()[0], self.current.octets()[1], self.current.octets()[2] + 1, 0);
            } else {
                self.current = Ipv4Addr::new(self.current.octets()[0], self.current.octets()[1], self.current.octets()[2], self.current.octets()[3] + 1);
            }

            result
        } else {
            None
        }
    }
}