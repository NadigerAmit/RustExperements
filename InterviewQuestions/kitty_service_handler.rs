use serde::{Deserialize, Serialize};

use jsonrpsee::http_client::HttpClientBuilder;

use crate::kitty;
use crate::kitty::TransactionResponse;
use sled::Db;
use sp_core::H256;

/// The default RPC endpoint for the wallet to connect to
//const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use crate::get_blockchain_node_endpoint;
//use crate::original_get_db;
use crate::convert_output_ref_from_string;

use axum::{http::HeaderMap, Extension, Json};
use std::sync::Arc;
use tokio::sync::Mutex;

use std::convert::Infallible;

use runtime::{
    kitties::KittyData, tradable_kitties::TradableKittyData, OuterVerifier, Transaction,
};
use tuxedo_core::types::Output;
use tuxedo_core::types::OutputRef;

#[derive(Debug, Deserialize)]
pub struct CreateKittyRequest {
    pub name: String,
    pub owner_public_key: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateKittyResponse {
    pub message: String,
    pub kitty: Option<KittyData>, // Add any additional fields as needed
}

pub async fn create_kitty(
    body: Json<CreateKittyRequest>,
) -> Result<Json<CreateKittyResponse>, Infallible> {
    println!("create_kitties called ");
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));
    //let db = sync_and_get_db().await.expect("Error");

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(CreateKittyResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                kitty: None,
            }));
        }
    };

    let pb_key_bytes = match hex::decode(body.owner_public_key.as_str()) {
        Ok(p) => p,
        Err(_) => {
            return Ok(Json(CreateKittyResponse {
                message: format!("Invalid in public key, Can't decode"),
                kitty: None,
            }));
        }
    };

    // Convert the bytes to H256
    let public_key_h256 = H256::from_slice(&pb_key_bytes);

    match kitty::create_kitty(&client, body.name.to_string(), public_key_h256).await {
        Ok(Some(created_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = CreateKittyResponse {
                message: format!("Kitty created successfully"),
                kitty: Some(created_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(CreateKittyResponse {
            message: format!("Kitty creation failed: No data returned"),
            kitty: None,
        })),
        Err(err) => Ok(Json(CreateKittyResponse {
            message: format!("Error creating kitty: {:?}", err),
            kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// Get kitty by DNA
////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKittyByDnaResponse {
    pub message: String,
    pub kitty: Option<KittyData>,
}

pub async fn get_kitty_by_dna(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetKittyByDnaResponse> {
    println!("Headers map = {:?}", headers);
    let dna_header = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");
    let db = db.lock().await;
    let mut found_kitty: Option<(KittyData, OutputRef)> = None;

    if let Ok(Some((kitty_info, out_ref))) =
        crate::sync::get_kitty_from_local_db_based_on_dna(&db, dna_header)
    {
        found_kitty = Some((kitty_info, out_ref));
    }

    let response = match found_kitty {
        Some((kitty_info, _)) => GetKittyByDnaResponse {
            message: format!("Success: Found Kitty with DNA {:?}", dna_header),
            kitty: Some(kitty_info),
        },
        None => GetKittyByDnaResponse {
            message: format!("Error: Can't find Kitty with DNA {:?}", dna_header),
            kitty: None,
        },
    };

    Json(response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTdKittyByDnaResponse {
    pub message: String,
    pub td_kitty: Option<TradableKittyData>,
}

pub async fn get_td_kitty_by_dna(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTdKittyByDnaResponse> {
    println!("Headers map = in td kitty {:?}", headers);
    let dna_header = headers
        .get("td-kitty-dna")
        .expect("Td-Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Td-Kitty DNA header");
    let db = db.lock().await;
    let mut found_td_kitty: Option<(TradableKittyData, OutputRef)> = None;

    if let Ok(Some((td_kitty_info, out_ref))) =
        crate::sync::get_tradable_kitty_from_local_db_based_on_dna(&db, dna_header)
    {
        found_td_kitty = Some((td_kitty_info, out_ref));
    }

    let response = match found_td_kitty {
        Some((kitty_info, _)) => GetTdKittyByDnaResponse {
            message: format!("Success: Found Tradable Kitty with DNA {:?}", dna_header),
            td_kitty: Some(kitty_info),
        },
        None => GetTdKittyByDnaResponse {
            message: format!("Error: Can't find Tradable Kitty with DNA {:?}", dna_header),
            td_kitty: None,
        },
    };

    Json(response)
}

////////////////////////////////////////////////////////////////////
// Get all kitty List
////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerKitty {
    pub owner_pub_key: H256,
    pub kitty: KittyData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllKittiesResponse {
    pub message: String,
    pub owner_kitty_list: Option<Vec<OwnerKitty>>,
}

pub async fn get_all_kitty_list(
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetAllKittiesResponse> {
    let db = db.lock().await;

    match crate::sync::get_all_kitties_from_local_db(&db) {
        Ok(all_kitties) => {
            let kitty_list: Vec<OwnerKitty> = all_kitties
                .map(|(owner, kitty_data)| OwnerKitty {
                    owner_pub_key: owner,
                    kitty: kitty_data,
                })
                .collect();

            if !kitty_list.is_empty() {
                return Json(GetAllKittiesResponse {
                    message: format!("Success: Found Kitties"),
                    owner_kitty_list: Some(kitty_list),
                });
            }
        }
        Err(_) => {
            return Json(GetAllKittiesResponse {
                message: format!("Error: Can't find Kitties"),
                owner_kitty_list: None,
            });
        }
    }

    Json(GetAllKittiesResponse {
        message: format!("Error: Can't find Kitties"),
        owner_kitty_list: None,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerTradableKitty {
    pub owner_pub_key: H256,
    pub td_kitty: TradableKittyData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllTdKittiesResponse {
    pub message: String,
    pub td_kitty_list: Option<Vec<OwnerTradableKitty>>,
}

pub async fn get_all_td_kitty_list(
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetAllTdKittiesResponse> {
    let db = db.lock().await;

    match crate::sync::get_all_tradable_kitties_from_local_db(&db) {
        Ok(owned_kitties) => {
            let tradable_kitty_list: Vec<OwnerTradableKitty> = owned_kitties
                .map(|(owner, td_kitty_data)| OwnerTradableKitty {
                    owner_pub_key: owner,
                    td_kitty: td_kitty_data,
                })
                .collect();

            if !tradable_kitty_list.is_empty() {
                return Json(GetAllTdKittiesResponse {
                    message: format!("Success: Found TradableKitties"),
                    td_kitty_list: Some(tradable_kitty_list),
                });
            }
        }
        Err(_) => {
            return Json(GetAllTdKittiesResponse {
                message: format!("Error: Can't find TradableKitties"),
                td_kitty_list: None,
            });
        }
    }

    Json(GetAllTdKittiesResponse {
        message: format!("Error: Can't find Kitties"),
        td_kitty_list: None,
    })
}
////////////////////////////////////////////////////////////////////
// Get owned kitties
////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedKittiesResponse {
    pub message: String,
    pub kitty_list: Option<Vec<KittyData>>,
}
use std::str::FromStr;
pub async fn get_owned_kitty_list(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetOwnedKittiesResponse> {
    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = match H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    ) {
        Ok(public_key_h256) => public_key_h256,
        Err(err) => {
            // Return an error response or handle the error appropriately
            return Json(GetOwnedKittiesResponse {
                message: format!("Failed to extract the H256 from public key {:?}", err),
                kitty_list: None,
            });
        }
    };

    //let db = original_get_db().await.expect("Error");
    // let db = get_db().await.expect("Error");
    let db = db.lock().await;

    match crate::sync::get_owned_kitties_from_local_db(&db, &public_key_h256) {
        Ok(owned_kitties) => {
            let kitty_list: Vec<KittyData> = owned_kitties.map(|(_, kitty, _)| kitty).collect();

            if !kitty_list.is_empty() {
                return Json(GetOwnedKittiesResponse {
                    message: format!("Success: Found Kitties"),
                    kitty_list: Some(kitty_list),
                });
            }
        }
        Err(_) => {
            return Json(GetOwnedKittiesResponse {
                message: format!("Error: Can't find Kitties"),
                kitty_list: None,
            });
        }
    }

    Json(GetOwnedKittiesResponse {
        message: format!("Error: Can't find Kitties"),
        kitty_list: None,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedTdKittiesResponse {
    pub message: String,
    pub td_kitty_list: Option<Vec<TradableKittyData>>,
}

pub async fn get_owned_td_kitty_list(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetOwnedTdKittiesResponse> {
    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");
    let public_key_h256 = match H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    ) {
        Ok(public_key_h256) => public_key_h256,
        Err(err) => {
            // Return an error response or handle the error appropriately
            return Json(GetOwnedTdKittiesResponse {
                message: format!("Failed to extract the H256 from public key {:?}", err),
                td_kitty_list: None,
            });
        }
    };
    let db = db.lock().await;

    match crate::sync::get_owned_tradable_kitties_from_local_db(&db, &public_key_h256) {
        Ok(owned_kitties) => {
            let tradable_kitty_list: Vec<TradableKittyData> =
                owned_kitties.map(|(_, kitty, _)| kitty).collect();

            if !tradable_kitty_list.is_empty() {
                return Json(GetOwnedTdKittiesResponse {
                    message: format!("Success: Found TradableKitties"),
                    td_kitty_list: Some(tradable_kitty_list),
                });
            }
        }
        Err(_) => {
            return Json(GetOwnedTdKittiesResponse {
                message: format!("Error: Can't find TradableKitties"),
                td_kitty_list: None,
            });
        }
    }

    Json(GetOwnedTdKittiesResponse {
        message: format!("Error: Can't find td Kitties"),
        td_kitty_list: None,
    })
}

////////////////////////////////////////////////////////////////////
// Common structures and functions
////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTxnAndUtxoListForList {
    pub message: String,
    pub transaction: Option<Transaction>,
    pub input_utxo_list: Option<Vec<Output<OuterVerifier>>>,
    pub encoded: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct SignedTxnRequest {
    pub signed_transaction: Transaction,
}

async fn create_response(
    res: Option<TransactionResponse>,
    message: String,
) -> Json<GetTxnAndUtxoListForList> {
    match res {
        Some(res) => {
            let txn = res.transaction;
            let client_result = HttpClientBuilder::default()
                .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));
            let client = match client_result {
                Ok(client) => client,
                Err(err) => {
                    return Json(GetTxnAndUtxoListForList {
                        message: format!("Error creating HTTP client: {:?}", err),
                        transaction: None,
                        input_utxo_list: None,
                        encoded: vec![],
                    });
                }
            };
            let utxo_list = kitty::create_inpututxo_list(&mut txn.clone(), &client).await;
            Json(GetTxnAndUtxoListForList {
                message,
                transaction: Some(txn),
                input_utxo_list: utxo_list.expect("Cant crate the Utxo List"),
                encoded: res.encoded,
            })
        }
        None => Json(GetTxnAndUtxoListForList {
            message,
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
    }
}

////////////////////////////////////////////////////////////////////
// List kitty for Sale
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_list_kitty_for_sale(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    println!("Headers map = {:?}", headers);

    let dna_header = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");

    let price_header = headers.get("kitty-price").expect("Kitty price is missing");

    let price_number: u128 = price_header
        .to_str()
        .expect("Failed to parse priceheader")
        .parse()
        .expect("ailed to parse price number as u128");

    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    );

    //let db = original_get_db().await.expect("Error");
    let db = db.lock().await;

    match kitty::create_txn_for_list_kitty(&db, dna_header, price_number, public_key_h256.unwrap())
        .await
    {
        Ok(txn) => {
            create_response(
                txn,
                "List kitty for Sale txn created successfully".to_string(),
            )
            .await
        }
        Err(err) => {
            create_response(
                None,
                format!("Error!! List kitty for sale txn creation: {:?}", err),
            )
            .await
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListKittyForSaleResponse {
    pub message: String,
    pub td_kitty: Option<TradableKittyData>, // Add any additional fields as needed
}

pub async fn list_kitty_for_sale(
    body: Json<SignedTxnRequest>,
) -> Result<Json<ListKittyForSaleResponse>, Infallible> {
    println!("List kitties for sale is called {:?}", body);
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(ListKittyForSaleResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                td_kitty: None,
            }));
        }
    };

    match kitty::list_kitty_for_sale(&body.signed_transaction, &client).await {
        Ok(Some(listed_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = ListKittyForSaleResponse {
                message: format!("Kitty listed for sale successfully"),
                td_kitty: Some(listed_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(ListKittyForSaleResponse {
            message: format!("Kitty listing forsale  failed: No data returned"),
            td_kitty: None,
        })),
        Err(err) => Ok(Json(ListKittyForSaleResponse {
            message: format!("Error listing forsale: {:?}", err),
            td_kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// De-list kitty from Sale
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_delist_kitty_from_sale(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    // create_tx_for_list_kitty
    println!("Headers map = {:?}", headers);
    let dna_header = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");

    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    );

    //let db = original_get_db().await.expect("Error");
    let db = db.lock().await;

    match kitty::create_txn_for_delist_kitty(&db, dna_header, public_key_h256.unwrap()).await {
        Ok(txn) => {
            create_response(
                txn,
                "List kitty for Sale txn created successfully".to_string(),
            )
            .await
        }
        Err(err) => {
            create_response(
                None,
                format!("Error!! List kitty for sale txn creation: {:?}", err),
            )
            .await
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DelistKittyFromSaleResponse {
    pub message: String,
    pub kitty: Option<KittyData>,
}
pub async fn delist_kitty_from_sale(
    body: Json<SignedTxnRequest>,
) -> Result<Json<DelistKittyFromSaleResponse>, Infallible> {
    println!("List kitties for sale is called {:?}", body);
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(DelistKittyFromSaleResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                kitty: None,
            }));
        }
    };

    match kitty::delist_kitty_from_sale(&body.signed_transaction, &client).await {
        Ok(Some(delisted_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = DelistKittyFromSaleResponse {
                message: format!("Kitty delisted from sale successfully"),
                kitty: Some(delisted_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(DelistKittyFromSaleResponse {
            message: format!("Kitty delisting from sale  failed: No data returned"),
            kitty: None,
        })),
        Err(err) => Ok(Json(DelistKittyFromSaleResponse {
            message: format!("Error delisting from sale: {:?}", err),
            kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// Update kitty name
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_kitty_name_update(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    println!("Headers map = {:?}", headers);
    let dna_header = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");

    let new_name_header = headers
        .get("kitty-new-name")
        .expect("Kitty name is missing");

    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    );

    //let db = original_get_db().await.expect("Error");
    let db = db.lock().await;

    match kitty::create_txn_for_kitty_name_update(
        &db,
        dna_header,
        new_name_header
            .to_str()
            .expect("Failed to parse name header")
            .to_string(),
        public_key_h256.unwrap(),
    )
    .await
    {
        Ok(txn) => {
            create_response(
                txn,
                "Kitty name update txn created successfully".to_string(),
            )
            .await
        }
        Err(err) => {
            create_response(
                None,
                format!("Error!! Kitty name update txn creation: {:?}", err),
            )
            .await
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateKittyNameResponse {
    pub message: String,
    pub kitty: Option<KittyData>,
}
pub async fn update_kitty_name(
    body: Json<SignedTxnRequest>,
) -> Result<Json<UpdateKittyNameResponse>, Infallible> {
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(UpdateKittyNameResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                kitty: None,
            }));
        }
    };

    match kitty::update_kitty_name(&body.signed_transaction, &client).await {
        Ok(Some(updated_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = UpdateKittyNameResponse {
                message: format!("Kitty name updated successfully"),
                kitty: Some(updated_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(UpdateKittyNameResponse {
            message: format!("Kitty name update failed: No data returned"),
            kitty: None,
        })),
        Err(err) => Ok(Json(UpdateKittyNameResponse {
            message: format!("Error!! Kitty name update: {:?}", err),
            kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// Update tradable kitty name
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_td_kitty_name_update(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    println!("Headers map = {:?}", headers);
    let dna_header = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");
    //let db = original_get_db().await.expect("Error");
    let db = db.lock().await;

    let new_name_header = headers
        .get("kitty-new-name")
        .expect("Kitty name is missing");

    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    );

    match kitty::create_txn_for_td_kitty_name_update(
        &db,
        dna_header,
        new_name_header
            .to_str()
            .expect("Failed to parse name header")
            .to_string(),
        public_key_h256.unwrap(),
    )
    .await
    {
        Ok(txn) => {
            create_response(
                txn,
                "Td Kitty name update txn created successfully".to_string(),
            )
            .await
        }
        Err(err) => {
            create_response(
                None,
                format!("Error!! Td-Kitty name update txn creation: {:?}", err),
            )
            .await
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateTddKittyNameResponse {
    pub message: String,
    pub td_kitty: Option<TradableKittyData>,
}
pub async fn update_td_kitty_name(
    body: Json<SignedTxnRequest>,
) -> Result<Json<UpdateTddKittyNameResponse>, Infallible> {
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(UpdateTddKittyNameResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                td_kitty: None,
            }));
        }
    };

    match kitty::update_td_kitty_name(&body.signed_transaction, &client).await {
        Ok(Some(updated_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = UpdateTddKittyNameResponse {
                message: format!("Td-Kitty name updated successfully"),
                td_kitty: Some(updated_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(UpdateTddKittyNameResponse {
            message: format!("Td-Kitty name update failed: No data returned"),
            td_kitty: None,
        })),
        Err(err) => Ok(Json(UpdateTddKittyNameResponse {
            message: format!("Error!! Td-Kitty name update: {:?}", err),
            td_kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// Update td-kitty price
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_td_kitty_price_update(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    println!("Headers map = {:?}", headers);
    let dna_header = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");

    let price_header = headers.get("kitty-price").expect("Kitty price is missing");

    // Convert the block number to the appropriate type if needed
    let price_number: u128 = price_header
        .to_str()
        .expect("Failed to parse priceheader to str")
        .parse()
        .expect("Failed to parse priceheader to u128");

    //let db = sync_and_get_db().await.expect("Error");
    let db = db.lock().await;

    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    );

    match kitty::create_txn_for_td_kitty_price_update(
        &db,
        dna_header,
        price_number,
        public_key_h256.unwrap(),
    )
    .await
    {
        Ok(Some(res)) => {
            // Convert created_kitty to JSON and include it in the response
            let txn = res.transaction;
            let client_result = HttpClientBuilder::default()
                .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));
            let client = match client_result {
                Ok(client) => client,
                Err(err) => {
                    return Json(GetTxnAndUtxoListForList {
                        message: format!("Error creating HTTP client: {:?}", err),
                        transaction: None,
                        input_utxo_list: None,
                        encoded: vec![],
                    });
                }
            };
            let utxo_list = kitty::create_inpututxo_list(&mut txn.clone(), &client).await;

            let response = GetTxnAndUtxoListForList {
                message: format!("Kitty name update txn created successfully"),
                transaction: Some(txn),
                input_utxo_list: utxo_list.expect("Cant crate the Utxo List"),
                encoded: res.encoded,
            };
            Json(response)
        }
        Ok(None) => Json(GetTxnAndUtxoListForList {
            message: format!("Kitty name update txn creation failed: No input returned"),
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
        Err(err) => Json(GetTxnAndUtxoListForList {
            message: format!("Error!! Kitty name update txn creation: {:?}", err),
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateTdKittyPriceResponse {
    pub message: String,
    pub td_kitty: Option<TradableKittyData>,
}

pub async fn update_td_kitty_price(
    body: Json<SignedTxnRequest>,
) -> Result<Json<UpdateTdKittyPriceResponse>, Infallible> {
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(UpdateTdKittyPriceResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                td_kitty: None,
            }));
        }
    };

    match kitty::update_td_kitty_price(&body.signed_transaction, &client).await {
        Ok(Some(updated_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = UpdateTdKittyPriceResponse {
                message: format!("Kitty price updated successfully"),
                td_kitty: Some(updated_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(UpdateTdKittyPriceResponse {
            message: format!("Kitty price update failed: No data returned"),
            td_kitty: None,
        })),
        Err(err) => Ok(Json(UpdateTdKittyPriceResponse {
            message: format!("Error in kitty price update: {:?}", err),
            td_kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// Breed kitty
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_breed_kitty(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    println!("Headers map = {:?}", headers);
    let mom_dna = headers
        .get("mom-dna")
        .expect("MOM DNA header is missing")
        .to_str()
        .expect("Failed to parse MOM DNA header");

    let dad_dna = headers
        .get("dad-dna")
        .expect("Dad DNA header is missing")
        .to_str()
        .expect("Failed to parse Dad DNA header");

    let child_kitty_name = headers
        .get("child-kitty-name")
        .expect("Child Kitty name is missing");

    //let db = sync_and_get_db().await.expect("Error");
    let db = db.lock().await;

    let public_key_header = headers
        .get("owner_public_key")
        .expect("public_key_header is missing");

    let public_key_h256 = H256::from_str(
        public_key_header
            .to_str()
            .expect("Failed to convert to H256"),
    );

    match kitty::create_txn_for_breed_kitty(
        &db,
        mom_dna,
        dad_dna,
        child_kitty_name
            .to_str()
            .expect("Failed to parse name header")
            .to_string(),
        public_key_h256.unwrap(),
    )
    .await
    {
        Ok(Some(res)) => {
            // Convert created_kitty to JSON and include it in the response
            let txn = res.transaction;
            let client_result = HttpClientBuilder::default()
                .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));
            let client = match client_result {
                Ok(client) => client,
                Err(err) => {
                    return Json(GetTxnAndUtxoListForList {
                        message: format!("Error creating HTTP client: {:?}", err),
                        transaction: None,
                        input_utxo_list: None,
                        encoded: vec![],
                    });
                }
            };
            let utxo_list = kitty::create_inpututxo_list(&mut txn.clone(), &client).await;

            let response = GetTxnAndUtxoListForList {
                message: format!("Kitty name update txn created successfully"),
                transaction: Some(txn),
                input_utxo_list: utxo_list.expect("Cant crate the Utxo List"),
                encoded: res.encoded,
            };
            Json(response)
        }
        Ok(None) => Json(GetTxnAndUtxoListForList {
            message: format!("Kitty name update txn creation failed: No input returned"),
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
        Err(err) => Json(GetTxnAndUtxoListForList {
            message: format!("Error!! Kitty name update txn creation: {:?}", err),
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
    }
}

#[derive(Debug, Serialize)]
pub struct BreedKittyResponse {
    pub message: String,
    pub mom_kitty: Option<KittyData>,
    pub dad_kitty: Option<KittyData>,
    pub child_kitty: Option<KittyData>,
}

pub async fn breed_kitty(
    body: Json<SignedTxnRequest>,
) -> Result<Json<BreedKittyResponse>, Infallible> {
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(BreedKittyResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                mom_kitty: None,
                dad_kitty: None,
                child_kitty: None,
            }));
        }
    };

    match kitty::breed_kitty(&body.signed_transaction, &client).await {
        Ok(Some(kitty_family)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = BreedKittyResponse {
                message: format!("Kitty breeding done successfully"),
                mom_kitty: Some(kitty_family[0].clone()),
                dad_kitty: Some(kitty_family[1].clone()),
                child_kitty: Some(kitty_family[2].clone()),
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(BreedKittyResponse {
            message: format!("Kitty breeding failed: No data returned"),
            mom_kitty: None,
            dad_kitty: None,
            child_kitty: None,
        })),
        Err(err) => Ok(Json(BreedKittyResponse {
            message: format!("Error in kitty breed: {:?}", err),
            mom_kitty: None,
            dad_kitty: None,
            child_kitty: None,
        })),
    }
}

////////////////////////////////////////////////////////////////////
// Buy kitty
////////////////////////////////////////////////////////////////////

pub async fn get_txn_and_inpututxolist_for_buy_kitty(
    headers: HeaderMap,
    Extension(db): Extension<Arc<Mutex<Db>>>,
) -> Json<GetTxnAndUtxoListForList> {
    println!("Headers map = {:?}", headers);

    let input_coins: Vec<OutputRef> = headers
        .get_all("input-coins")
        .iter()
        // Convert each coin string to an OutputRef, filter out errors
        .filter_map(|header| {
            let coin_str = header.to_str().unwrap_or_default();
            match convert_output_ref_from_string(coin_str) {
                Ok(output_ref) => Some(output_ref),
                Err(err) => {
                    // Print error message and skip this coin
                    eprintln!("Error converting input coin: {}", err);
                    None
                }
            }
        })
        .collect();
    println!("Input coins: {:?}", input_coins);
    let output_amount: Vec<u128> = headers.get("output_amount").map_or_else(
        || Vec::new(),
        |header| {
            header
                .to_str()
                .unwrap_or_default()
                .split(',')
                .filter_map(|amount_str| amount_str.parse().ok())
                .collect()
        },
    );
    // Use the converted coins Vec<OutputRef> as needed
    println!("output_amount: {:?}", output_amount);

    let kitty_dna = headers
        .get("kitty-dna")
        .expect("Kitty DNA header is missing")
        .to_str()
        .expect("Failed to parse Kitty DNA header");

    let db = db.lock().await;

    let buyer_public_key = headers
        .get("buyer_public_key")
        .expect("buyer_public_key is missing");

    let buyer_public_key_h256 = H256::from_str(
        buyer_public_key
            .to_str()
            .expect("Failed to convert buyer_public_keyto H256"),
    );

    let seller_public_key = headers
        .get("seller_public_key")
        .expect("seller_public_key is missing");

    let seller_public_key_h256 = H256::from_str(
        seller_public_key
            .to_str()
            .expect("Failed to convert seller_public_key to H256"),
    );

    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Json(GetTxnAndUtxoListForList {
                message: format!("Error creating HTTP client: {:?}", err),
                transaction: None,
                input_utxo_list: None,
                encoded: vec![],
            });
        }
    };

    match kitty::create_txn_for_buy_kitty(
        &db,
        input_coins,
        &kitty_dna,
        buyer_public_key_h256.unwrap(),
        seller_public_key_h256.unwrap(),
        &output_amount,
        &client,
    )
    .await
    {
        Ok(Some(res)) => {
            // Convert created_kitty to JSON and include it in the response
            let txn = res.transaction;
            let utxo_list = kitty::create_inpututxo_list(&mut txn.clone(), &client).await;

            let response = GetTxnAndUtxoListForList {
                message: format!("Kitty name update txn created successfully"),
                transaction: Some(txn),
                input_utxo_list: utxo_list.expect("Cant crate the Utxo List"),
                encoded: res.encoded,
            };
            Json(response)
        }
        Ok(None) => Json(GetTxnAndUtxoListForList {
            message: format!("Kitty name update txn creation failed: No input returned"),
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
        Err(err) => Json(GetTxnAndUtxoListForList {
            message: format!("Error!! Kitty name update txn creation: {:?}", err),
            transaction: None,
            input_utxo_list: None,
            encoded: vec![],
        }),
    }
}

#[derive(Debug, Serialize)]
pub struct BuyTdKittyResponse {
    pub message: String,
    pub td_kitty: Option<TradableKittyData>,
}

pub async fn buy_kitty(
    body: Json<SignedTxnRequest>,
) -> Result<Json<BuyTdKittyResponse>, Infallible> {
    let client_result = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"));

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(BuyTdKittyResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                td_kitty: None,
            }));
        }
    };

    match kitty::buy_kitty(&body.signed_transaction, &client).await {
        Ok(Some(traded_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = BuyTdKittyResponse {
                message: format!("Kitty traded successfully"),
                td_kitty: Some(traded_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        }
        Ok(None) => Ok(Json(BuyTdKittyResponse {
            message: format!("Kitty trade failed: No data returned"),
            td_kitty: None,
        })),
        Err(err) => Ok(Json(BuyTdKittyResponse {
            message: format!("Error in trading: {:?}", err),
            td_kitty: None,
        })),
    }
}

#[cfg(test)]
mod tests {
    use crate::create_kitty;
    use crate::get_local_keystore;
    use crate::service_handlers::kitty_handler::kitty_service_handler::CreateKittyRequest;
    use crate::service_handlers::kitty_handler::kitty_service_handler::CreateKittyResponse;
    use axum::Json;
    use axum::{http::HeaderMap, http::HeaderValue, Extension};
    use std::convert::Infallible;
    pub const SHAWN_PUB_KEY: &str =
        "d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67";

    #[tokio::test]
    async fn test_create_kitty_success() {
        // Create a MintCoinsRequest
        let _ = get_local_keystore().await.expect("Error"); // this is prerequisite
        let request = CreateKittyRequest {
            name: "Amit".to_string(),
            owner_public_key: SHAWN_PUB_KEY.to_string(),
        };

        // Wrap it in Json
        let json_request = Json(request);

        // Call create_kitty with the Json object
        let response = create_kitty(json_request).await;
        assert!(
            response
                .clone()
                .unwrap()
                .message
                .contains("Kitty created successfully")
                && response.unwrap().kitty != None
        )
    }

    #[tokio::test]
    async fn test_create_kitty_fail_due_unknown_public_key() {
        let _ = get_local_keystore().await.expect("Error");
        let request = CreateKittyRequest {
            name: "Amit".to_string(),
            owner_public_key: SHAWN_PUB_KEY.to_string(),
        };

        // Wrap it in Json
        let json_request = Json(request);

        // Call create_kitty with the Json object
        let response = create_kitty(json_request).await;
        // Still minting coin is success with uninserted public key from blockchain.
        assert!(
            response
                .clone()
                .unwrap()
                .message
                .contains("Kitty created successfully")
                && response.unwrap().kitty != None
        )
    }
    //Invalid in public key, Can't decode

    #[tokio::test]
    async fn test_create_kitty_fail_due_inavlid_public_key() {
        let _ = get_local_keystore().await.expect("Error");
        let request = CreateKittyRequest {
            name: "Amit".to_string(),
            owner_public_key: "Invalid public key".to_string(),
        };

        // Wrap it in Json
        let json_request = Json(request);

        // Call create_kitty with the Json object
        let response = create_kitty(json_request).await;
        // Still minting coin is success with uninserted public key from blockchain.
        assert!(response
            .clone()
            .unwrap()
            .message
            .contains("Invalid in public key, Can't decode"))
    }

    // Test case for get_txn_and_inpututxolist_for_list_kitty_for_sale startts here :

    async fn pre_requsite_create_kitty() -> Result<Json<CreateKittyResponse>, Infallible> {
        let _ = get_local_keystore().await.expect("Error"); // this is prerequisite
        let request = CreateKittyRequest {
            name: "Amit".to_string(),
            owner_public_key: SHAWN_PUB_KEY.to_string(),
        };

        // Wrap it in Json
        let json_request = Json(request);

        // Call create_kitty with the Json object
        create_kitty(json_request).await
    }

    use crate::get_db;
    use crate::get_txn_and_inpututxolist_for_list_kitty_for_sale;
    use crate::sync_and_get_db;
    use hex::encode;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::time::sleep;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_get_txn_and_inpututxolist_for_list_kitty_for_sale_success() {
        let kitty_create_response = pre_requsite_create_kitty();
        let db = Arc::new(Mutex::new(get_db().await.expect("Failed to init db")));
        let clone_db = db.clone();
        sleep(Duration::from_secs(5)).await;
        let _ = sync_and_get_db(clone_db).await;

        let _ = get_local_keystore().await.expect("Error");
        let mut headers = HeaderMap::new();
        let kitty_dna = kitty_create_response
            .await
            .unwrap()
            .kitty
            .clone()
            .unwrap()
            .dna;
        let hex_string = encode(&kitty_dna.0);

        // Convert hex string to HeaderValue
        let header_value = HeaderValue::from_str(&hex_string).unwrap();
        headers.insert("kitty-dna", header_value);
        headers.insert("kitty-price", HeaderValue::from_static("200"));
        headers.insert(
            "owner_public_key",
            HeaderValue::from_str(SHAWN_PUB_KEY).unwrap(),
        );
        let response =
            get_txn_and_inpututxolist_for_list_kitty_for_sale(headers, Extension(db.clone())).await;
        assert!(response
            .message
            .contains("List kitty for Sale txn created successfull"))
    }
}
