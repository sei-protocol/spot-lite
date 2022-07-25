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

func ParseDeposit(raw json.RawMessage) Deposit {
	deposit := Deposit{}
	if err := json.Unmarshal(raw, &deposit); err != nil {
		panic(err)
	}
	return deposit
}

func ParseWithdraw(raw json.RawMessage) Withdraw {
	withdraw := Withdraw{}
	if err := json.Unmarshal(raw, &withdraw); err != nil {
		panic(err)
	}
	return withdraw
}

func ParseCancel(raw json.RawMessage) Cancel {
	cancel := Cancel{}
	if err := json.Unmarshal(raw, &cancel); err != nil {
		panic(err)
	}
	return cancel
}

func ParseStartingBalance(raw json.RawMessage) StartingBalance {
	startingBalance := StartingBalance{}
	if err := json.Unmarshal(raw, &startingBalance); err != nil {
		panic(err)
	}
	return startingBalance
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
