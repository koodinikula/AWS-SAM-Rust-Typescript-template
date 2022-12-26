build:
	cargo lambda build --release

build-HelloWorldRS:
	cargo lambda build  --release
	cp ./target/lambda/lambdars/bootstrap $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap