extern crate bip39;
extern crate kusama_runtime;

use bip39::{Language, Mnemonic, MnemonicType};
use kusama_runtime::{BalancesCall, Call, Runtime, UncheckedExtrinsic, VERSION};
use polkadot_primitives::{AccountId, Balance, Hash, Signature};
use sp_core::crypto::Pair as PairTrait;
use sp_core::sr25519::Pair;
use sp_core::H256;
use sp_runtime::traits::{IdentifyAccount, Verify};

type AccountPublic = <Signature as Verify>::Signer;

//#[test]
fn test_1() {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

    let genesis_hash: H256 =
        H256::from_slice(b"dcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025b");
    let (signer, _) = Pair::from_entropy(mnemonic.entropy(), None);
    let index: u32 = 100;
    let to: AccountId = AccountPublic::from(signer.public()).into_account();
    let amount: Balance = 1_000_000;
    let function = Call::Balances(BalancesCall::transfer(to.into(), amount));
}
