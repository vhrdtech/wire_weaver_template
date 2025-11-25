# wire_weaver_template

## TODO
* [ ] Checkout the `api` crate [README](api/README.md)
* [ ] Run one of the firmwares
  * [ ] If you do not have hardware to run it - adapt to other dev. board with USB that embassy supports
  * [ ] TODO: Alternatively - run the virtual device emulator (can later be used for integration tests as well)
* [ ] Run driver/examples/simple: `cargo run -p driver --example simple`
* [ ] Optionally checkout the REPL example in driver/examples/repl: `cargo run -p driver --example repl`
* [ ] Rename `api` crate to your name (this will break the code in several places, TODO: add rename script)

## Syncing upstream template changes
Add upstream:
`git remote add upstream https://github.com/vhrdtech/wire_weaver_template.git`

Fetch changes from upstream:
`git fetch upstream`

Merge:
`git merge upstream/main --allow-unrelated-histories`