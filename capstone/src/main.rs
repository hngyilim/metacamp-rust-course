mod location;

use std::fs::File;
use std::io::{BufReader, BufRead};
mod transaction;
use transaction::Transaction;
use location::Continent;
use std::collections::HashMap;

fn main() {
    /// a. create file variable by passing "./transactions.csv" into the File::open function,
    /// followed by calling the unwrap method
    let file = File::open("./transactions.csv").unwrap();

    /// b. create reader variable by passing file variable into the BufReader::new function
    let reader = BufReader::new(file);

    /// c. create mutable transactions variable of type Vec<Transaction> by calling Vec::new
    /// method
    let mut transactions: Vec<Transaction> = Vec::new();

    /// d. create mutable skipped_lines variable of no explicit type simply calling Vec::new method
    let mut skipped_lines: Vec<_> = Vec::new();

    ///     e. run a for loop destructured into arbitrary variables of (idx, line) using the reader variable
    /// which calls lines method followed by enumerate method
    /// - if idx equals to 0, continue
    /// - create line_str variable by using line to call the unwrap method
    /// - create parsed_transaction variable by passing line_str into
    /// Transaction::fram_csv_line method
    /// - match on parsed_transaction
    /// - if matches on Ok variant, push value within Ok into transactions
    /// - If matches on Err variant, push the tuple of (idx, value within Err, line_str)
    /// into skipped_lines
    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(error_string) => skipped_lines.push((idx, error_string, line_str)),
        };
    };

    /// f. run a for loop by using transactions to call the iter method
    /// - print every item in transactions
    for transaction in &transactions {
        println!("Valid Transaction: {:?}", transaction);
    }
    /// g. do the same for skipped_lines

    for skipped_line in skipped_lines {
        println!("Skipped lines: {:?}", skipped_line);
    }

    ///take home 1 
    ///Utilize HashMap to keep track of the total invested amount per continent
    /// and print the result out for each continent
    /// - Hint: You would need to convert the continent to String to store as keys
    let mut invested_per_continent: HashMap<String, f64> = HashMap::new(); 
    for transaction in &transactions {
        let continent_str = transaction.continent.to_string();
    
        match invested_per_continent.get(&continent_str) {
            Some(amount) =>  invested_per_continent.insert(continent_str, amount + transaction.amount),
            None => invested_per_continent.insert(continent_str, transaction.amount),
        };
    };
    println!("Total Invested Amount Per Continent:");
    for (key, value) in &invested_per_continent {
        println!("{}: {}", key, value);
    };

    /// Create a function that takes in a reference slice of transactions and a
    /// reference of Continent, and filters rows by the Continent. Print only
    /// transactions with European companies
    Transaction::filter_transactions_by_continent(transactions, Continent::Europe);
}
