package driver

import (
	"github.com/sei-protocol/spot-integration-tests/parser"
)

type Handler struct {
	monikerToOrderIds map[string]uint64
}

func NewHandler() Handler {
	return Handler{
		monikerToOrderIds: map[string]uint64{},
	}
}

func (h *Handler) HandleInputs(config parser.EncodingConfig, inputs []parser.Input, contractAddr string) {
	for _, input := range inputs {
		switch input.InputType {
		case parser.INPUT_TYPE_ORDER_PLACEMENT:
			h.handleOrder(config, parser.ParseFundedOrder(input.Details), contractAddr)
		case parser.INPUT_TYPE_ORDER_CANCELLATION:
			h.handleCancel(config, parser.ParseCancel(input.Details), contractAddr)
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
