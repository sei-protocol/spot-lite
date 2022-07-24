package driver

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/cosmos/cosmos-sdk/crypto/hd"
	"github.com/cosmos/cosmos-sdk/crypto/keyring"
	cryptotypes "github.com/cosmos/cosmos-sdk/crypto/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func GetKey(accountName string) cryptotypes.PrivKey {
	mnemonic := getMnemonic(accountName)
	keyringAlgos := keyring.SigningAlgoList{hd.Secp256k1}
	algoStr := string(hd.Secp256k1Type)
	algo, _ := keyring.NewSigningAlgoFromString(algoStr, keyringAlgos)
	hdpath := hd.CreateHDPath(sdk.GetConfig().GetCoinType(), 0, 0).String()
	derivedPriv, _ := algo.Derive()(mnemonic, "", hdpath)
	return algo.Generate()(derivedPriv)
}

func GetAddress(accountName string) string {
	resp, err := http.Get(KEY_SERVER)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	var keys map[string]interface{}
	if err := json.Unmarshal(body, &keys); err != nil {
		panic(err)
	}
	key := keys[accountName].(map[string]interface{})
	address := key["address"].(string)
	return address[:len(address)-1] // remove newline character
}

func getMnemonic(accountName string) string {
	resp, err := http.Get(KEY_SERVER)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	var keys map[string]interface{}
	if err := json.Unmarshal(body, &keys); err != nil {
		panic(err)
	}
	key := keys[accountName].(map[string]interface{})
	mnemonic := key["mnemonic"].(string)
	return mnemonic[:len(mnemonic)-1] // remove newline character
}
