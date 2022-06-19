## [1.3.1](https://github.com/dragossutu/scancli/compare/1.3.0...1.3.1) (2022-06-19)


### Bug Fixes

* **#6:** Make contract address CLI arg posistional again. ([c22c000](https://github.com/dragossutu/scancli/commit/c22c0001f9f8eb1b307d763e535b401f9d0063a6)), closes [#6](https://github.com/dragossutu/scancli/issues/6) [#6](https://github.com/dragossutu/scancli/issues/6)

# [1.3.0](https://github.com/dragossutu/scancli/compare/1.2.0...1.3.0) (2022-06-19)


### Features

* **rename-crate:** Rename crate, repo and lib function ([8f3f505](https://github.com/dragossutu/scancli/commit/8f3f505c992ce42bcd6e823a63bfd8b6e80a98a3))

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
