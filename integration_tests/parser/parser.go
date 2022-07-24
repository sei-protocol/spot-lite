package parser

import (
	"encoding/json"
	"io/ioutil"
	"os"
)

func ParseTestFile(filename string) Test {
	pwd, _ := os.Getwd()
	file, _ := ioutil.ReadFile(pwd + "/tests/" + filename + ".json")
	test := Test{}
	if err := json.Unmarshal([]byte(file), &test); err != nil {
		panic(err)
	}
	return test
}

func ParseFundedOrder(raw json.RawMessage) FundedOrder {
	order := FundedOrder{}
	if err := json.Unmarshal(raw, &order); err != nil {
		panic(err)
	}
	return order
}

func ParseCancel(raw json.RawMessage) Cancel {
	cancel := Cancel{}
	if err := json.Unmarshal(raw, &cancel); err != nil {
		panic(err)
	}
	return cancel
}

func ParseBalance(balanceBytes []byte) Balance {
	var res ContractBalanceResponse
	if err := json.Unmarshal(balanceBytes, &res); err != nil {
		panic(err)
	}
	return res.Balance
}

func ParseOrder(orderBytes []byte) Order {
	var res ContractOrderResponse
	if err := json.Unmarshal(orderBytes, &res); err != nil {
		panic(err)
	}
	return res.Order
}
