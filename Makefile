ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream:
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 12286257 -t +1000

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONE: package
package:
	substreams pack -o substreams.spkg substreams.yaml

.PHONE: deploy
deploy:
	graph deploy --product hosted-service mendesfabio/balancer-substreams

.PHONE: test
test:
	cargo test --target aarch64-apple-darwin