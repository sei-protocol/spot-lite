#!/bin/bash

echo -n Customized clearing_house Contract \(../../artifacts/spot_lite.wasm by default\):
read contract
echo
echo -n Customized Key Name:\(admin by default\)
read keyname
echo
echo -n Keyring Password:\(12345678 by default\)
read password
echo

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  printf "OS is linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  printf "OS is Mac"
  sed_flag=\'\'
fi

if [ -z "${contract}" ];
then contract=../../artifacts/spot_lite.wasm
fi 
if [ -z "${keyname}" ];
then keyname=admin
fi 
if [ -z "${password}" ];
then password="12345678\n"
fi 

seid=~/go/bin/seid
code=$(printf $password | $seid tx wasm store $contract -y --from=$keyname --chain-id=sei-chain --gas=10000000 --fees=10000000usei --broadcast-mode=block | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
admin_addr=$(printf $password |$seid keys show admin | grep -A 1 "address" | sed -n 's/.*address: //p')

addr=$(printf $password |$seid tx wasm instantiate $code "{}" --from admin --broadcast-mode=block --label "spot" --no-admin --chain-id sei-chain --gas=30000000 --fees=300000usei -y | grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)

sed -i $sed_flag "s/\"contract_addr\": .*,/\"contract_addr\": \"$addr\",/g" data/register_pair_proposal.json
printf $password |$seid tx dex register-contract $addr $code false -y --from=$keyname --chain-id=sei-chain --fees=10000000usei --gas=10000000 --broadcast-mode=block
proposal_id=$(printf $password |$seid tx dex register-pairs-proposal data/register_pair_proposal.json -y --from=$keyname --chain-id=sei-chain --fees=10000000usei --gas=500000 --broadcast-mode=block | grep -A 1 -m 1 "proposal_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf $password |$seid tx gov deposit $proposal_id 10000000usei -y --from=admin --chain-id=sei-chain --fees=10000000usei --gas=500000 --broadcast-mode=block
printf $password |$seid tx gov vote $proposal_id yes -y --from=admin --chain-id=sei-chain --fees=2000usei --gas=500000 --broadcast-mode=block

# sleep 10 second and send a test order to USDC<>ATOM pair
printf "\n\nWaiting for the proposal to pass"
sleep 10
printf $password |$seid tx dex place-orders $addr 'LONG?1.01?5?USDC?ATOM?LIMIT?{"leverage":"1","position_effect":"Open"}' --amount=1000000000uusdc -y --from=$keyname --chain-id=sei-chain --fees=1000000usei --gas=50000000 --broadcast-mode=block
printf $password |$seid q dex list-long-book $addr USDC ATOM

printf "\n\nDeployed spot contract address is %s\n" $addr
