//! Macros for conditional compilation

/// Macro to conditionally derive serde traits
#[macro_export]
macro_rules! derive_serde {
    ($($item:item)*) => {
        $(
            #[cfg_attr(feature = "serde-support", derive(serde::Serialize, serde::Deserialize))]
            $item
        )*
    };
}

/// Macro to conditionally derive serde traits with other derives
#[macro_export]
macro_rules! derive_with_serde {
    ($derives:meta, $($item:item)*) => {
        $(
            #[derive($derives)]
            #[cfg_attr(feature = "serde-support", derive(serde::Serialize, serde::Deserialize))]
            $item
        )*
    };
}
