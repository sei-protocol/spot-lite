package parser

import (
	"encoding/json"

	sdk "github.com/cosmos/cosmos-sdk/types"
	dextypes "github.com/sei-protocol/sei-chain/x/dex/types"
	dextypesutils "github.com/sei-protocol/sei-chain/x/dex/types/utils"
)

func ToSeiOrderPlacement(fundedOrder FundedOrder) dextypes.Order {
	order := fundedOrder.Order
	positionDirection, err := dextypesutils.GetPositionDirectionFromStr(order.PositionDirection)
	if err != nil {
		panic(err)
	}
	orderType, err := dextypesutils.GetOrderTypeFromStr(order.OrderType)
	if err != nil {
		panic(err)
	}
	price := sdk.MustNewDecFromStr(order.Price)
	quantity := sdk.MustNewDecFromStr(order.Quantity)
	orderData := OrderData{
		PositionEffect: order.PositionEffect,
	}
	orderDataBz, err := json.Marshal(orderData)
	if err != nil {
		panic(err)
	}
	return dextypes.Order{
		Account:           fundedOrder.Account,
		PositionDirection: positionDirection,
		Price:             price,
		Quantity:          quantity,
		PriceDenom:        order.PriceDenom,
		AssetDenom:        order.AssetDenom,
		Data:              string(orderDataBz),
		OrderType:         orderType,
	}
}
