package test

import (
	"testing"

	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/std"
	"github.com/cosmos/cosmos-sdk/x/auth/tx"
	"github.com/sei-protocol/sei-chain/app"
	dextypes "github.com/sei-protocol/sei-chain/x/dex/types"
	"github.com/sei-protocol/spot-integration-tests/driver"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

// each scenario now are independent
var TESTS_TO_RUN = []string{
	"simple_buy_sell",
	"simple_cancellation",
	"simple_deposit",
	"simple_withdraw",
}

func getEncodingConfig() parser.EncodingConfig {
	cdc := codec.NewLegacyAmino()
	interfaceRegistry := types.NewInterfaceRegistry()
	marshaler := codec.NewProtoCodec(interfaceRegistry)
	config := parser.EncodingConfig{
		InterfaceRegistry: interfaceRegistry,
		Marshaler:         marshaler,
		TxConfig:          tx.NewTxConfig(marshaler, tx.DefaultSignModes),
		Amino:             cdc,
	}
	std.RegisterLegacyAminoCodec(config.Amino)
	std.RegisterInterfaces(config.InterfaceRegistry)
	app.ModuleBasics.RegisterLegacyAminoCodec(config.Amino)
	app.ModuleBasics.RegisterInterfaces(config.InterfaceRegistry)
	return config
}

func TestAll(t *testing.T) {
	config := getEncodingConfig()
	for _, filename := range TESTS_TO_RUN {
		t.Logf("Testing %s\n", filename)
		test := parser.ParseTestFile(filename)
		adminKey := driver.GetKey(driver.ADMIN_KEY_NAME)
		contractAddr := driver.SendIntantiate(config, adminKey)
		t.Logf("Contract address %s\n", contractAddr)
		for _, pair := range test.Pairs {
			driver.SendRegisterContract(config, adminKey, contractAddr)
			proposalId := driver.SendPairsProposal(config, adminKey, filename, contractAddr, []*dextypes.Pair{
				{PriceDenom: pair.PriceDenom, AssetDenom: pair.AssetDenom, Ticksize: &pair.Ticksize},
			})
			t.Logf("Created proposal with ID %s\n", proposalId)
			driver.VoteAndWaitUntilProposalApproved(config, adminKey, proposalId)
			t.Logf("Proposal approved\n")
		}
		handler := driver.NewHandler()
		handler.HandleInputs(config, test.Inputs, contractAddr)
		handler.VerifyExpectations(t, test.Expectations, contractAddr)
	}
}
