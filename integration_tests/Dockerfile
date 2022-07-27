# syntax=docker/dockerfile:1

FROM golang:1.18-buster

# Build from repo root: `docker build -f integration_tests/Dockerfile .`

RUN apt-get clean
RUN apt-get -y update
RUN apt-get -y install git
RUN apt-get -y install python3
RUN apt-get -y install jq
RUN apt-get -y install gcc
# RUN curl https://get.docker.com/builds/Linux/x86_64/docker-latest.tgz | tar xvz -C /tmp/ && mv /tmp/docker/docker /usr/bin/docker
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable && rustup target list --installed && rustup target add wasm32-unknown-unknown
RUN mkdir contract
ADD .  /contract/
WORKDIR /contract
RUN RUSTFLAGS='-C link-arg=-s' cargo build --target=wasm32-unknown-unknown  -r
WORKDIR /
RUN mkdir repos
WORKDIR /repos
# Each Vortex commit needs to specify which sei-chain version it should be deployed on. Once sei-chain enters
# a more stable release cycle, we can change raw commit hash here to release tag.
RUN git clone https://github.com/sei-protocol/sei-chain.git && cd sei-chain && git checkout a2cb8833ff391dd6539147f1fa7f4fefa89c7724
WORKDIR /repos/sei-chain
RUN make install
RUN /go/bin/seid init integration --chain-id sei-chain
RUN printf '12345678\n12345678\n' | /go/bin/seid keys add admin 2> /key_output
RUN tail -n 1 /key_output > /admin.mnemonic && rm /key_output
RUN printf '12345678\n12345678\n' | /go/bin/seid keys add alice 2> /key_output
RUN tail -n 1 /key_output > /alice.mnemonic && rm /key_output
RUN printf '12345678\n12345678\n' | /go/bin/seid keys add bob 2> /key_output
RUN tail -n 1 /key_output > /bob.mnemonic && rm /key_output
RUN ACCOUNT_ADDRESS=$(printf '12345678\n' | /go/bin/seid keys show admin -a); printf '12345678\n' | /go/bin/seid add-genesis-account $ACCOUNT_ADDRESS 100000000000000000000usei,100000000000000000000uatom;
RUN ACCOUNT_ADDRESS=$(printf '12345678\n' | /go/bin/seid keys show alice -a); printf '12345678\n' | /go/bin/seid add-genesis-account $ACCOUNT_ADDRESS 10000000000000000000usei,100000000000000000000uatom;
RUN ACCOUNT_ADDRESS=$(printf '12345678\n' | /go/bin/seid keys show bob -a); printf '12345678\n' | /go/bin/seid add-genesis-account $ACCOUNT_ADDRESS 10000000000000000000usei,100000000000000000000uatom;
RUN printf '12345678\n' | /go/bin/seid gentx admin 70000000000000000000usei --chain-id sei-chain
RUN /go/bin/seid collect-gentxs
RUN cat /root/.sei/config/genesis.json | jq '.app_state["crisis"]["constant_fee"]["denom"]="usei"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json
RUN cat /root/.sei/config/genesis.json | jq '.app_state["gov"]["deposit_params"]["min_deposit"][0]["denom"]="usei"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json
RUN cat /root/.sei/config/genesis.json | jq '.app_state["mint"]["params"]["mint_denom"]="usei"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json
RUN cat /root/.sei/config/genesis.json | jq '.app_state["staking"]["params"]["bond_denom"]="usei"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json
RUN cat /root/.sei/config/genesis.json | jq '.app_state["gov"]["deposit_params"]["max_deposit_period"]="5s"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json
RUN cat /root/.sei/config/genesis.json | jq '.app_state["gov"]["voting_params"]["voting_period"]="5s"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json
RUN cat /root/.sei/config/genesis.json | jq '.consensus_params["block"]["time_iota_ms"]="50"' > /root/.sei/config/tmp_genesis.json && mv /root/.sei/config/tmp_genesis.json /root/.sei/config/genesis.json

RUN chmod 777 /contract/integration_tests/container_init.sh
RUN printf '12345678\n' | /go/bin/seid keys show admin -a > /admin.addr
RUN printf '12345678\n' | /go/bin/seid keys show alice -a > /alice.addr
RUN printf '12345678\n' | /go/bin/seid keys show bob -a > /bob.addr

# Since we only have one container we can use host's network by `docker run --rm -v /var/run/docker.sock:/var/run/docker.sock --name it --network host <image id>`.
# Once we need to have more containers we will need to figure out network mapping in a better way.
# On Mac, use `docker run --rm -v /var/run/docker.sock:/var/run/docker.sock --name it -p 8088:8088 -p 9090:9090 -p 26657:26657 -p 1317:1317 -p 26656:26656 <image id>`
CMD /contract/integration_tests/container_init.sh
