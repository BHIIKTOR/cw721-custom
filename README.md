# cw721-offchain-randomization

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