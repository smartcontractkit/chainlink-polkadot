.DEFAULT_GOAL=help

# Show this help.
help:
	@awk '/^#/{c=substr($$0,3);next}c&&/^[[:alpha:]][[:print:]]+:/{print substr($$1,1,index($$1,":")),c}1{c=0}' $(MAKEFILE_LIST) | column -s: -t

# Run the example chain
run-chain:
	cd substrate-node-example; cargo run --release -- --dev

purge-chain:
	cd substrate-node-example; cargo run --release -- purge-chain --dev

run-front-end:
	cd substrate-node-example/front-end; yarn start