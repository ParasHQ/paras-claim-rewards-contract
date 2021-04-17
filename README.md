PARAS Claim Rewards Contract
==============

## Building this contract
```bash
yarn build
```

## Using this contract

### Quickest deploy
```bash
yarn dev
```

## Testing
To test run:
```bash
yarn test
```

# Contract functions

## View methods

### Get rewards

```
get_rewards({"from_index":"0","limit":10,"account_id":"irfi.testnet"})
```

### Get reward\_amount

```
get_reward_amount({"account_id":"irfi.testnet"})
```

## Call methods

### New 
```
near call --accountId owner.testnet --networkId network_id contract_account new '{"owner":"owner.testnet","token":"ft.paras.testnet"}'
```

### Claim reward

```
claim_reward '{"amount":"1"}' --amount 0.000000000000000000000001
```

### Push reward - Only Owner
```
near call --accountId owner.testnet --networkId network_id contract_account push_reward '{"account_id":"alice.testnet","amount":"10","memo":"second reward"}' --amount 0.000000000000000000000001
```
