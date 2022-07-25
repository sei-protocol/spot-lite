package driver

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/sei-protocol/spot-integration-tests/parser"
)

type Handler struct {
	monikerToOrderIds         map[string]uint64
	keyNameToStartingBalances map[string]sdk.Coins
}

func NewHandler() Handler {
	return Handler{
		monikerToOrderIds:         map[string]uint64{},
		keyNameToStartingBalances: map[string]sdk.Coins{},
	}
}

func (h *Handler) HandleInputs(config parser.EncodingConfig, inputs []parser.Input, contractAddr string) {
	for _, input := range inputs {
		switch input.InputType {
		case parser.INPUT_TYPE_ORDER_PLACEMENT:
			h.handleOrder(config, parser.ParseFundedOrder(input.Details), contractAddr)
		case parser.INPUT_TYPE_ORDER_CANCELLATION:
			h.handleCancel(config, parser.ParseCancel(input.Details), contractAddr)
		case parser.INPUT_TYPE_LOAD_STARTING_BALANCE:
			h.handleLoadStartingBalance(config, parser.ParseStartingBalance(input.Details))
		case parser.INPUT_TYPE_DEPOSIT:
			h.handleDeposit(config, parser.ParseDeposit(input.Details), contractAddr)
		case parser.INPUT_TYPE_WITHDRAW:
			h.handleWithdraw(config, parser.ParseWithdraw(input.Details), contractAddr)
		default:
			panic("Unknown input type")
		}
	}
}

func (h *Handler) handleOrder(config parser.EncodingConfig, order parser.FundedOrder, contractAddr string) {
	if _, ok := h.monikerToOrderIds[order.Moniker]; ok {
		panic("Duplicated moniker")
	}
	key := GetKey(order.Account)
	orderId := SendOrder(config, key, order, contractAddr)
	h.monikerToOrderIds[order.Moniker] = orderId
}

func (h *Handler) handleCancel(config parser.EncodingConfig, cancel parser.Cancel, contractAddr string) {
	key := GetKey(cancel.Account)
	SendCancel(config, key, cancel, contractAddr, h.monikerToOrderIds)
}

func (h *Handler) handleLoadStartingBalance(config parser.EncodingConfig, startingBalance parser.StartingBalance) {
	if _, ok := h.keyNameToStartingBalances[startingBalance.Account]; !ok {
		h.keyNameToStartingBalances[startingBalance.Account] = sdk.NewCoins()
	}
	balance := GetBankBalance(startingBalance.Account, startingBalance.Denom)
	h.keyNameToStartingBalances[startingBalance.Account] = append(h.keyNameToStartingBalances[startingBalance.Account], balance)
}

func (h *Handler) handleDeposit(config parser.EncodingConfig, deposit parser.Deposit, contractAddr string) {
	key := GetKey(deposit.Account)
	Deposit(config, key, deposit, contractAddr)
}

func (h *Handler) handleWithdraw(config parser.EncodingConfig, withdraw parser.Withdraw, contractAddr string) {
	key := GetKey(withdraw.Account)
	Withdraw(config, key, withdraw, contractAddr)
}
