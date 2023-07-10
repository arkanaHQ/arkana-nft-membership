# ARKANA MEMBERSHIP NFT

Cloned from https://github.com/TENK-DAO/tenk

User can only mint 1 NFT
Only allowed signer can mint from this contract

## Build Contract
```bash
./build.sh
```

## Deploy Contract (Arkana Basic)
```bash
near call --accountId $CONTRACT_ID $CONTRACT_ID new '{"owner_id":"$OWNER_ID","metadata":{"spec":"nft-1.0.0","name":"Arkana Basic","symbol":"ARKANABASIC","base_uri":"https://w3s.link/ipfs/bafybeiepws7d7sfe47ifvbvld7gkjp7pckm4beylguktq3b5glzvpufwvq"},"size":4294967295,"sale":{"price":"0"}}'
```

## Add Signer Account (to allow signer for metatx)
```bash
near call <contract_id> add_signer_account '{"account_id": "<allowed signer account_id>"}' --accountId <owner or admin account_id>
```

## Start sale
```bash
near call --accountId $OWNER_ID $CONTRACT_ID start_sale ''
```
