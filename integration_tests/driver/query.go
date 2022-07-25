package driver

import (
	"context"
	"fmt"
	"strconv"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

func QueryBalance(address string, denom string, contractAddr string) parser.Balance {
	query := fmt.Sprintf("{\"get_balance\":{\"account\":\"%s\",\"denom\":\"%s\"}}", address, denom)
	response, _ := queryWasm(query, contractAddr)

	return parser.ParseBalance(response)
}

func QueryOrder(orderID uint64, contractAddr string) (parser.Order, error) {
	query := fmt.Sprintf("{\"get_order\":{\"id\":%d}}", orderID)
	response, err := queryWasm(query, contractAddr)
	if err != nil {
		return parser.Order{}, err
	}

	return parser.ParseOrder(response), nil
}

func queryWasm(query string, contractAddr string) (wasmtypes.RawContractMessage, error) {
	grpcConn := getGRPCConn()
	defer grpcConn.Close()
	client := wasmtypes.NewQueryClient(grpcConn)
	res, err := client.SmartContractState(
		context.Background(),
		&wasmtypes.QuerySmartContractStateRequest{
			Address:   contractAddr,
			QueryData: asciiDecodeString(query),
		},
	)
	if err != nil {
		return nil, err
	}
	return res.Data, nil
}

func IsProposalHandled(proposalId string) bool {
	grpcConn := getGRPCConn()
	defer grpcConn.Close()
	client := govtypes.NewQueryClient(grpcConn)
	proposalID, err := strconv.ParseUint(proposalId, 10, 64)
	if err != nil {
		panic(err)
	}
	res, err := client.Proposal(context.Background(), &govtypes.QueryProposalRequest{ProposalId: proposalID})
	return err == nil && res.Proposal.Status == govtypes.StatusPassed
}

func GetBankBalance(account string, denom string) sdk.Coin {
	grpcConn := getGRPCConn()
	defer grpcConn.Close()
	client := banktypes.NewQueryClient(grpcConn)
	address, err := sdk.AccAddressFromBech32(GetAddress(account))
	if err != nil {
		panic(err)
	}
	res, err := client.Balance(context.Background(), banktypes.NewQueryBalanceRequest(address, denom))
	if err != nil {
		panic(err)
	}
	return *res.Balance
}

func asciiDecodeString(s string) []byte {
	return []byte(s)
}
