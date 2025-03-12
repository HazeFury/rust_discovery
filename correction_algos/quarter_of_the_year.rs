// Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.

// For example: month 2 (February), is part of the first quarter; month 6 (June), is part of the second quarter; and month 11 (November), is part of the fourth quarter.

// Constraint:

// 1 <= month <= 12

// -------------------------------------------------------------------------


fn quarter_of(month: u8) -> u8 {
    match month {
       m if m > 0 && m <= 3 => 1,
       m if m > 3 && m <= 6 => 2,
       m if m > 6 && m <= 9 => 3,
       m if m > 9 && m <= 12 => 4,
        _ => 0
      }
  }
  
  fn main() {
          assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
          assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
          assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
  }