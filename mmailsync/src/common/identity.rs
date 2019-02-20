use serde::{ Serialize, Deserialize };
use crate::common::FeatureUsage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "featureUsage")]
    pub feature_usage: FeatureUsage,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub object: String,
    #[serde(rename = "stripeCustomerId")]
    pub stripe_customer_id: String,
    #[serde(rename = "stripePeriodEnd")]
    pub stripe_period_end: String,
    #[serde(rename = "stripePlan")]
    pub stripe_plan: String,
    #[serde(rename = "stripePlanEffective")]
    pub stripe_plan_effective: String,
    pub token: String,
}