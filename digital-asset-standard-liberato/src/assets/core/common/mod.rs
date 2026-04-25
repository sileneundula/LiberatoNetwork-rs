use fixedstr::{str64, str256};

pub struct MuscariaDigitalAsset {
    asset_id: Option<u64>,
    asset_address: str256, // Muscaria Address For Digital Asset
    asset_standard: Option<str64>, // Muscaria Standard For Digital Asset
}

pub enum MuscariaDigitalAssetStandards {
    MuscariaDigitalAsset,
    MuscariaDigitalAssetV2,
    MuscariaDigitalAssetV3,
}