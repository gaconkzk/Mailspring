mod account;
mod address;
mod settings;

mod identity;
mod feature_usage;

// TODO we also need
// mod folder (with localStatus)
// mod message (with ref to folder, )


// re-export for shorten model name
pub use self::account::Account;
pub use self::address::Address;
pub use self::settings::{Settings, MailClient, MailProtocolType };

pub use self::feature_usage::{ FeatureUsage, Feature };
pub use self::identity::Identity;