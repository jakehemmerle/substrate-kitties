#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{StorageValue, ValueQuery};

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::storage]
    #[pallet::getter(fn get_storage_value)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue<
        _,
        u64,
        ValueQuery,
    >;
}