# GraphQL Test for Holochain

## build

`hc package --strip-meta`

## run

`mkdir -c ./tmp-storage && holochain_container -c ./container-config.toml`

and to test use

`curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "graphql-test/users/main/graphql", "id":123, "params": {"query": "query { apiVersion }"} }' localhost:3400`