//! The Mint that represents the native token

/// There are 10^9 lamports in one HC
pub const DECIMALS: u8 = 9;

/// The symbol in HPL-Toekn HC
pub const SYMBOL: &str = "WHC";
/// The name in HPL-Toekn HC
pub const NAME: &str = "Wrap HC";
/// The icon url in HPL-Toekn HC
pub const ICON: &str = "https://pubchain-icon.xone.la/a6r9zz185xev3w9dgylbrwbcxi40v2wh.png";

// The Mint for native HC Token accounts
huione_program::declare_id!("Hc11111111111111111111111111111111111111111");

#[cfg(test)]
mod tests {
    use super::*;
    use huione_program::native_token::*;

    #[test]
    fn test_decimals() {
        // assert!(
        //     (lamports_to_hc(42) - crate::amount_to_ui_amount(42, DECIMALS)).abs() < f64::EPSILON
        // );
        assert_eq!(
            hc_to_lamports("42."),
            crate::ui_amount_to_amount(42,0.0, DECIMALS)
        );
    }
}
