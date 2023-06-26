# ARKANA MEMBERSHIP NFT

Cloned from https://github.com/TENK-DAO/tenk

User can only mint 1 NFT
Only allowed signer can mint from this contract

## Build Contract
```bash
./build.sh
```

## Add Signer Account (to allow signer for metatx)
```bash
near call <contrac_id> add_signer_account '{"account_id": "<allowed signer account_id>"}' --accountId <owner or admin account_id>
```