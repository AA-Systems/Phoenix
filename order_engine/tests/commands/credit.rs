use order_engine::commands::apply_command::{
    ApplyError, ApplyOutcome, apply_command, apply_command_effects,
};
use types::{command::Command, ledger_entries::LedgerEntryType};
use uuid::Uuid;

use crate::common::{USDC_AVAILABLE, fixture};

#[test]
fn credit_balance_increases_available() {
    let mut fx = fixture();

    apply_command(
        &mut fx.state,
        Command::CreditBalance {
            command_id: Uuid::new_v4(),
            user_id: fx.user_id,
            asset_id: fx.usdc_id,
            amount: 50_000_000,
        },
    )
    .unwrap();

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, USDC_AVAILABLE + 50_000_000);
    assert_eq!(usdc.locked, 0);
}

#[test]
fn credit_balance_emits_deposit_ledger_intent() {
    let mut fx = fixture();
    let command_id = Uuid::new_v4();

    let outcome = apply_command_effects(
        &mut fx.state,
        Command::CreditBalance {
            command_id,
            user_id: fx.user_id,
            asset_id: fx.usdc_id,
            amount: 25_000_000,
        },
    )
    .unwrap();

    match outcome {
        ApplyOutcome::Applied { intents, .. } => {
            assert_eq!(intents.len(), 1);
            let intent = &intents[0];
            assert_eq!(intent.command_id, command_id);
            assert_eq!(intent.entry_type, LedgerEntryType::Deposit);
            assert_eq!(intent.available_delta, 25_000_000);
            assert_eq!(intent.locked_delta, 0);
            assert_eq!(intent.available_after, USDC_AVAILABLE + 25_000_000);
            assert_eq!(intent.locked_after, 0);
        }
        other => panic!("expected Applied, got {other:?}"),
    }
}

#[test]
fn credit_balance_creates_row_for_new_user_asset() {
    let mut fx = fixture();
    let other = Uuid::new_v4();

    apply_command(
        &mut fx.state,
        Command::CreditBalance {
            command_id: Uuid::new_v4(),
            user_id: other,
            asset_id: fx.usdc_id,
            amount: 1_000_000,
        },
    )
    .unwrap();

    let usdc = fx.state.balances.get(&(other, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, 1_000_000);
    assert_eq!(usdc.locked, 0);
}

#[test]
fn credit_balance_rejects_unknown_asset() {
    let mut fx = fixture();

    let err = apply_command(
        &mut fx.state,
        Command::CreditBalance {
            command_id: Uuid::new_v4(),
            user_id: fx.user_id,
            asset_id: Uuid::new_v4(),
            amount: 1,
        },
    )
    .unwrap_err();

    assert_eq!(err, ApplyError::AssetNotFound);
}

#[test]
fn credit_balance_is_idempotent_by_command_id() {
    let mut fx = fixture();
    let command_id = Uuid::new_v4();

    let command = Command::CreditBalance {
        command_id,
        user_id: fx.user_id,
        asset_id: fx.usdc_id,
        amount: 10_000_000,
    };

    apply_command(&mut fx.state, command.clone()).unwrap();
    apply_command(&mut fx.state, command).unwrap();

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, USDC_AVAILABLE + 10_000_000);
}
