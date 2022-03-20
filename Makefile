.PHONY: doc
doc:
	cd common_concept && cargo doc --no-deps
	rm -rf ./docs
	cp -r common_concept/target/doc ./docs