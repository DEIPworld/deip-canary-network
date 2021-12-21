//! # DEIP Portal Module
//! A module to make transactions with a Portal signature
//! 
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//! A module to make transactions with a Portal signature
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create` - Create a Portal.
//! * `update` - Update a Portal.
//! * `schedule` - Schedule an extrinsic to be executed within Portal context.
//! * `exec` - Call-wrapper that may be scheduled.
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests;
mod transaction_ctx;
mod portal;


#[doc(inline)]
pub use pallet::*;
#[doc(inline)]
pub use transaction_ctx::*;
#[doc(inline)]
pub use portal::*;

#[doc(hidden)]
pub use deip_transaction_ctx::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    
    use frame_support::pallet_prelude::*;
    use frame_support::{Hashable};
    use frame_support::weights::{PostDispatchInfo, GetDispatchInfo};
    
    use frame_support::traits::{UnfilteredDispatchable, IsSubType, ExtrinsicCall};
    use frame_support::log::debug;
    
    use sp_std::prelude::*;
    use sp_std::collections::{btree_map::BTreeMap};
    use sp_std::iter::FromIterator;
    
    use sp_runtime::{MultiSigner, traits::{Dispatchable, IdentifyAccount}, DispatchResultWithInfo};
    use frame_support::dispatch::DispatchResult;
    use codec::EncodeLike;
    
    use super::*;
    
    use frame_system::offchain::SendTransactionTypes;
    use sp_runtime::traits::Extrinsic;
    use sp_std::fmt::Debug;

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> + PortalModuleT<Self> + Debug {
    
        type TenantLookup: TenantLookupT<Self::AccountId, TenantId = Self::PortalId> + Debug;
        
        type PortalId: Member + Parameter + Copy + Default;
        type Portal: PortalT<Self> + Member + Parameter;
        
        type Call: Parameter +
             Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo> +
             GetDispatchInfo +
             From<frame_system::pallet::Call<Self>> +
             UnfilteredDispatchable<Origin = Self::Origin> +
             frame_support::dispatch::Codec + 
             IsSubType<Call<Self>>;
        
        type UnsignedValidator: ValidateUnsigned<Call=<Self as Config>::Call>;
        
        type UncheckedExtrinsic: Encode + EncodeLike + Decode + Clone
            + sp_std::fmt::Debug + Eq + PartialEq
            + frame_support::traits::ExtrinsicCall
            + Extrinsic<Call = <Self as Config>::Call>;
    }
    
    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(n: BlockNumberFor<T>) {
            if !sp_io::offchain::is_validator() {
                debug!("{}", "not a validator");
                return
            }
            let _ = crate::PortalCtxOf::<T>::submit_scheduled(n);
        }
    }
    
    #[pallet::error]
    pub enum Error<T> {
        DelegateMismatch,
        PortalMismatch,
        AlreadyScheduled,
        UnproperCall,
        NotScheduled,
        OwnerIsNotATenant,
        PortalAlreadyExist,
        PortalNotFound,
    }
    
    // #[pallet::event]
    // #[pallet::metadata(u32 = "SpecialU32")]
    // #[pallet::generate_deposit(fn deposit_event)]
    // pub enum Event<T: Config> {}
    
    #[doc(hidden)]
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}
    
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }
    
    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(
            source: TransactionSource,
            call: &Self::Call,
        )
            -> TransactionValidity
        {
            if let Call::exec_postponed(ref _portal_id, target_call) = call {
                T::UnsignedValidator::validate_unsigned(source, target_call)
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }
    
    use deip_transaction_ctx::{PortalCtxT, TransactionCtxT};
    
    #[pallet::call]
    impl<T: Config> Pallet<T>
    {
        #[pallet::weight(10000)]
        pub fn create(
            origin: OriginFor<T>,
            delegate: PortalDelegate<T>,
            metadata: PortalMetadata
        )
            -> DispatchResultWithPostInfo
        {
            // sp_runtime::runtime_logger::RuntimeLogger::init();
            let owner = ensure_signed(origin)?;
            T::create_portal(owner, delegate, metadata)?;
            Ok(Some(0).into())
        }
        
        #[pallet::weight(10000)]
        pub fn update(
            origin: OriginFor<T>,
            update: PortalUpdate<T>
        )
            -> DispatchResultWithPostInfo
        {
            // sp_runtime::runtime_logger::RuntimeLogger::init();
            let owner = ensure_signed(origin)?;
            T::update_portal(owner, update)?;
            Ok(Some(0).into())
        }
        
        #[pallet::weight(0)]
        pub fn schedule(
            origin: OriginFor<T>,
            xt: Box<T::UncheckedExtrinsic>,
        )
            -> DispatchResultWithPostInfo
        {
            // sp_runtime::runtime_logger::RuntimeLogger::init();
            let delegate = ensure_signed(origin)?;
            crate::PortalCtxOf::<T>::current().schedule_extrinsic(*xt, delegate)?;
            Ok(Some(0).into())
        }
        
        #[pallet::weight(0)]
        pub fn exec(
            origin: OriginFor<T>,
            portal_id: PortalId<T>,
            call: Box<<T as Config>::Call>,
        )
            -> DispatchResultWithPostInfo
        {
            // sp_runtime::runtime_logger::RuntimeLogger::init();
            crate::PortalCtxOf::<T>::current().dispatch_scheduled(portal_id, *call, origin)?
        }
        
        #[pallet::weight(0)]
        pub fn exec_postponed(
            origin: OriginFor<T>,
            portal_id: PortalId<T>,
            call: Box<<T as Config>::Call>,
        )
            -> DispatchResultWithPostInfo
        {
            // sp_runtime::runtime_logger::RuntimeLogger::init();
            ensure_none(origin)?;
            crate::PortalCtxOf::<T>::current().dispatch(portal_id, *call, RawOrigin::None.into())
        }
    }
    
    // ==== Storage ====:
    
    pub type ExtrinsicId = u32;
    pub type ExtrinsicIdList = Vec<ExtrinsicId>;
    pub type PortalInfo<PortalId> = Vec<(PortalId, ExtrinsicIdList)>;
    pub type ExtrinsicHash = [u8; 32];
    
    pub fn transpose<'a, T, S, PortalId>(source: S) -> T
        where T: FromIterator<(&'a ExtrinsicId, &'a PortalId)> + 'a,
              S: Iterator<Item = &'a (PortalId, ExtrinsicIdList)> + 'a,
              PortalId: 'a
    {
        T::from_iter(source.map(|(x, y)| { y.iter().map(move |z| (z, x)) }).flatten())
    }
    
    #[pallet::storage]
    pub(super) type PendingTx<T: Config> = StorageDoubleMap<_,
        Twox64Concat,
        T::BlockNumber,
        Blake2_128Concat,
        ExtrinsicHash,
        T::UncheckedExtrinsic,
    >;
    
    #[pallet::storage]
    pub(super) type ScheduledTx<T: Config> = StorageMap<_,
        Blake2_128Concat,
        ExtrinsicHash,
        PortalId<T>,
    >;
    
    #[pallet::storage]
    pub(super) type PortalTagOfTransaction<T: Config> = StorageDoubleMap<_,
        Twox64Concat,
        BlockNumberFor<T>,
        Blake2_128Concat,
        PortalId<T>,
        ExtrinsicIdList,
        OptionQuery
    >;
    
    #[pallet::storage]
    pub(super) type PortalRepository<T: Config> = StorageMap<_,
        Blake2_128Concat,
        PortalId<T>,
        T::Portal,
    >;
    
    #[pallet::storage]
    pub(super) type DelegateLookup<T: Config> = StorageMap<_,
        Blake2_128Concat,
        PortalId<T>,
        PortalDelegate<T>,
    >;
    
    #[pallet::storage]
    pub(super) type OwnerLookup<T: Config> = StorageMap<_,
        Blake2_128Concat,
        PortalOwner<T>,
        PortalId<T>,
    >;
    
    use storage_ops::*;
    use sp_core::H256;

    /// Module contains abstractions over pallet storage operations
    pub mod storage_ops {
        use sp_std::prelude::*;
        // use pallet_deip_toolkit::storage_ops::StorageOp;
        use super::{Config, Pallet};
        // 
        // /// Storage operations
        // pub enum StorageOps<T: Config> {
        //     /// Deposit event
        //     DepositEvent(Event<T>),
        //     /// Create DAO
        //     CreateDao(DaoOf<T>),
        //     /// Update DAO
        //     UpdateDao(DaoOf<T>),
        //     
        // }
        // impl<T: Config> StorageOp for StorageOps<T> {
        //     fn exec(self) {
        //         match self {
        //             Self::DepositEvent(e) => {
        //                 Pallet::<T>::deposit_event(e)
        //             },
        //             Self::CreateDao(dao) => {
        //                 DaoLookup::<T>::insert(dao.dao_key().clone(), dao.id().clone());
        //                 DaoRepository::<T>::insert(*dao.id(), dao);
        //             }
        //             Self::UpdateDao(dao) => {
        //                 DaoRepository::<T>::insert(*dao.id(), dao);
        //             }
        //         }
        //     }
        // }
    }
}
