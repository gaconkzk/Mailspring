use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureUsage {
    #[serde(rename = "contact-profiles")]
    pub contact_profiles: Feature,
    #[serde(rename = "link-tracking")]
    pub link_tracking: Feature,
    #[serde(rename = "open-tracking")]
    pub open_tracking: Feature,
    #[serde(rename = "send-later")]
    pub send_later: Feature,
    #[serde(rename = "send-reminders")]
    pub send_reminders: Feature,
    pub snooze: Feature,
    #[serde(rename = "thread-sharing")]
    pub thread_sharing: Feature,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    #[serde(rename = "featureLimitName")]
    pub limit_name: String,
    pub period: String,
    pub quota: u8,
    #[serde(rename = "usedInPeriod")]
    pub used_in_period: u8,
}