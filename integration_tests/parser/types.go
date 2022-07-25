package parser

import (
	"encoding/json"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

type EncodingConfig struct {
	InterfaceRegistry types.InterfaceRegistry
	// NOTE: this field will be renamed to Codec
	Marshaler codec.Codec
	TxConfig  client.TxConfig
	Amino     *codec.LegacyAmino
}

type Test struct {
	Pairs        []Pair       `json:"pairs"`
	Inputs       []Input      `json:"inputs"`
	Expectations Expectations `json:"expectations"`
}

type Pair struct {
	PriceDenom string  `json:"price_denom"`
	AssetDenom string  `json:"asset_denom"`
	Ticksize   sdk.Dec `json:"ticksize"`
}

type Input struct {
	InputType string          `json:"type"`
	Details   json.RawMessage `json:"details"`
}

type FundedOrder struct {
	Moniker string         `json:"moniker"`
	Account string         `json:"account"`
	Order   OrderPlacement `json:"order"`
	Fund    string         `json:"fund"`
}

type OrderData struct {
	PositionEffect string `json:"position_effect"`
}

type Cancel struct {
	Account string `json:"account"`
	Moniker string `json:"moniker"`
}

type Deposit struct {
	Account string `json:"account"`
	Fund    string `json:"fund"`
}

type Withdraw struct {
	Account string `json:"account"`
	Fund    string `json:"fund"`
}

type OrderPlacement struct {
	PositionDirection string `json:"position_direction"`
	Price             string `json:"price"`
	Quantity          string `json:"quantity"`
	PriceDenom        string `json:"price_denom"`
	AssetDenom        string `json:"asset_denom"`
	PositionEffect    string `json:"position_effect"`
	OrderType         string `json:"order_type"`
}

type StartingBalance struct {
	Account string `json:"account"`
	Denom   string `json:"denom"`
}

type Expectations struct {
	Balances     []BalanceExpectation     `json:"balances"`
	Orders       []OrderExpectation       `json:"orders"`
	BankBalances []BankBalanceExpectation `json:"bank_balances"`
}

type BalanceExpectation struct {
	Account string  `json:"account"`
	Denom   string  `json:"denom"`
	Balance Balance `json:"balance"`
}

type OrderExpectation struct {
	Moniker string `json:"moniker"`
	Exists  bool   `json:"exists"`
	Order   Order  `json:"order"`
}

type BankBalanceExpectation struct {
	Account string  `json:"account"`
	Denom   string  `json:"denom"`
	Delta   sdk.Int `json:"delta"`
}

type Balance struct {
	Amount   string `json:"amount"`
	Withheld string `json:"withheld"`
}

type Order struct {
	Id                uint64 `json:"id"`
	Account           string `json:"account"`
	PriceDenom        string `json:"price_denom"`
	AssetDenom        string `json:"asset_denom"`
	Price             string `json:"price"`
	Quantity          string `json:"quantity"`
	RemainingQuantity string `json:"remaining_quantity"`
	Direction         string `json:"direction"`
	Effect            string `json:"effect"`
	OrderType         string `json:"order_type"`
}

type ContractWithdrawMsg struct {
	Withdraw ContractWithdraw `json:"withdraw"`
}

type ContractWithdraw struct {
	Coins sdk.Coins `json:"coins"`
}

type ContractBalanceResponse struct {
	Balance Balance `json:"balance"`
}

type ContractOrderResponse struct {
	Order Order `json:"order"`
}
