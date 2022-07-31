# [1.9.0](https://github.com/dragossutu/etherscan/compare/1.8.1...1.9.0) (2022-07-31)


### Features

* **gh-23:** Run tests for x86_64-unknown-linux-musl target, to run on same target as release ([cb964c7](https://github.com/dragossutu/etherscan/commit/cb964c7f1646d6f30d2131db6aac421affe02103)), closes [#23](https://github.com/dragossutu/etherscan/issues/23)

## [1.8.1](https://github.com/dragossutu/etherscan/compare/1.8.0...1.8.1) (2022-07-30)


### Bug Fixes

* **gh-18:** Fix binary path in semantic-release config ([963b441](https://github.com/dragossutu/etherscan/commit/963b441db12a04d0631db376cdcbe1202831b1de))

# [1.8.0](https://github.com/dragossutu/etherscan/compare/1.7.0...1.8.0) (2022-07-30)


### Features

* **gh-18:** Change linux target to build for musl, add cross config to use clux/muslrust:stable Docker image for musl build ([082fb8f](https://github.com/dragossutu/etherscan/commit/082fb8fc034e0c403de78bc156134b6d86d36bac)), closes [#18](https://github.com/dragossutu/etherscan/issues/18)
* **gh-18:** Change linux target to build for musl, add cross config to use clux/muslrust:stable Docker image for musl build ([03a36b8](https://github.com/dragossutu/etherscan/commit/03a36b8db65b4870fdc322285ff7328187549ab0)), closes [#18](https://github.com/dragossutu/etherscan/issues/18)
* **gh-18:** Change linux target to build for musl, use cross to build and add cross config to use clux/muslrust:stable Docker image for musl build ([fd91b9b](https://github.com/dragossutu/etherscan/commit/fd91b9bc2d4c56841362cc41deca47ba6cc1771d)), closes [#18](https://github.com/dragossutu/etherscan/issues/18)

# [1.7.0](https://github.com/dragossutu/etherscan/compare/1.6.0...1.7.0) (2022-07-29)


### Bug Fixes

* **gh-16:** Fix rust toolchain action syntax ([6f4340c](https://github.com/dragossutu/etherscan/commit/6f4340cab80b6b2472e38262e185df9396219f3c))


### Features

* **gh-16:** Build Linux x86_64 binary using cargo instead of cross ([8a59173](https://github.com/dragossutu/etherscan/commit/8a591736deeca840a927dee40c235e8dad0e1fb3)), closes [#16](https://github.com/dragossutu/etherscan/issues/16)
* **gh-16:** Build Linux x86_64 binary using cross ([9b7886c](https://github.com/dragossutu/etherscan/commit/9b7886c8546927bd3676ceb8454a8d10ceca0518)), closes [#16](https://github.com/dragossutu/etherscan/issues/16)
* **gh-16:** Don't create release in pipeline anymore ([f11d6ba](https://github.com/dragossutu/etherscan/commit/f11d6bae8c27960b06c7bd9e4285101579adff86)), closes [#16](https://github.com/dragossutu/etherscan/issues/16)
* **gh-16:** Rename Github Actions workflow, run on push on any branch, install rust stable toolchain in pipeline ([d912c3d](https://github.com/dragossutu/etherscan/commit/d912c3d52aa9f1253622003a7887116eff44248a)), closes [#16](https://github.com/dragossutu/etherscan/issues/16)
* **gh-16:** Run pipeline on main branch only, add Linux x86_64 bianry on release ([8cff25d](https://github.com/dragossutu/etherscan/commit/8cff25d8f88d7bfc7e7914fc06ef9f45ba2bebce)), closes [#16](https://github.com/dragossutu/etherscan/issues/16)
* **gh-16:** Run tests in pipeline ([23d6ec4](https://github.com/dragossutu/etherscan/commit/23d6ec47821dda840b72c5424a05602a353212ae)), closes [#16](https://github.com/dragossutu/etherscan/issues/16)

# [1.6.0](https://github.com/dragossutu/etherscan/compare/1.5.0...1.6.0) (2022-07-24)


### Features

* **#14:** Cargo update ([e19a857](https://github.com/dragossutu/etherscan/commit/e19a85709ced4c9cf31b9b2f6f12f7dda2f42eb5)), closes [#14](https://github.com/dragossutu/etherscan/issues/14)
* **#14:** Rename binary in Changelog URLs ([8c957eb](https://github.com/dragossutu/etherscan/commit/8c957eb6bb4a7736e5b72f628b2214911feae730)), closes [#14](https://github.com/dragossutu/etherscan/issues/14)
* **#14:** Rename crate ([a714527](https://github.com/dragossutu/etherscan/commit/a714527dade2caaf330f56bb9e3cb1ea00d638b8)), closes [#14](https://github.com/dragossutu/etherscan/issues/14)
* **#14:** Update binary name in end_to_end test ([4bbdc72](https://github.com/dragossutu/etherscan/commit/4bbdc72fddf7ef5a8258c4b4d9f49cd1d83fd3f7)), closes [#14](https://github.com/dragossutu/etherscan/issues/14)

# [1.5.0](https://github.com/dragossutu/etherscan/compare/1.4.0...1.5.0) (2022-07-24)


### Features

* **#12:** Move networks to API URLs hashmap from client to main ([8fae61f](https://github.com/dragossutu/etherscan/commit/8fae61f66b61285305b2da1d2d37454bdd78bd28)), closes [#12](https://github.com/dragossutu/etherscan/issues/12)
* **#12:** Remove unnecessary lib.rs, move lib code to main ([cf51694](https://github.com/dragossutu/etherscan/commit/cf516949aa5266cace0c568cb288a087f0ad1671)), closes [#12](https://github.com/dragossutu/etherscan/issues/12)
* **#12:** Replace networks HashMap with match block, replace Strings with &str in Client, update CLI args descriptions ([2c25a89](https://github.com/dragossutu/etherscan/commit/2c25a8970a2696b207a1b5b2808f6344262595d1)), closes [#12](https://github.com/dragossutu/etherscan/issues/12)

# [1.4.0](https://github.com/dragossutu/etherscan/compare/1.3.2...1.4.0) (2022-06-26)


### Features

* **#8:** Add network CLI arg to be able to download contracts from main networks of all the blockchains that use the etherscan block explorer implementation. Add CLI args descriptions. ([d3ff8cd](https://github.com/dragossutu/etherscan/commit/d3ff8cd9f998dc2ecfe696719a23b5f0777824a4)), closes [#8](https://github.com/dragossutu/etherscan/issues/8)
* **#8:** Remove api key file from gitignore ([e461c29](https://github.com/dragossutu/etherscan/commit/e461c29482583f998a852973cb275c6f27e8e198)), closes [#8](https://github.com/dragossutu/etherscan/issues/8)

## [1.3.2](https://github.com/dragossutu/etherscan/compare/1.3.1...1.3.2) (2022-06-20)


### Bug Fixes

* **#9:** Update semantic-release config to run cargo check and commit Cargo.lock file also on release ([388d4ad](https://github.com/dragossutu/etherscan/commit/388d4ade98c2e67af21389d7f89930f4fdb1c0d4)), closes [#9](https://github.com/dragossutu/etherscan/issues/9)

## [1.3.1](https://github.com/dragossutu/etherscan/compare/1.3.0...1.3.1) (2022-06-19)


### Bug Fixes

* **#6:** Make contract address CLI arg posistional again. ([c22c000](https://github.com/dragossutu/etherscan/commit/c22c0001f9f8eb1b307d763e535b401f9d0063a6)), closes [#6](https://github.com/dragossutu/etherscan/issues/6) [#6](https://github.com/dragossutu/etherscan/issues/6)

# [1.3.0](https://github.com/dragossutu/etherscan/compare/1.2.0...1.3.0) (2022-06-19)


### Features

* **rename-crate:** Rename crate, repo and lib function ([8f3f505](https://github.com/dragossutu/etherscan/commit/8f3f505c992ce42bcd6e823a63bfd8b6e80a98a3))

# [1.2.0](https://github.com/dragossutu/esctl/compare/1.1.0...1.2.0) (2022-06-19)


### Features

* **refactor:** Add anyhow context to most errors, add tests for contracts::client and use pretty_assertions crate ([b3c55f2](https://github.com/dragossutu/esctl/commit/b3c55f2aef04a13dd2e09e91206ede9da4909bc2))
* **refactor:** Add CLI arg for API URL, handle default value of api_key_file_path args using clap, sort use statements ([0817268](https://github.com/dragossutu/esctl/commit/0817268b8e612bdea1854e7842d0277f6c2928b8))
* **refactor:** Add integration test with success cases, use anyhow package to add error contexts, replace hardcoded API URL in client with constructor arg ([62e1931](https://github.com/dragossutu/esctl/commit/62e19313855c1760ea237c5f469c82164c2f50fa))
* **refactor:** Cargo update ([d941069](https://github.com/dragossutu/esctl/commit/d94106928bb32f9f115a92b94d685134d73bf944))
* **refactor:** Extract code from main to contracts module and into seprate client and service files and add service happy path test ([5a832fe](https://github.com/dragossutu/esctl/commit/5a832fe6b9a43e31d61650bd0d2e6b147af82bf5))
* **refactor:** Extract code from main.rs to lib.rs to make writting integrations tests easier ([86b572c](https://github.com/dragossutu/esctl/commit/86b572c8d2c83cc53a505597d1e2fcef6611de84))
* **refactor:** Replace shorthand reqwest get() method with Client and call get() on Client ([dd6c263](https://github.com/dragossutu/esctl/commit/dd6c263edc1f8f05be0a2247a3a3530cf340b7c7))
* **refactor:** Run cargo fmt ([8aa6704](https://github.com/dragossutu/esctl/commit/8aa6704ada69d87faf709e99f01dcc88b2197491))

# [1.1.0](https://github.com/dragossutu/esctl/compare/1.0.0...1.1.0) (2022-06-13)


### Features

* **pipeline:** Install toml-cli in github actions pipeline ([e6d60ea](https://github.com/dragossutu/esctl/commit/e6d60ea62a136324545b75b056298b8e70446bd4))
* **pipeline:** Update version in Cargo.toml instead of VERSION file in github actions pipeline ([4cfe3ac](https://github.com/dragossutu/esctl/commit/4cfe3ac029e01911719ebbac596260f0250dd202))

# 1.0.0 (2022-05-31)


### Features

* **github-actions:** Add Github Actions workflows file ([878ef2b](https://github.com/dragossutu/esctl/commit/878ef2b8b560b5940bb9ca83b1d8ed54bd8949e9))
* **github-actions:** Add semantic-release config ([cd13792](https://github.com/dragossutu/esctl/commit/cd137927de26698e80931e2b7260fec14b6084aa))
* **github-actions:** Install semantic-release plugins to update changelog and version files ([8373d72](https://github.com/dragossutu/esctl/commit/8373d723a21cc7785631664f724b5557a4a330aa))
* **github-actions:** Remove setup-node action ([421bff0](https://github.com/dragossutu/esctl/commit/421bff0f260e0537a6e505ba489db2424b055317))
* **github-actions:** Rename env var and secret for github token to GH_TOKEN ([3806347](https://github.com/dragossutu/esctl/commit/38063470724a9a11f7231e9fff3fe86f8fc38953))
* **github-actions:** Run semantic-release Github workflow only on push to main ([4264736](https://github.com/dragossutu/esctl/commit/426473696049bc3363e8fa1693fecf8f299b622b))
* **github-actions:** Use setup-node action version 3 and install semantic-release ([2328dd8](https://github.com/dragossutu/esctl/commit/2328dd8de9158f5418b09d89a703f9d7e1ac9c27))
