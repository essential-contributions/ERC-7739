use super::curve_builder::{
    AssetBasedIntentCurve, AssetType, CurveFlags, CurveParameters, EvaluationType,
};
use ethers::{
    abi::{AbiDecode, AbiEncode},
    prelude::*,
};

#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct AssetBasedIntentSegment {
    call_data: Bytes,
    asset_releases: Vec<AssetBasedIntentCurve>,
    asset_requirements: Vec<AssetBasedIntentCurve>,
}

impl AssetBasedIntentSegment {
    pub fn new(call_data: Bytes) -> Self {
        Self {
            call_data,
            asset_releases: vec![],
            asset_requirements: vec![],
        }
    }

    pub fn add_asset_release_curve(
        &mut self,
        asset_contract: Address,
        asset_id: U256,
        asset_type: AssetType,
        curve_parameters: CurveParameters,
    ) -> &mut Self {
        let flags: u128 = AbiDecode::decode(
            CurveFlags::new(
                asset_type,
                curve_parameters.get_curve_type(),
                EvaluationType::ABSOLUTE,
            )
            .encode(),
        )
        .unwrap();
        let curve =
            AssetBasedIntentCurve::new(asset_id, asset_contract, flags, curve_parameters.into());

        self.asset_releases.push(curve);

        self
    }

    pub fn add_asset_requirement_curve(
        &mut self,
        asset_contract: Address,
        asset_id: U256,
        asset_type: AssetType,
        curve_parameters: CurveParameters,
        evaluation_type: EvaluationType,
    ) -> &mut Self {
        let flags: u128 = AbiDecode::decode(
            CurveFlags::new(
                asset_type,
                curve_parameters.get_curve_type(),
                evaluation_type,
            )
            .encode(),
        )
        .unwrap();
        let curve =
            AssetBasedIntentCurve::new(asset_id, asset_contract, flags, curve_parameters.into());
        self.asset_requirements.push(curve);

        self
    }
}
