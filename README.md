# spot-contract

This is the github repo for the Spot Lite protocol.

# Set up local Sei 
Please follow the documentation on the official Sei doc to set up your local sei testing environment.

You can also use this deployment script to automate set up Sei locally: https://github.com/sei-protocol/sei-chain/blob/master/scripts/initialize_local_test_node.sh

# Instantiating a contract 
The following steps show how to upload your contract to the chain. Note that the following steps are run from your contract directory, so they assume `seid` is in your $PATH:
```
cargo build
```

Use `rust-optimizer` to reduce the size of the compiled wasm file:
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

# Deploy Spot Lite contract to Sei
The following steps show how to deploy your contract to the chain and place a test order. Make sure you have the Sei chain running in the background before proceeding!

```
// store the contract to Sei
seid tx wasm store artifacts/spot_lite.wasm -y --from=admin --chain-id=sei-chain --gas=2000000 --fees=20000usei --broadcast-mode=block

// instantiate the contract
seid tx wasm instantiate 1 "{}" --from admin --broadcast-mode=block --label "spot" --admin sei10fptzxjjewqgrazq6hrn8hvyza6s92qhzqu98h --chain-id sei-chain --gas=2000000 --fees=20000usei -y

// register contract to the dex module
seid tx dex register-contract sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m 1 true true 1000000000 -y --from=admin --chain-id=sei-chain --fees=20000usei --gas=2000000 --broadcast-mode=block

// register tradable pairs 
seid tx dex register-pairs scripts/deployment/data/register_pair_proposal.json -y --from=admin --chain-id=sei-chain --fees=2000usei --gas=200000 --broadcast-mode=block

```

You can also use the deployment script below to deploy the spot contract:
``` 
cd scripts/deployment
chmod 755 spot_deploy.sh
./spot_deploy.sh
```

Then you're all set! Remember to write down the contract address printed out at the end of the scripts, as you will need the address to interact with the deployed contract.

# Interact with Spot Lite contract
You can place order through the `seid` Cli and query the result there:
```
# order is formatted in OrderDirection?Quantity?Price?PriceAsset?QuoteAsset?OrderType?OrderData
 seid tx dex place-orders sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m 'LONG?1.01?5?USDC?ATOM?LIMIT?{"leverage":"1","position_effect":"Open"}' --amount=1000000uusdc -y --from=admin --chain-id=sei-chain --fees=1000usei --gas=100000 --broadcast-mode=block
seid q dex list-long-book sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m  uusdc uatom
```