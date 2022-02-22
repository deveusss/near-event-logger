## To build smart contracts
```bash
./build.sh
```

## To deploy smart contracts to testnet
```bash
. ./deploy-testnet.sh
```
> additional dot is required for running shell script in current session, so that all required parameters will be initialized correctly

### Call log_event function
```bash
$signer=your_user
near call $CONTRACT_NAME log_event '{"time": 9929332, "operation": "min_nft","transaction_hash":"hash100220"}' --accountId $signer --amount --gas 6000000000000
```
