use sp_runtime::RuntimeDebug;
use sp_std::convert::From;
use sp_std::convert::Into;
use sp_std::convert::TryFrom;
use sp_std::convert::TryInto;
use sp_std::fmt;
use sp_std::str;

use stellar::types::AssetAlphaNum12;
use stellar::types::AssetAlphaNum4;
use stellar::PublicKey;
use substrate_stellar_sdk as stellar;

use codec::{Decode, Encode};

pub type Bytes4 = [u8; 4];
pub type Bytes12 = [u8; 12];
pub type AssetIssuer = [u8; 32];

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
    Native,
    StellarNative,
    AlphaNum4 { code: Bytes4, issuer: AssetIssuer },
    AlphaNum12 { code: Bytes12, issuer: AssetIssuer },
}

impl Default for CurrencyId {
    fn default() -> Self {
        CurrencyId::Native
    }
}

impl TryFrom<(&str, AssetIssuer)> for CurrencyId {
    type Error = &'static str;

    fn try_from(value: (&str, AssetIssuer)) -> Result<Self, Self::Error> {
        let slice = value.0;
        let issuer = value.1;
        if slice.len() <= 4 {
            let mut code: Bytes4 = [0; 4];
            code[..slice.len()].copy_from_slice(slice.as_bytes());
            Ok(CurrencyId::AlphaNum4 { code, issuer })
        } else if slice.len() > 4 && slice.len() <= 12 {
            let mut code: Bytes12 = [0; 12];
            code[..slice.len()].copy_from_slice(slice.as_bytes());
            Ok(CurrencyId::AlphaNum12 { code, issuer })
        } else {
            Err("More than 12 bytes not supported")
        }
    }
}

impl From<stellar::Asset> for CurrencyId {
    fn from(asset: stellar::Asset) -> Self {
        match asset {
            stellar::Asset::AssetTypeNative => CurrencyId::StellarNative,
            stellar::Asset::AssetTypeCreditAlphanum4(asset_alpha_num4) => CurrencyId::AlphaNum4 {
                code: asset_alpha_num4.asset_code,
                issuer: asset_alpha_num4.issuer.as_binary().clone(),
            },
            stellar::Asset::AssetTypeCreditAlphanum12(asset_alpha_num12) => {
                CurrencyId::AlphaNum12 {
                    code: asset_alpha_num12.asset_code,
                    issuer: asset_alpha_num12.issuer.as_binary().clone(),
                }
            }
        }
    }
}

impl TryInto<stellar::Asset> for CurrencyId {
    type Error = &'static str;

    fn try_into(self) -> Result<stellar::Asset, Self::Error> {
        match self {
            Self::Native => Err("PEN token not defined in the Stellar world."),
            Self::StellarNative => Ok(stellar::Asset::native()),
            Self::AlphaNum4 { code, issuer } => {
                Ok(stellar::Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                    asset_code: code,
                    issuer: PublicKey::PublicKeyTypeEd25519(issuer),
                }))
            }
            Self::AlphaNum12 { code, issuer } => {
                Ok(stellar::Asset::AssetTypeCreditAlphanum12(AssetAlphaNum12 {
                    asset_code: code,
                    issuer: PublicKey::PublicKeyTypeEd25519(issuer),
                }))
            }
        }
    }
}

impl fmt::Display for CurrencyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native => write!(f, "PEN"),
            Self::StellarNative => write!(f, "XLM"),
            Self::AlphaNum4 { code, issuer } => {
                write!(
                    f,
                    "{{ code: {}, issuer: {} }}",
                    str::from_utf8(code).unwrap(),
                    str::from_utf8(issuer).unwrap()
                )
            }
            Self::AlphaNum12 { code, issuer } => {
                write!(
                    f,
                    "{{ code: {}, issuer: {} }}",
                    str::from_utf8(code).unwrap(),
                    str::from_utf8(issuer).unwrap()
                )
            }
        }
    }
}
