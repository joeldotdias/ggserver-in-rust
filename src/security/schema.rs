use serde::{ Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct SecurityProfileDto {
    pub name: String,
    #[serde(rename= "badgeId")]
    pub badge_id: String,
    #[serde(rename= "phoneNo")]
    pub phone_no: String,
    #[serde(rename= "pfpUrl")]
    pub pfp_url: Option<String>,
    pub society: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct VisitorSecurityDto {
    #[serde(rename="visitorId")]
    pub visitor_id: i32,
    pub name: String,
    #[serde(rename="hostFlat")]
    pub host_flat: i32,
    #[serde(rename="hostBuilding")]
    pub host_building: String,
    pub society: String,
    pub code: String
}

#[derive(Deserialize, Debug)]
pub struct VerifiedVisitorParams {
    #[serde(rename="visitorId")]
    pub visitor_id: i32
}

// #[derive(FromRow, Debug)]
// pub struct VerifiedVisitorDetails {
//     pub name: String,
//     pub phone_no: String,
//     pub host_flat: i32,
//     pub host_building: String,
//     pub host_society: String,
// }

#[derive(Debug, FromRow)]
pub struct VerifiedVisitorDetails {
    pub name: String,
    pub phone_no: String,
    pub resident_id: i32,
}


#[derive(Deserialize, Debug)]
pub struct UpdateSecurityProfileSchema {
    pub name: String,
    #[serde(rename="aboutMe")]
    pub about_me: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdatePfpParams {
    pub pfpUrl: String
}