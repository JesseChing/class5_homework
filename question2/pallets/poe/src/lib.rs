#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec; 


  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  }


  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);
  



  #[pallet::storage]
  pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

  #[pallet::event]
  #[pallet::metadata(T::AccountId = "AccountId")]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
	/// 存证创建
	ClaimCreated(T::AccountId, Vec<u8>),
	/// 存证删除
	ClaimRevoked(T::AccountId, Vec<u8>),
	// 存证转移
	ClaimMove(T::AccountId, Vec<u8>),
	// 存证持有者
	ClaimOwner(T::AccountId, Vec<u8>),
  }

  #[pallet::error]
  pub enum Error<T> {
	/// 存证已存在
	ProofAlreadyClaimed,
	/// 存证不存在
	NoSuchProof,
	/// 非存证持有者
	NotProofOwner,

  }
  
  #[pallet::call]
  impl<T: Config> Pallet<T> {
	
	/**
	 * 保存存证
	 */
	#[pallet::weight(0)]
	pub fn create_claim(
	  origin: OriginFor<T>,
	  proof: Vec<u8>,
	  ) -> DispatchResultWithPostInfo {
		let sender = ensure_signed(origin)?;
  
		// 检查是否已存在相同的存证
		ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
  
		// 获取当前区块
		let current_block = <frame_system::Pallet<T>>::block_number();
  
		// 保存存证信息
		Proofs::<T>::insert(&proof, (&sender, current_block));
  
		// Emit an event that the claim was created.
		Self::deposit_event(Event::ClaimCreated(sender, proof));
  
		Ok(().into())
		}
  
		/**
		 * 删除存证
		 */
		#[pallet::weight(0)]
		pub fn revoke_claim(
		  origin: OriginFor<T>,
		  proof: Vec<u8>
		  ) -> DispatchResultWithPostInfo {
		
			let sender = ensure_signed(origin)?;
  
			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
  
			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);
  
			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);
  
			// Remove claim from storage.
			Proofs::<T>::remove(&proof);
  
			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, proof));
			Ok(().into())
		  }

		/**
		 * 转移存证
		 */
		#[pallet::weight(0)]
		pub fn change_claim_owner(
		  origin: OriginFor<T>,
		  proof: Vec<u8>,
		  to_account_id: T::AccountId
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			//检查存证信息是否存在
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, current_block) = Proofs::<T>::get(&proof);
			//检查是否为存证的持有者
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			//覆盖原有的记录，改变持有者
			Proofs::<T>::insert(&proof, (&to_account_id, current_block));
            
			Self::deposit_event(Event::ClaimMove(to_account_id, proof));

			Ok(().into())
		}

		/**
		 * 判断某个账号是否为存证的拥有者
		 */
		#[pallet::weight(0)]
		pub fn is_the_claim_owner(
		  origin: OriginFor<T>,
		  proof: Vec<u8>,
		  to_account_id: T::AccountId
		) -> DispatchResultWithPostInfo {
			//检查存证信息是否存在
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof);
			//检查是否为存证的持有者
			ensure!(to_account_id == owner, Error::<T>::NotProofOwner);

			Self::deposit_event(Event::ClaimOwner(to_account_id, proof));

			Ok(().into())
		}
		
	}


}