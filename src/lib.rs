// include!("gen/mod.rs");

#[allow(clippy::all)]
mod gen;
pub use gen::*;

pub mod prelude {
    use core::fmt;
    use std::{num::ParseIntError, str::FromStr};

    use crate::cosmos::base::v1beta1::DecCoin;

    use super::*;
    use ethnum::U256;

    use cosmos::base::v1beta1::Coin;
    use tonic::metadata::{Ascii, MetadataValue};

    pub const HEIGHT_METADATA_KEY: &str = "x-cosmos-block-height";

    pub trait CoinExt<'a> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str>;
    }

    // impl<'a, I> CoinOptionExt<'a> for I
    // where
    //     I: AsRef<[Coin]>,
    // {
    //     fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a Coin> {
    //         let denom = denom.into();
    //         self.as_ref().iter().find(move |c| c.denom == denom)
    //     }
    // }

    impl<'a> CoinExt<'a> for Option<Coin> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str> {
            self.as_ref()
                .filter(|c| c.denom == denom.into())
                .map(|c| c.denom.as_str())
        }
    }

    impl<'a> CoinExt<'a> for Vec<Coin> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str> {
            let denom = denom.into();
            for coin in self.iter() {
                if coin.denom == denom {
                    return Some(coin.amount.as_str());
                }
            }
            None
        }
    }

    impl<'a> CoinExt<'a> for Vec<DecCoin> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str> {
            let denom = denom.into();
            for coin in self.iter() {
                if coin.denom == denom {
                    return Some(coin.amount.as_str());
                }
            }
            None
        }
    }

    pub trait AmountExt {
        fn to_u256(&self) -> Result<U256, ParseIntError>;
    }

    impl AmountExt for String {
        fn to_u256(&self) -> Result<U256, ParseIntError> {
            U256::from_str(self)
        }
    }

    impl<'a> AmountExt for &'a str {
        fn to_u256(&self) -> Result<U256, ParseIntError> {
            U256::from_str(self)
        }
    }

    impl AmountExt for Coin {
        fn to_u256(&self) -> Result<U256, ParseIntError> {
            self.amount.to_u256()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Denom {
        aISLM,
        ISLM,
        Other(String),
    }

    impl Denom {
        pub fn conversion_pair(&self) -> Option<(Self, U256, U256)> {
            match self {
                Self::aISLM => Some((Self::ISLM, U256::new(1), U256::pow(U256::new(10), 18))),
                Self::ISLM => Some((Self::aISLM, U256::pow(U256::new(10), 18), U256::new(1))),
                Self::Other(_) => None,
            }
        }
    }

    impl fmt::Display for Denom {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::aISLM => write!(f, "aISLM"),
                Self::ISLM => write!(f, "ISLM"),
                Denom::Other(d) => write!(f, "{}", d),
            }
        }
    }

    pub trait CosmosMessage {
        fn with_height(self, height: MetadataValue<Ascii>) -> tonic::Request<Self>
        where
            Self: Sized,
        {
            let mut req = tonic::Request::new(self);
            req.metadata_mut().insert(HEIGHT_METADATA_KEY, height);
            req
        }
    }

    impl<T> CosmosMessage for T {}

    pub trait CosmosResponse<T> {
        fn get_height(&self) -> Option<&MetadataValue<Ascii>>;
    }

    impl<T> CosmosResponse<T> for tonic::Response<T> {
        fn get_height(&self) -> Option<&MetadataValue<Ascii>> {
            self.metadata().get(HEIGHT_METADATA_KEY)
        }
    }
}

#[cfg(test)]
mod test {
    use super::cosmos::base::v1beta1::Coin;
    use super::prelude::*;
    use ethnum::U256;
    use std::str::FromStr;

    #[test]
    fn denom() {
        let oneislm_in_aislm =
            U256::from_str(&format!("1{}", (0..18).map(|_| "0").collect::<String>())).unwrap();
        let oneislm_in_islm = U256::from_str("1").unwrap();

        let (denom, mul, div) = Denom::aISLM.conversion_pair().unwrap();
        assert_eq!(denom, Denom::ISLM);
        assert_eq!(oneislm_in_islm, oneislm_in_aislm * mul / div);

        let (denom, mul, div) = Denom::ISLM.conversion_pair().unwrap();
        assert_eq!(denom, Denom::aISLM);
        assert_eq!(oneislm_in_aislm, oneislm_in_islm * mul / div);
    }

    #[test]
    fn coin_option_ext() {
        let coin = Coin {
            denom: "aISLM".to_string(),
            amount: "0".to_string(),
        };

        assert!(vec![coin.clone()].get_denom("aISLM").is_some());
        assert!(Some(coin).get_denom("aISLM").is_some());
    }

    #[test]
    fn coin_ext() {
        let mut coin = Coin {
            denom: "ISLM".to_string(),
            amount: "1.2345".to_string(),
        };

        assert!(coin.to_u256().is_err());

        coin.amount = "12345".to_string();
        assert_eq!(coin.to_u256().unwrap(), U256::from_str("12345").unwrap());

        assert_eq!(
            "12345".to_string().to_u256().unwrap(),
            U256::from_str("12345").unwrap()
        );
        assert_eq!("12345".to_u256().unwrap(), U256::from_str("12345").unwrap());
    }
}
