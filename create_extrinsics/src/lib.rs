use bip39::{Language, Mnemonic, MnemonicType};
use kusama_runtime::{BalancesCall, Call, Runtime, UncheckedExtrinsic, VERSION};
use polkadot_primitives::{AccountId, Balance, Hash, Signature};
use sp_core::crypto::Pair as PairTrait;
use sp_core::sr25519::Pair;
use sp_core::H256;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::generic::Era;
use parity_scale_codec::Encode;

type AccountPublic = <Signature as Verify>::Signer;

#[test]
fn test_create_extrinsic() {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

    //let genesis_hash: H256 =
        //H256::from_slice(b"dcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025b");
    let index: u32 = 100;

    let (signer, _) = Pair::from_entropy(mnemonic.entropy(), None);
    let to: AccountId = AccountPublic::from(signer.public()).into_account();
    let amount: Balance = 1_000_000;
    let function = Call::Balances(BalancesCall::transfer(to.into(), amount));

    let ext = UncheckedExtrinsic::new_unsigned(function);
    //println!("EXT: {:?}", ext.encode());
    println!("EXT: {:?}", ext);
}

// TODO
fn create_extrinsic(function: Call, index: u32, signer: Pair, genesis_hash: H256) {
    let extra = |i: u32, f: Balance| {
        (
			frame_system::CheckVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Runtime>::from(i),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(f),
			//Default::default(),
        )
    };

}