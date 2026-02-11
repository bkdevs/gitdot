#[macro_export]
macro_rules! cfg_modules {
    ($feature:literal, { $($item:item)* }) => {
        $(
            #[cfg(feature = $feature)]
            $item
        )*
    };
}

#[macro_export]
macro_rules! cfg_use {
    ($feature:literal, { $($item:item)* }) => {
        $(
            #[cfg(feature = $feature)]
            $item
        )*
    };
}
