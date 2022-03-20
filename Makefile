.PHONY: doc
doc:
	cd common_concept && cargo doc --no-deps
	rm -rf ./docs
	echo "<meta http-equiv=\"refresh\" content=\"0; url=build_wheel\">" > common_concept/target/doc/index.html
	cp -r common_concept/target/doc ./docs