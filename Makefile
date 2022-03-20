.PHONY: doc
doc:
	cd common_concept && cargo doc --no-deps
	rm -rf ./docs
	echo "<meta http-equiv=\"refresh\" content=\"0; url=common_concept\">" > common_concept/target/doc/index.html
	cp -r common_concept/target/doc ./docs