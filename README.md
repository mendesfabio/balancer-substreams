# Balancer Substreams

Extracts all pools registered to Balancer Vault with its tokens balances and users internal balances.

## Generating Protobuf

```bash
make protogen
```

## Compile

```bash
make build
```

## Run Substream

```bash
# run the graph_out module
make stream

```

## Pack to release

```bash
make package

```

## Deploy Substream

```bash
# have to change slug in Makefile
make deploy

```

## Visual Data Flow

```mermaid
graph TD;
  map_pools_registered[map: map_pools_registered];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_pools_registered;
  store_pools[store: store_pools];
  map_pools_registered --> store_pools;
  store_vault_pool_count[store: store_vault_pool_count];
  map_pools_registered --> store_vault_pool_count;
  map_pool_tokens_registered[map: map_pool_tokens_registered];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_pool_tokens_registered;
  store_pools --> map_pool_tokens_registered;
  store_pool_tokens[store: store_pool_tokens];
  map_pool_tokens_registered --> store_pool_tokens;
  store_erc20_tokens[store: store_erc20_tokens];
  map_pool_tokens_registered --> store_erc20_tokens;
  map_join_exit_balance_changes[map: map_join_exit_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_join_exit_balance_changes;
  store_pool_tokens --> map_join_exit_balance_changes;
  map_managed_balance_changes[map: map_managed_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_managed_balance_changes;
  store_pool_tokens --> map_managed_balance_changes;
  map_swap_balance_changes[map: map_swap_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_swap_balance_changes;
  store_pool_tokens --> map_swap_balance_changes;
  store_pool_token_balances[store: store_pool_token_balances];
  map_join_exit_balance_changes --> store_pool_token_balances;
  map_managed_balance_changes --> store_pool_token_balances;
  map_swap_balance_changes --> store_pool_token_balances;
  map_internal_balance_changes[map: map_internal_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_internal_balance_changes;
  store_user_internal_balances[store: store_user_internal_balances];
  map_internal_balance_changes --> store_user_internal_balances;
  store_internal_balance_changes[store: store_internal_balance_changes];
  map_internal_balance_changes --> store_internal_balance_changes;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  map_pools_registered --> graph_out;
  map_pool_tokens_registered --> graph_out;
  map_internal_balance_changes --> graph_out;
  store_erc20_tokens --> graph_out;
  store_user_internal_balances --> graph_out;
  store_vault_pool_count -- deltas --> graph_out;
  store_internal_balance_changes -- deltas --> graph_out;
  store_pool_token_balances -- deltas --> graph_out;

```
