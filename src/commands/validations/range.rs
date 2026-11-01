use color_print::cformat;

use crate::commands::validations::structure::Validations;


#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub from: usize,
    pub to: usize,
}

impl Range {
    pub fn parse(value: &str) -> Result<Self, String> {
        if !value.contains(":") {
            return Err(cformat!("\n<y>* Invalid range, should at least contain a '<r>:</>'</>"));
        }
        
        let parts: Vec<&str> = value.split(":").collect();
        if parts.len() > 2 {
            return Err(cformat!("\n<y>* Invalid range, can only contain one '<r>:</>'</>"));
        }
        
        let from_str = parts[0].trim();
        let to_str = parts[1].trim();
        
        let from = if from_str.is_empty() { 1 } 
            else { match from_str.parse::<usize>() {
                Ok(0) => return Err(cformat!("\n<y>* Invalid range, start page cannot be 0</>")),
                Ok(n) => n,
                Err(_) => return Err(cformat!("\n<y>* Invalid range, '<r>{}</>' is not a valid number</>", from_str)),
            }};
        
        let to = if to_str.is_empty() { usize::MAX } 
            else { match to_str.parse::<usize>() {
                Ok(0) => return Err(cformat!("\n<y>* Invalid range, end page cannot be 0</>")),
                Ok(n) => {
                    if n < from {
                        return Err(cformat!("\n<y>* Invalid range, end page (<r>{}</>) cannot be less than start page (<r>{}</>)</>", n, from));
                    }
                    n
                },
                Err(_) => return Err(cformat!("\n<y>* Invalid range, '<r>{}</>' is not a valid number</>", to_str)),
            }};
        
        Ok(Range { from, to })
    }
    pub fn to_or(&self, other: usize) -> usize { if self.to == usize::MAX { other } else { self.to } }
    pub fn take(&self) -> usize {
        if self.to == usize::MAX { return self.to; }
        self.to - self.from + 1
    }
}

impl Default for Range {
    fn default() -> Self {
        Self { 
            from: 1, 
            to: usize::MAX,
        }
    }
}

impl Validations for Range {
    type Output = Range;

    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        Range::parse(value)
    }

    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        Range::parse(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_range() {
        let r = Range::parse("2:6").unwrap();
        assert_eq!(r.from, 2);
        assert_eq!(r.to, 6);
        assert_eq!(r.take(), 5);
    }

    #[test]
    fn open_start_range() {
        let r = Range::parse(":3").unwrap();
        assert_eq!(r.from, 1);
        assert_eq!(r.to, 3);
        assert_eq!(r.take(), 3);
    }

    #[test]
    fn open_end_range() {
        let r = Range::parse("1:").unwrap();
        assert_eq!(r.from, 1);
        assert_eq!(r.to, usize::MAX);
        assert_eq!(r.take(), usize::MAX);
    }

    #[test]
    fn open_both_range() {
        let r = Range::parse(":").unwrap();
        assert_eq!(r.from, 1);
        assert_eq!(r.to, usize::MAX);
        assert_eq!(r.take(), usize::MAX);
    }

    #[test]
    fn same_from_and_to() {
        let r = Range::parse("3:3").unwrap();
        assert_eq!(r.from, 3);
        assert_eq!(r.to, 3);
        assert_eq!(r.take(), 1);
    }

    #[test]
    fn missing_colon() {
        assert!(Range::parse("3").is_err());
    }

    #[test]
    fn multiple_colons() {
        assert!(Range::parse("1:2:3").is_err());
    }

    #[test]
    fn from_greater_than_to() {
        assert!(Range::parse("5:2").is_err());
    }

    #[test]
    fn zero_from() {
        assert!(Range::parse("0:5").is_err());
    }

    #[test]
    fn zero_to() {
        assert!(Range::parse(":0").is_err());
    }

    #[test]
    fn invalid_number() {
        assert!(Range::parse("abc:2").is_err());
        assert!(Range::parse("2:xyz").is_err());
    }
}
