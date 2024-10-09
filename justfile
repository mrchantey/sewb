set dotenv-load

default:
	just --list --unsorted

publish:
	cargo publish --allow-dirty --no-verify

app:
	just run app scenes/basics.json scenes/wellbeing-inheritance.json

run example *args:
	cargo run --example {{example}} -- {{args}}

export-scenes:
	just run export_scenes

build *args:
	just export-scenes
	beetmash build \
	--example app \
	--release \
	--copy-local ../beetmash-apps \
	--copy-scenes scenes \
	{{args}}
