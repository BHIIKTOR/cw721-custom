# cw721-custom

The whole idea of this contract is to implement the CW721 standard.

It lacks things, like minting a random token instead of doing it sequentially, but this is yet a WPI (so far).

The contract allows for three different types of methods for storing token metadata:
* store - stores one token metadata
* store batch - batch metadata token storing with "heavy" costs
* store conf - An optimized TX that accepts the repetitive info once and the attributes for the metadata in bulk

It also allows to mint one token or mint in batch with configurable max amount.

It also allows the new owner of the token to burn their tokens based on configuration
and disallow the original minter to rebuy or burn the tokens

## Quick list of features (so far).

* Batch burn and mint
* Configuration for token owners to be able to burn tokens
* Configurable denom and amount
* Increasiable current supply and configurable total supply
* Mint start and end date
* It sends the funds to a configured wallet
* Max mint batch
* Toggle freeze contract operations
* InitMsg store conf (see exameple below)
* Toggle pause and freeze
* Migrate with clear of state and without but both with config

## InitMsg

```Rust
pub struct InstantiateMsg {
    // defaults to this msg sender
    pub admin: Option<String>,

    // Name of the NFT contract
    pub name: String,

    // symbol of the NFT
    pub symbol: String,

    // end and start date of minting, optional
    pub dates: mint::Dates,

    // cost amount and name of the denom
    pub cost: mint::Costs,

    // burn cofiguration to allow only admin or token owners
    pub burn: mint::Burn,

    // maximum token supply
    pub token_supply: Uint128,

    // wallet that recieves the funds
    pub wallet: mint::Wallet,

    // defaults to 10
    pub max_mint_batch: Option<Uint128>,

    // Used for StoreConf call but can be provided during the call
    pub store_conf: StoreConf,
}
```

### mint::Burn structure

```Rust
pub struct Burn {
  // peoplo who whold the tokens
  pub owners: bool,

  // the admin of the contract
  pub admin: Option<String>,

  // admin can burn owned
  pub can_burn_owned: bool
}
```

### mint::Dates structure

```Rust
// Start and end dates of the minting, both are optional
// if there is a start it will accept mints in that date
// if there is no end it's endless
#[cw_serde]
#[derive(Default)]
pub struct Dates {
  pub start: Option<Timestamp>,
  pub end: Option<Timestamp>,
}
```

### mint::Costs structure

```Rust
#[cw_serde]
pub struct Costs {
  pub denom: String,
  pub amount: Uint128,
}

impl Default for Costs {
    fn default() -> Self {
        Self {
          denom: String::from("ujuno"),
          amount: Uint128::from(1000000u64)
        }
    }
}
```

### mint::Wallet structure

```Rust
// sends funds to this wallet
#[cw_serde]
pub struct Wallet {
  pub name: String,
  pub wallet: Addr,
}

impl Default for Wallet {
    fn default() -> Self {
        Self {
            name: String::from(""),
            wallet: Addr::unchecked(String::from("")),
        }
    }
}
```

## Store conf msg syntax

```Rust
pub struct StoreConf {
    pub name: String,
    pub desc: String,
    pub ipfs: String,
    pub attributes: Vec<String>,
}

impl Default for StoreConf {
    fn default() -> Self {
        Self {
            name: String::from("nft"),
            desc: String::from("desc"),
            ipfs: String::from("ipfs://"),
            attributes: vec![],
        }
    }
}
```

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