use super::split::QifSplit;
use crate::errors::QifParsingError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a transaction
/// It has a date and an amount, and possibly some splits
#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(build_fn(error = "QifParsingError"))]
pub struct QifTransaction<'a> {
    /// Parsed date, with format YYYY-MM-DD
    pub date: String,
    pub amount: i64,
    pub memo: &'a str,
    pub payee: &'a str,
    pub category: &'a str,
    pub cleared_status: &'a str,
    pub address: Vec<&'a str>,
    pub splits: Vec<QifSplit<'a>>,
    pub number_of_the_check: &'a str,
}

impl<'a> fmt::Display for QifTransaction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.date, self.amount, self.memo, self.payee
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{date::parse_date, errors::QifParsingError, parse_number};

    use super::*;

    #[test]
    fn test_builder() -> Result<(), QifParsingError> {
        let tb = QifTransactionBuilder::default()
            .date("03/03/10".to_string())
            .amount(match parse_number("T-379.00") {
                Ok(it) => it,
                Err(err) => return Err(err),
            })
            .payee("CITY OF SPRINGFIELD")
            .memo("DUMMY")
            .category("DUMMY")
            .cleared_status("DUMMY")
            .address(vec!["DUMMY"])
            .number_of_the_check("DUMMY")
            .splits(vec![])
            .build()
            .or_else(|err: QifParsingError| return Err(err))
            .unwrap();
        assert_eq!(
            parse_date(&tb.date, "%m/%d/%y")
                .or_else(|err| return Err(err))
                .unwrap(),
            "2010-03-03"
        );
        assert_eq!(tb.amount, -37900);
        assert_eq!(tb.payee, "CITY OF SPRINGFIELD");
        Ok(())
    }
}

/*
!Type:Bank
D03/03/10
T-379.00
PCITY OF SPRINGFIELD
^
D03/04/10
T-20.28
PYOUR LOCAL SUPERMARKET
^
D03/03/10
T-421.35
PSPRINGFIELD WATER UTILITY
^ */
