use crate::commands::apply_command::ApplyError;

pub fn quote_notional(price: i64, quantity: i64, base_decimals: i32) -> Result<i64, ApplyError> {
    if base_decimals < 0 {
        return Err(ApplyError::AssetNotFound);
    }

    let scale = 10_i128
        .checked_pow(base_decimals as u32)
        .ok_or(ApplyError::Overflow)?;
    let notional = (price as i128)
        .checked_mul(quantity as i128)
        .ok_or(ApplyError::Overflow)?
        / scale;

    i64::try_from(notional).map_err(|_| ApplyError::Overflow)
}
