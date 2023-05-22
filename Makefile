ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream:
	substreams run -e $(ENDPOINT) substreams.yaml map_pools_registered -s 15503491 -t +1000

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
