#!/bin/bash

cd /contract
python3 -u /contract/integration_tests/server.py &
# echo "Change block time to 0.5s"
if [ ! -z "$1" ]; then
  CONFIG_PATH="$1"
else
  CONFIG_PATH="$HOME/.sei/config/config.toml"
fi

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  echo "Slow down timeout setting"
  sed -i 's/max_num_inbound_peers =.*/max_num_inbound_peers = 150/g' $CONFIG_PATH
  sed -i 's/max_num_outbound_peers =.*/max_num_outbound_peers = 150/g' $CONFIG_PATH
  sed -i 's/max_packet_msg_payload_size =.*/max_packet_msg_payload_size = 10240/g' $CONFIG_PATH
  sed -i 's/send_rate =.*/send_rate = 20480000/g' $CONFIG_PATH
  sed -i 's/recv_rate =.*/recv_rate = 20480000/g' $CONFIG_PATH
  sed -i 's/max_txs_bytes =.*/max_txs_bytes = 10737418240/g' $CONFIG_PATH
  sed -i 's/^size =.*/size = 5000/g' $CONFIG_PATH
  sed -i 's/max_tx_bytes =.*/max_tx_bytes = 2048576/g' $CONFIG_PATH
  sed -i 's/timeout_prevote =.*/timeout_prevote = "500ms"/g' $CONFIG_PATH
  sed -i 's/timeout_precommit =.*/timeout_precommit = "500ms"/g' $CONFIG_PATH
  sed -i 's/timeout_commit =.*/timeout_commit = "500ms"/g' $CONFIG_PATH
  sed -i 's/skip_timeout_commit =.*/skip_timeout_commit = false/g' $CONFIG_PATH
  sed -i 's|laddr = "tcp://127.0.0.1:26657"|laddr = "tcp://0.0.0.0:26657"|' $CONFIG_PATH
fi

echo "Starting seid..."
/go/bin/seid start --trace &
process_id=$!
sleep 10
ACCOUNT_ADDRESS=$(printf '12345678\n' | /go/bin/seid keys show admin -a)
printf '12345678\n' | /go/bin/seid tx wasm store /contract/target/wasm32-unknown-unknown/release/spot_lite.wasm -y --from=admin --chain-id=sei-chain --gas=5000000 --fees=1000000usei --broadcast-mode=block

wait $process_id
