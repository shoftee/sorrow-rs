pub mod buildings;
pub mod calendar;
pub mod options;
pub mod precision;
pub mod recipes;
pub mod resources;
pub mod time;
pub mod ui;

pub trait KeyIter {
    type Item;

    fn key_iter() -> impl Iterator<Item = Self::Item>;
}

#[macro_export]
macro_rules! state_key {
    { $vis:vis enum $ident:ident $tt:tt } => {
        #[derive(
            ::std::fmt::Debug,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::std::cmp::PartialEq,
            ::std::cmp::Eq,
            ::std::cmp::PartialOrd,
            ::std::cmp::Ord,
            ::std::hash::Hash,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::strum::EnumIter,
        )]
        $vis enum $ident $tt

        impl $crate::state::KeyIter for $ident {
            type Item = $ident;

            fn key_iter() -> impl Iterator<Item = Self::Item> {
                <$ident as ::strum::IntoEnumIterator>::iter()
            }
        }
    };
}
