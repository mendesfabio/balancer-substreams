.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_pools_registered -s 12292922 -t +10

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
