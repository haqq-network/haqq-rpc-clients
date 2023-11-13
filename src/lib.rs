// include!("gen/mod.rs");
mod gen;
pub use gen::*;

pub type ClientChannel = tonic::transport::Channel;
pub use tonic;

use cosmos::base::v1beta1::Coin;

pub trait CoinExt {
    fn get_denom(&self, denom: impl Into<String>) -> Option<&Coin>;
}

impl CoinExt for Option<Coin> {
    fn get_denom(&self, denom: impl Into<String>) -> Option<&Coin> {
        match self {
            coin @ Some(Coin { denom: d, .. }) if d == &denom.into() => coin.as_ref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn coin_ext() {
        let coin = Coin {
            denom: "aISLM".to_string(),
            amount: "0".to_string(),
        };

        assert!(Some(coin).get_denom("aISLM").is_some())
    }
}
