use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
/// PortRange
/// This is sample
pub enum PortRange{
    Single(u16),
    Range((u16, u16)),
    Multi(Vec<u16>)
}

impl PortRange {
    pub fn get_port_count(&self) -> u16 {
        match self {
            PortRange::Single(_) => 1,
            PortRange::Range((start, end)) => end - start,
            PortRange::Multi(ports) => ports.len() as u16,
        }
    }
}

impl FromStr for PortRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(n) => Ok(PortRange::Single(n)),
            Err(_) if s.contains(',') => {
                let ports: Vec<u16> = s.split(',').map(|n| n.parse::<u16>().unwrap()).collect();
                Ok(PortRange::Multi(ports))
            },
            Err(_) if s.contains('-') => {
                let (start, end) = s.splitn(2, '-').map(|n| n.parse::<u16>().unwrap()).collect_tuple().unwrap();
                Ok(PortRange::Range((start, end)))
            },
            Err(_) => Err(String::from("Failed to parse the port range"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::PortRange;
    #[test]
    fn test_single_correct_port() {
        assert_eq!(PortRange::Single(1234), PortRange::from_str("1234").unwrap());
    }

    #[test]
    fn test_port_range() {
        assert_eq!(PortRange::Range((10, 20)), PortRange::from_str("10-20").unwrap());
    }

    #[test]
    fn test_port_multi() {
        assert_eq!(PortRange::Multi(vec![1,2,5,600]), PortRange::from_str("1,2,5,600").unwrap());
    }

    #[test]
    fn test_get_port_count_of_single() {
        assert_eq!(1, PortRange::Single(1).get_port_count());
    }

    #[test]
    fn test_get_port_count_of_range() {
        assert_eq!(10, PortRange::Range((10, 20)).get_port_count());
    }

    #[test]
    fn test_get_port_count_of_multi() {
        assert_eq!(4, PortRange::Multi(vec![1, 2, 3, 4]).get_port_count());
    }
}