
use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, traits::Get, weights::Weight,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{AtLeast32Bit, UniqueSaturatedInto};
use sp_std::prelude::*;

// Define the struct representing an artwork, which consists of a name, artist, and appraisal value.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Artwork<AccountId, Hash> {
    name: Hash,
    artist: AccountId,
    appraisal_value: u32,
}

// Define the module's configuration trait, which specifies the type of events to be emitted and the type of artwork ID.
pub trait Config: system::Config {
    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;

    // The artwork ID must be a parameter, an unsigned integer of at least 32 bits, default to 0, and be copyable.
    type ArtworkId: Parameter + AtLeast32Bit + Default + Copy;
}

// Define the module's storage map, which maps artwork IDs to artwork structs and keeps track of the total number of artworks.
decl_storage! {
    trait Store for Module<T: Config> as ArtRegistry {
        pub Artworks get(fn artworks): map hasher(identity) T::ArtworkId => Option<Artwork<T::AccountId, T::Hash>>;
        pub ArtworkCount get(fn artwork_count): T::ArtworkId;
    }
}

// Define the module's events, which include artwork registration and appraisal updates.
decl_event!(
    pub enum Event<T> where
        AccountId = <T as system::Config>::AccountId,
        ArtworkId = <T as Config>::ArtworkId,
    {
        ArtworkRegistered(ArtworkId),
        AppraisalUpdated(ArtworkId, AccountId, u32),
    }
);

// Define the module's errors, which include artwork not found and owner mismatch.
decl_error! {
    pub enum Error for Module<T: Config> {
        ArtworkNotFound,
        OwnerMismatch,
    }
}

// Define the module's functions and associated weights, which include artwork registration and appraisal updates.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        fn register_artwork(origin, name: T::Hash, initial_appraisal_value: u32) {
            // Ensure that the sender of the transaction is a signed account.
            let sender = ensure_signed(origin)?;

            // Get the next artwork ID and create a new artwork struct with the provided name, artist, and appraisal value.
            let artwork_id = Self::next_artwork_id()?;
            let artwork = Artwork {
                name,
                artist: sender.clone(),
                appraisal_value: initial_appraisal_value,
            };

            // Insert the new artwork into the Artworks storage map and increment the artwork count.
            <Artworks<T>>::insert(artwork_id, artwork.clone());
            <ArtworkCount<T>>::put(artwork_id + 1.into());

            // Emit an ArtworkRegistered event.
            Self::deposit_event(Event::ArtworkRegistered(artwork_id));
        }

        #[weight = 10_000]
        fn update_appraisal(origin, artwork_id: T::ArtworkId, new_appraisal_value: u32) {
            // Ensure that the sender of the transaction is a signed account.
use codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, traits::Get, weights::Weight};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{AtLeast32Bit, UniqueSaturatedInto};
use sp_std::prelude::*;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Artwork<AccountId, Hash> {
    name: Hash,
    artist: AccountId,
    appraisal_value: u32,
}

pub trait Config: system::Config {
    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;

    type ArtworkId: Parameter + AtLeast32Bit + Default + Copy;
}

decl_storage! {
    trait Store for Module<T: Config> as ArtRegistry {
        pub Artworks get(fn artworks): map hasher(identity) T::ArtworkId => Option<Artwork<T::AccountId, T::Hash>>;
        pub ArtworkCount get(fn artwork_count): T::ArtworkId;
    }
}

decl_event!(
    pub enum Event<T> where
        AccountId = <T as system::Config>::AccountId,
        ArtworkId = <T as Config>::ArtworkId,
    {
        ArtworkRegistered(ArtworkId),
        AppraisalUpdated(ArtworkId, AccountId, u32),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        ArtworkNotFound,
        OwnerMismatch,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        fn register_artwork(origin, name: T::Hash, initial_appraisal_value: u32) {
            let sender = ensure_signed(origin)?;

            let artwork_id = Self::next_artwork_id()?;
            let artwork = Artwork {
                name,
                artist: sender.clone(),
                appraisal_value: initial_appraisal_value,
            };

            <Artworks<T>>::insert(artwork_id, artwork.clone());
            <ArtworkCount<T>>::put(artwork_id + 1.into());

            Self::deposit_event(Event::ArtworkRegistered(artwork_id));
        }

        #[weight = 10_000]
        fn update_appraisal(origin, artwork_id: T::ArtworkId, new_appraisal_value: u32) {
            let sender = ensure_signed(origin)?;

            let artwork = <Artworks<T>>::get(&artwork_id).ok_or(Error::<T>::ArtworkNotFound)?;
            ensure!(artwork.artist == sender, Error::<T>::OwnerMismatch);

            <Artworks<T>>::mutate(&artwork_id, |a| a.appraisal_value = new_appraisal_value);

            Self::deposit_event(Event::AppraisalUpdated(artwork_id, sender, new_appraisal_value));
        }
    }
}

impl<T: Config> Module<T> {
    fn next_artwork_id() -> Result<T::ArtworkId, &'static str> {
        let current_id = Self::artwork_count();
        current_id
            .checked_add(&1.into())
            .ok_or("Artwork ID overflow")
    }
}
