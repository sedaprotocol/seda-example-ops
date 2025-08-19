use anyhow::{Result, anyhow};
use const_hex::const_decode_to_array;
use ethabi::{Token, ethereum_types::U256};
use k256::ecdsa::{Signature, VerifyingKey, signature::hazmat::PrehashVerifier};
use seda_sdk_rs::{
    HttpFetchMethod, HttpFetchResponse, Process, bytes::ToBytes, elog,
    generate_proxy_http_signing_message, get_unfiltered_reveals, keccak256, log,
};

pub fn tally_phase() -> Result<()> {
    // Retrieve consensus reveals from the tally phase.
    let reveals = get_unfiltered_reveals()?;

    if reveals.len() != 1 {
        elog!(
            "Expected exactly one reveal(replication factor 1), found {}",
            reveals.len()
        );
        return Err(anyhow::anyhow!("Invalid number of reveals"));
    }

    let http_response: HttpFetchResponse = serde_json::from_slice(&reveals[0].body.reveal)?;

    let signature_hex = http_response
        .headers
        .get("x-seda-signature")
        .ok_or_else(|| anyhow!("Missing x-seda-signature header"))?;
    let public_key_hex = http_response
        .headers
        .get("x-seda-publickey")
        .ok_or_else(|| anyhow!("Missing x-seda-publickey header"))?;

    let signature: [u8; 64] = const_decode_to_array(signature_hex.as_bytes())?;
    let public_key: [u8; 33] = const_decode_to_array(public_key_hex.as_bytes())?;

    let message = generate_proxy_http_signing_message(
        http_response.url,
        HttpFetchMethod::Get,
        Vec::with_capacity(0).to_bytes(),
        http_response.bytes.clone().to_bytes(),
    )
    .eject();

    let public_key_obj = VerifyingKey::from_sec1_bytes(&public_key)?;
    let signature_obj = Signature::from_slice(&signature)?;
    let hashed_message = keccak256(message);
    let verified = public_key_obj
        .verify_prehash(&hashed_message, &signature_obj)
        .is_ok();

    if !verified {
        elog!("Signature verification failed for the proxy response: {verified}");
        return Err(anyhow::anyhow!("Signature verification failed"));
    }

    // Parse the API response as defined earlier.
    let response_data = serde_json::from_slice::<
        serde_json::value::Map<String, serde_json::value::Value>,
    >(&http_response.bytes)?;

    // Extract the prices for each symbol from the response data.
    let prices = response_data
        .values()
        .map(|price| (price["usd"].as_f64().unwrap_or_default() * 1_000_000f64) as u128)
        .map(U256::from)
        .map(Token::Uint)
        .collect::<Vec<Token>>();
    log!("Fetched prices: {prices:?}");

    // Encode the final median price as a EVM `uint256[]`.
    let result = ethabi::encode(&[Token::Array(prices)]);
    // Report the successful result in the tally phase.
    Process::success(&result);

    Ok(())
}
