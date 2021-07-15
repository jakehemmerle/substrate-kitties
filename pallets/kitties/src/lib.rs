#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{StorageValue, ValueQuery};
    use frame_support::sp_runtime::traits::{Hash};
    use frame_support::traits::{Randomness};
    use sp_core::hash::{H256};

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Debug, Clone, PartialEq)]
    enum Gender {
        Male,
        Female,
    }
    #[derive(Clone, Encode, Decode, Default, PartialEq)]
    pub struct Kitty<Hash, Balance> {
        id: Hash,
        dna: Hash,
        price: Balance,
        gender: Gender
    }    

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type KittyRandomness: Randomness<H256>;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {
        fn random_hash(sender: &T::AccountId) -> T::Hash {
            let nonce = <Nonce<T>>::get();
            let seed = T::Randomness::random_seed();
    
            T::Hashing::hash_of(&(seed, &sender, nonce))
        }
    }
    

    #[pallet::storage]
    #[pallet::getter(fn get_storage_value)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue<
        _,
        u64,
        ValueQuery,
    >;
}