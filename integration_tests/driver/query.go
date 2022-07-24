package driver

import (
	"context"
	"fmt"
	"strconv"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

func QueryBalance(address string, denom string, contractAddr string) parser.Balance {
	query := fmt.Sprintf("{\"get_balance\":{\"account\":\"%s\",\"denom\":\"%s\"}}", address, denom)
	response := queryWasm(query, contractAddr)

	return parser.ParseBalance(response)
}

func QueryOrder(orderID uint64, contractAddr string) parser.Order {
	query := fmt.Sprintf("{\"get_order\":{\"id\":%d}}", orderID)
	response := queryWasm(query, contractAddr)

	return parser.ParseOrder(response)
}

func queryWasm(query string, contractAddr string) wasmtypes.RawContractMessage {
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
		panic(err)
	}
	return res.Data
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

func asciiDecodeString(s string) []byte {
	return []byte(s)
}
