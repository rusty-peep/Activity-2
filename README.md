# Basic Bank Transaction System

## Overview
This assignment involves creating a simple `BankAccount` struct to simulate basic bank transactions such as depositing and withdrawing money. The goal is to understand how to define structs and work with methods in Rust.

## Instructions

1. **Define the `BankAccount` Struct:**
    - Define a struct with two fields: `account_number` (u32) and `balance` (f64).

2. **Implement Methods:**
    - Implement the following methods for the `BankAccount` struct:
      - `new`: Creates a new bank account with an initial balance.
      - `get_balance`: Returns the current balance of the account. **Do not modify this method.**
      - `deposit`: Increases the balance by a given amount.
      - `withdraw`: Decreases the balance by a given amount if the amount is less than or equal to the current balance.
      - `transfer`: Transfers a specified amount from one account to another if the amount is less than or equal to the balance of the first account.

3. **Test the `BankAccount` Struct:**
    - Create instances of `BankAccount` and test the methods in the `main` function.


