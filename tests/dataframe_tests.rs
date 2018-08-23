
extern crate blackjack;

use blackjack::prelude::*;


#[test]
fn test_new_dataframe() {
    println!("Working!");
}

#[test]
fn test_add_columns_same_length() {
    let series_int = Series::arange(0, 5);
    let series_flt = Series::arange(0, 5);
    let mut df = DataFrame::new();
    df.add_column(series_int);
    df.add_column(series_flt);
}
