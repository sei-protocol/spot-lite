# spot-contract

This is the github repo for the Spot Lite protocol.

# Set up local Sei 
Please follow the documentation here for setting up your local sei testing environment: https://docs.seinetwork.io/smart-contracts-and-local-development/set-up-a-local-network

You can also use this deployment script to automate set up Sei locally: https://github.com/sei-protocol/sei-chain/blob/master/scripts/initialize_local_test_node.sh

# Instantiating a contract 
The following steps show how to upload your contract to the chain. 

Build the image. Note that the following steps are run from your contract directory, so they assume seid is in your $PATH:
```
cargo build
```
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6
```

# Deploy Spot Lite contract to Sei
The following steps show how to deploy your contract to the chain and place a test order. Make sure you have the Sei chain running in the background before you move forwards!

```
cd scripts/deployment
chmod 755 spot_deploy.sh
./spot_deploy.sh
```

Then you're all set! Remember to write down the contract address printed out at the end of the scripts as you will need the address to interact with the deployed contract.

# Interact with Spot Lite contract
You can place order through the seid cli and query the result there:
```
# order is formatted in OrderDirection?Quantity?Price?PriceAsset?QuoteAsset?OrderType?OrderData
seid tx dex place-orders $contract_addr 'LONG?1.01?5?USDC?ATOM?LIMIT?{"leverage":"1","position_effect":"Open"}' --amount=1000000000uusdc -y --from=$key_name --chain-id=sei-chain --fees=1000000usei --gas=50000000 --broadcast-mode=block
seid q dex list-long-book $contract_addr USDC ATOM
```