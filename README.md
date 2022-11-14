# cw721-offchain-randomization

The whole idea of this contract is to allow people to skip the two contract layout for usual token contracts in CW.
It lacks things, like minting a random token instead of doing it sequentially, but this is a WPI (so far).

The contract allows for three different types of storing methods:
* store - 1:1 data storage (based on opensea medata)
* store batch - 1:1 data storage (based on opensea medata) but in batch
* store conf - and optimized method that allows to send the repetitive info once and the attributes for the metadata in mass

It also allows to mint one token or mint many.

It also allows the new owner of the token to burn their tokens based on configuration and disallow the original minter to rebuy or burn the tokens

## Quick list of features

* Owners can burn tokens
* Configurable denom and amount
* Increasiable current supply but configurable total supply
* Mint start date
* Funds wallet
* Max mint batch
* InitMsg store conf (see exameple below)

## InitMsg

```Rust
pub struct InstantiateMsg {
    // Name of the NFT contract
    pub name: String,

    // Symbol of the NFT contract
    pub symbol: String,

    // minter
    pub minter: String, // who can mint, passed to the CW721 base contract

    // when the minting can start?
    pub start_mint: Option<Timestamp>,

    pub cost_denom: String, // name of the token
    pub cost_amount: Uint128, // amount

    // maximum token supply
    pub token_supply: Uint128,

    // wallet that recieves the funds
    pub funds_wallet: String,

    /// max size of minting batch
    pub max_mint_batch: Uint128,

    // turn this ON to allow holders of the nft to burn their tokens
    pub owners_can_burn: bool,

    // turn this off to do not allow the contract owner to burn tokens
    pub minter_can_burn: bool,

    pub store_conf: Option<StoreConf>,
}
```

### InitMsg JSON example

```JSON
{
  "name": "",
  "symbol": "",
  "minter": "",
  "cost_denom": "uluna",
  "cost_amount": "5000000",
  "token_supply": "100000",
  "start_mint": "1667925343853",
  "max_mint_batch": "10",
  "owners_can_burn": true,
  "minter_can_burn": false,
  "funds_wallet": ""
}
```

## Store conf msg syntax

```JSON
{
  "store_conf": {
    "conf": {
      "desc": "",
      "ipfs": "",
      "name": "",
      "attributes": []
    },
    "attributes": []
}
```

## Store conf msg example

```JSON
{
  "store_conf": {
    "conf": {
      "desc": "my amazing nft",
      "ipfs": "https://ipfs.com/",
      "name": "MyNFT",
      "attributes": [
        "background",
        "color",
        "something"
      ]
    },
    "attributes": [
      [0, 1, 2],
      [3, 2, 3],
      [5, 7, 8]
    ]
}
```