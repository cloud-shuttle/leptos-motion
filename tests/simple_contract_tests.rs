//! Simple Contract Tests for Leptos Motion
//!
//! This test suite runs basic contract tests to ensure core functionality works.

use leptos_motion_contracts::simple_contract_tests::*;

#[test]
fn test_all_simple_contracts() {
    let results = run_all_simple_contract_tests();
    print_contract_test_results(&results);
}
