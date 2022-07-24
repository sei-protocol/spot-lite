package driver

import (
	"strconv"
	"time"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	cryptotypes "github.com/cosmos/cosmos-sdk/crypto/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	govutils "github.com/cosmos/cosmos-sdk/x/gov/client/utils"
	"github.com/cosmos/cosmos-sdk/x/gov/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	dextypes "github.com/sei-protocol/sei-chain/x/dex/types"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

func SendIntantiate(config parser.EncodingConfig, key cryptotypes.PrivKey) string {
	txBuilder := config.TxConfig.NewTxBuilder()
	adminAddr := sdk.AccAddress(key.PubKey().Address()).String()
	msgstr := "{}"
	msg := wasmtypes.MsgInstantiateContract{
		Sender: adminAddr,
		Admin:  adminAddr,
		CodeID: 1,
		Label:  "dex",
		Msg:    asciiDecodeString(msgstr),
		Funds: []sdk.Coin{
			sdk.NewCoin("usei", sdk.NewInt(100000)),
		},
	}
	_ = txBuilder.SetMsgs(&msg)
	(txBuilder).SetGasLimit(2000000)
	(txBuilder).SetFeeAmount([]sdk.Coin{
		sdk.NewCoin("usei", sdk.NewInt(100000)),
	})
	signTx(config, key, &txBuilder)
	txResp := *sendTx(config, key, &txBuilder)
	return getEventAttributeValue(txResp, "instantiate", "_contract_address")
}

func SendRegisterContract(config parser.EncodingConfig, key cryptotypes.PrivKey, contractAddr string) {
	txBuilder := config.TxConfig.NewTxBuilder()
	msg := dextypes.MsgRegisterContract{
		Creator: sdk.AccAddress(key.PubKey().Address()).String(),
		Contract: &dextypes.ContractInfo{
			CodeId:            1,
			ContractAddr:      contractAddr,
			NeedOrderMatching: true,
			NeedHook:          false,
		},
	}
	_ = txBuilder.SetMsgs(&msg)
	(txBuilder).SetGasLimit(2000000)
	(txBuilder).SetFeeAmount([]sdk.Coin{
		sdk.NewCoin("usei", sdk.NewInt(100000)),
	})
	signTx(config, key, &txBuilder)
	sendTx(config, key, &txBuilder)
}

func SendPairsProposal(
	config parser.EncodingConfig,
	key cryptotypes.PrivKey,
	title string,
	contractAddr string,
	pairs []*dextypes.Pair,
) string {
	txBuilder := config.TxConfig.NewTxBuilder()
	from := sdk.AccAddress(key.PubKey().Address())
	content := dextypes.NewRegisterPairsProposal(
		title,
		title,
		[]dextypes.BatchContractPair{
			{
				ContractAddr: contractAddr,
				Pairs:        pairs,
			},
		},
	)
	deposit := sdk.NewCoins(
		sdk.NewCoin("usei", govtypes.DefaultMinDepositTokens),
	)
	msg, err := govtypes.NewMsgSubmitProposal(&content, deposit, from)
	if err != nil {
		panic(err)
	}
	_ = txBuilder.SetMsgs(msg)
	(txBuilder).SetGasLimit(500000)
	(txBuilder).SetFeeAmount([]sdk.Coin{
		sdk.NewCoin("usei", sdk.NewInt(10000000)),
	})
	signTx(config, key, &txBuilder)
	txResp := *sendTx(config, key, &txBuilder)
	return getEventAttributeValue(txResp, "submit_proposal", "proposal_id")
}

func VoteAndWaitUntilProposalApproved(config parser.EncodingConfig, key cryptotypes.PrivKey, proposalId string) {
	txBuilder := config.TxConfig.NewTxBuilder()
	from := sdk.AccAddress(key.PubKey().Address())
	proposalID, err := strconv.ParseUint(proposalId, 10, 64)
	if err != nil {
		panic(err)
	}

	byteVoteOption, err := types.VoteOptionFromString(govutils.NormalizeVoteOption("yes"))
	if err != nil {
		panic(err)
	}
	msg := govtypes.NewMsgVote(from, proposalID, byteVoteOption)
	_ = txBuilder.SetMsgs(msg)
	(txBuilder).SetGasLimit(500000)
	(txBuilder).SetFeeAmount([]sdk.Coin{
		sdk.NewCoin("usei", sdk.NewInt(10000000)),
	})
	signTx(config, key, &txBuilder)
	sendTx(config, key, &txBuilder)
	for {
		if IsProposalHandled(proposalId) {
			break
		}
		time.Sleep(time.Second * VOTE_WAIT_SECONDS)
	}
}
