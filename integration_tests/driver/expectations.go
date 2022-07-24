package driver

import (
	"math"
	"testing"

	"github.com/sei-protocol/spot-integration-tests/parser"
	"github.com/stretchr/testify/assert"
)

var EPSILON float64 = math.Pow(10, -8)

func (h *Handler) VerifyExpectations(t *testing.T, expectations parser.Expectations, contractAddr string) {
	for _, balance := range expectations.Balances {
		h.verifyBalance(t, balance, contractAddr)
	}
	for _, order := range expectations.Orders {
		h.verifyOrder(t, order, contractAddr)
	}
}

func (h *Handler) verifyBalance(t *testing.T, balanceExpectation parser.BalanceExpectation, contractAddr string) {
	balance := QueryBalance(GetAddress(balanceExpectation.Account), balanceExpectation.Denom, contractAddr)
	assert.Equal(t, balance, balanceExpectation.Balance)
}

func (h *Handler) verifyOrder(t *testing.T, orderExpectation parser.OrderExpectation, contractAddr string) {
	orderID := h.monikerToOrderIds[orderExpectation.Moniker]
	order := QueryOrder(orderID, contractAddr)
	orderExpectation.Order.Id = orderID
	orderExpectation.Order.Account = order.Account
	assert.Equal(t, order, orderExpectation.Order)
}
