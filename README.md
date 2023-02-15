# cw721-custom

The whole idea of this contract is to allow people to skip the two contract layout for usual token contracts in CW.

It lacks things, like minting a random token instead of doing it sequentially, but this is a WPI (so far).

The contract allows for three different types of storing methods:
* store - 1:1 data storage (based on opensea meta-data)
* store batch - 1:1 data storage (based on opensea meta-data) but in batch
* store conf - An optimized TX that accepts the repetitive info once and the attributes for the metadata in bulk

It also allows to mint one token or mint in batch with configurable max amount.

It also allows the new owner of the token to burn their tokens based on configuration
and disallow the original minter to rebuy or burn the tokens

## Quick list of features (so far).

* Batch burn and mint
* Configuration for token owners to be able to burn tokens
* Configurable denom and amount
* Increasiable current supply and configurable total supply
* Mint start date
* Funds wallet
* Max mint batch
* Toggle freeze contract operations
* Migratable
* InitMsg store conf (see exameple below)

## InitMsg

```Rust
pub struct InstantiateMsg {
    // Name of the NFT contract
    pub name: String,

    pub symbol: String,

    pub minter: String,

    pub dates: mint::Dates,

    pub cost: mint::Costs,

    pub burn: mint::Burn,

    // maximum token supply
    pub token_supply: Uint128,

    // wallet that recieves the funds
    pub funds_wallet: String,

    // defaults to 10
    pub max_mint_batch: Option<Uint128>,

    // Used for StoreConf call but can be provided during the call
    pub store_conf: Option<StoreConf>,
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