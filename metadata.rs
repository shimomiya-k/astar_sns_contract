use ink_env::{AccountId, Hash};
use ink_prelude::vec::Vec;
use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};

// 投稿用の構造体
#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PartialEq, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Post {
    pub name: Hash,
    pub user_id: AccountId,
    pub created_time: Hash,
    pub img_url: Hash,
    pub user_img_url: Hash,
    pub description: Hash,
    pub num_of_likes: u128,
    pub post_id: u128,
}

// プロフィール用の構造体
#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PartialEq, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Profile {
    pub following_list: Vec<AccountId>,
    pub follower_list: Vec<AccountId>,
    pub friend_list: Vec<AccountId>,
    pub user_id: AccountId,
    pub name: Option<Hash>,
    pub img_url: Option<Hash>,
    pub message_list_id_list: Vec<u128>,
    pub post_id_list: Vec<u128>,
}

// メッセージ用の構造体
#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PartialEq, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Message {
    pub message: Hash,
    pub sender_id: AccountId,
    pub created_time: Hash,
}
