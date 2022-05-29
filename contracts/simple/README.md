## Commands

near dev-deploy --wasmFile ../res/simple.wasm --helperUrl https://near-contract-helper.onrender.com
source neardev/dev-account.env
echo $CONTRACT_NAME
near call $CONTRACT_NAME get_nothing --accountId $CONTRACT_NAME
near call $CONTRACT_NAME get_something '{"add": 10}' --accountId $CONTRACT_NAME
near view-state $CONTRACT_NAME --finality final