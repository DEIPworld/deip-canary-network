use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct PayloadDetails<AccountId> {
    pub owner: AccountId,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum PayloadAssetId<AssetId, ClassId> {
    Ft(AssetId),
    Nft(ClassId),
}
