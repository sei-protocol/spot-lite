package driver

import (
	"encoding/hex"
	"encoding/json"

	wasmdtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	cryptotypes "github.com/cosmos/cosmos-sdk/crypto/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	dextypes "github.com/sei-protocol/sei-chain/x/dex/types"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

func SendOrder(config parser.EncodingConfig, key cryptotypes.PrivKey, order parser.FundedOrder, contractAddr string) uint64 {
	seiOrder := parser.ToSeiOrderPlacement(order)
	orderPlacements := []*dextypes.Order{&seiOrder}
	amount, _ := sdk.ParseCoinsNormalized(order.Fund)
	txBuilder := config.TxConfig.NewTxBuilder()
	msg := dextypes.MsgPlaceOrders{
		Creator:      sdk.AccAddress(key.PubKey().Address()).String(),
		Orders:       orderPlacements,
		ContractAddr: contractAddr,
		Funds:        amount,
	}
	_ = txBuilder.SetMsgs(&msg)
	signTx(config, key, &txBuilder)
	resp := sendTx(config, key, &txBuilder)
	msgResp := sdk.TxMsgData{}
	respDataBytes, err := hex.DecodeString(resp.Data)
	if err != nil {
		panic(err)
	}
	if err := msgResp.Unmarshal(respDataBytes); err != nil {
		panic(err)
	}
	orderPlacementResponse := dextypes.MsgPlaceOrdersResponse{}
	orderMsgData := msgResp.Data[0].Data
	if err := orderPlacementResponse.Unmarshal([]byte(orderMsgData)); err != nil {
		panic(err)
	}
	return orderPlacementResponse.OrderIds[0]
}

func SendCancel(
	config parser.EncodingConfig,
	key cryptotypes.PrivKey,
	order parser.Cancel,
	contractAddr string,
	monikerToOrderIds map[string]uint64,
) {
	txBuilder := config.TxConfig.NewTxBuilder()
	msg := dextypes.MsgCancelOrders{
		Creator:      sdk.AccAddress(key.PubKey().Address()).String(),
		OrderIds:     []uint64{monikerToOrderIds[order.Moniker]},
		ContractAddr: contractAddr,
	}
	_ = txBuilder.SetMsgs(&msg)
	(txBuilder).SetGasLimit(2000000)
	(txBuilder).SetFeeAmount([]sdk.Coin{
		sdk.NewCoin("usei", sdk.NewInt(100000)),
	})
	signTx(config, key, &txBuilder)
	sendTx(config, key, &txBuilder)
}

func Deposit(config parser.EncodingConfig, key cryptotypes.PrivKey, deposit parser.Deposit, contractAddr string) {
	amount, _ := sdk.ParseCoinsNormalized(deposit.Fund)
	txBuilder := config.TxConfig.NewTxBuilder()
	msg := wasmdtypes.MsgExecuteContract{
		Sender:   sdk.AccAddress(key.PubKey().Address()).String(),
		Contract: contractAddr,
		Msg:      []byte("{\"deposit\":{}}"),
		Funds:    amount,
	}
	_ = txBuilder.SetMsgs(&msg)
	addGasFee(&txBuilder)
	signTx(config, key, &txBuilder)
	sendTx(config, key, &txBuilder)
}

func Withdraw(config parser.EncodingConfig, key cryptotypes.PrivKey, withdraw parser.Withdraw, contractAddr string) {
	amount, _ := sdk.ParseCoinsNormalized(withdraw.Fund)
	contractMsg := parser.ContractWithdrawMsg{Withdraw: parser.ContractWithdraw{Coins: amount}}
	msgJson, _ := json.Marshal(contractMsg)
	txBuilder := config.TxConfig.NewTxBuilder()
	msg := wasmdtypes.MsgExecuteContract{
		Sender:   sdk.AccAddress(key.PubKey().Address()).String(),
		Contract: contractAddr,
		Msg:      msgJson,
		Funds:    sdk.Coins{},
	}
	_ = txBuilder.SetMsgs(&msg)
	addGasFee(&txBuilder)
	signTx(config, key, &txBuilder)
	sendTx(config, key, &txBuilder)
}
