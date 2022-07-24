package driver

import (
	"context"
	"fmt"
	"time"

	"github.com/cosmos/cosmos-sdk/client"
	clienttx "github.com/cosmos/cosmos-sdk/client/tx"
	cryptotypes "github.com/cosmos/cosmos-sdk/crypto/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	typestx "github.com/cosmos/cosmos-sdk/types/tx"
	"github.com/cosmos/cosmos-sdk/types/tx/signing"
	xauthsigning "github.com/cosmos/cosmos-sdk/x/auth/signing"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

func addGasFee(txBuilder *client.TxBuilder) {
	(*txBuilder).SetGasLimit(2000000)
	(*txBuilder).SetFeeAmount([]sdk.Coin{
		sdk.NewCoin("usei", sdk.NewInt(100000)),
	})
}

func signTx(config parser.EncodingConfig, key cryptotypes.PrivKey, txBuilder *client.TxBuilder) {
	var sigsV2 []signing.SignatureV2
	accountNum, seqNum := getAccountNumberSequenceNumber(config, key)
	sigV2 := signing.SignatureV2{
		PubKey: key.PubKey(),
		Data: &signing.SingleSignatureData{
			SignMode:  config.TxConfig.SignModeHandler().DefaultMode(),
			Signature: nil,
		},
		Sequence: seqNum,
	}
	sigsV2 = append(sigsV2, sigV2)
	_ = (*txBuilder).SetSignatures(sigsV2...)
	sigsV2 = []signing.SignatureV2{}
	signerData := xauthsigning.SignerData{
		ChainID:       CHAIN_ID,
		AccountNumber: accountNum,
		Sequence:      seqNum,
	}
	sigV2, _ = clienttx.SignWithPrivKey(
		config.TxConfig.SignModeHandler().DefaultMode(),
		signerData,
		*txBuilder,
		key,
		config.TxConfig,
		seqNum,
	)
	sigsV2 = append(sigsV2, sigV2)
	_ = (*txBuilder).SetSignatures(sigsV2...)
}

func sendTx(
	config parser.EncodingConfig,
	key cryptotypes.PrivKey,
	txBuilder *client.TxBuilder,
) *sdk.TxResponse {
	grpcConn := getGRPCConn()
	defer grpcConn.Close()
	client := typestx.NewServiceClient(grpcConn)

	txBytes, _ := config.TxConfig.TxEncoder()((*txBuilder).GetTx())
	grpcRes, err := client.BroadcastTx(
		context.Background(),
		&typestx.BroadcastTxRequest{
			Mode:    typestx.BroadcastMode_BROADCAST_MODE_BLOCK,
			TxBytes: txBytes,
		},
	)
	if err != nil {
		panic(err)
	}
	fmt.Println(grpcRes.TxResponse)
	if grpcRes.TxResponse.Code != 0 && grpcRes.TxResponse.Code != 9 {
		panic(fmt.Sprintf("Error: %d\n", grpcRes.TxResponse.Code))
	}
	return grpcRes.TxResponse
}

func getAccountNumberSequenceNumber(config parser.EncodingConfig, privKey cryptotypes.PrivKey) (uint64, uint64) {
	hexAccount := privKey.PubKey().Address()
	address, err := sdk.AccAddressFromHex(hexAccount.String())
	if err != nil {
		panic(err)
	}
	accountRetriever := authtypes.AccountRetriever{}
	cl, err := client.NewClientFromNode(NODE_URI)
	if err != nil {
		panic(err)
	}
	context := client.Context{}
	context = context.WithNodeURI(NODE_URI)
	context = context.WithClient(cl)
	context = context.WithInterfaceRegistry(config.InterfaceRegistry)
	account, seq, err := accountRetriever.GetAccountNumberSequence(context, address)
	if err != nil {
		time.Sleep(5 * time.Second)
		// retry once after 5 seconds
		account, seq, err = accountRetriever.GetAccountNumberSequence(context, address)
		if err != nil {
			panic(err)
		}
	}
	return account, seq
}

func getEventAttributeValue(response sdk.TxResponse, eventType string, attributeKey string) string {
	for _, log := range response.Logs {
		for _, event := range log.Events {
			if event.Type != eventType {
				continue
			}
			for _, attribute := range event.Attributes {
				if attribute.Key != attributeKey {
					continue
				}
				return attribute.Value
			}
		}
	}
	panic(fmt.Sprintf("Event %s attribute %s not found", eventType, attributeKey))
}
