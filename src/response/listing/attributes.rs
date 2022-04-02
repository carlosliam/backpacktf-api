use num_enum::TryFromPrimitiveError;
use serde::{Serialize, Deserialize};
use tf2_enum::{StrangePart, Rarity};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticleAttribute {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub image_url: String,
    pub r#type: String,
}

// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct PaintAttribute {
//     pub id: u32,
//     pub name: String,
//     pub color: String,
// }

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextureAttribute {
    pub id: u32,
    pub item_defindex: Option<u32>,
    pub rarity: Rarity,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillEaterTypeAttribute {
    pub id: Option<u32>,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillEaterAttribute {
    pub score: u64,
    pub kill_eater: KillEaterTypeAttribute,
}

impl KillEaterAttribute {
    
    pub fn try_into_strange_part(&self) -> Result<StrangePart, TryFromPrimitiveError<StrangePart>> {
        StrangePart::try_from(self.kill_eater.id.unwrap_or_default() as u8)
    }
}

// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct QualityAttribute {
//     pub id: u32,
//     pub name: String,
//     pub color: String,
// }

// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct WearTierAttribute {
//     pub id: u32,
//     pub name: String,
//     pub short: String,
// }