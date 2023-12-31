use log::{debug, error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GumRoadError {
    success: bool,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GumRoadCard {
    visual: Option<String>,
    #[serde(rename = "type")]
    card_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GumRoadPurchase {
    seller_id: String,
    product_id: String,
    product_name: String,
    permalink: String,
    product_permalink: String,
    email: String,
    price: i32,
    gumroad_fee: i32,
    currency: String,
    quantity: i32,
    discover_fee_charged: bool,
    can_contact: bool,
    referrer: String,
    card: GumRoadCard,
    order_number: i32,
    sale_id: String,
    sale_timestamp: String,
    purchaser_id: String,
    subscription_id: String,
    variants: String,
    license_key: String,
    is_multiseat_license: bool,
    ip_country: String,
    recurrence: String,
    is_gift_receiver_purchase: bool,
    refunded: bool,
    disputed: bool,
    dispute_won: bool,
    id: String,
    created_at: String,
    custom_fields: Vec<String>,
    chargebacked: Option<bool>,
    subscription_ended_at: Option<String>,
    subscription_cancelled_at: Option<String>,
    subscription_failed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GumRoadVerifyResponse {
    success: bool,
    uses: i32,
    purchase: GumRoadPurchase,
}

pub async fn verify_license(
    license_key: String,
) -> Result<GumRoadVerifyResponse, async_graphql::Error> {
    let client = reqwest::Client::new();
    let req_body = serde_json::json!({
        "product_id": "BCA9WkIAJRaqBGV4tA7E6g==",
        "license_key": license_key,
        "increment_uses_count": false
    });

    let res = client
        .post("https://api.gumroad.com/v2/licenses/verify")
        .json(&req_body)
        .send()
        .await;

    match res {
        Ok(res) => {
            let body = res.text().await;
            debug!("Gumroad Response Body: {:?}", body);
            match body {
                Ok(body) => {
                    let gumroad_response = serde_json::from_str::<GumRoadVerifyResponse>(&body);
                    match gumroad_response {
                        Ok(gumroad_response) => {
                            debug!("Gumroad Response: {:?}", gumroad_response);
                            Ok(gumroad_response)
                        }
                        Err(_err) => {
                            let gumroad_error = serde_json::from_str::<GumRoadError>(&body);
                            match gumroad_error {
                                Ok(gumroad_error) => {
                                    error!("Gumroad Error: {:?}", gumroad_error);
                                    Err(async_graphql::Error::new(format!(
                                        "Gumroad Error: {:?}",
                                        gumroad_error.message
                                    )))
                                }
                                Err(err) => {
                                    error!("Error Parsing Gumroad Response: {}", err);
                                    Err(async_graphql::Error::new(format!(
                                        "Error Parsing Gumroad Response: {}",
                                        err
                                    )))
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("Error Parsing Gumroad Response: {}", err);
                    Err(async_graphql::Error::new(format!(
                        "Error Parsing Gumroad Response: {}",
                        err
                    )))
                }
            }
        }
        Err(err) => {
            error!("Error Verifying License: {}", err);
            Err(async_graphql::Error::new(format!(
                "Error Verifying License: {}",
                err
            )))
        }
    }
}
