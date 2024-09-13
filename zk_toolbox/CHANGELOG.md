# Changelog

## [0.1.3](https://github.com/matter-labs/zksync-era/compare/zk_toolbox-v0.1.2...zk_toolbox-v0.1.3) (2024-09-13)


### Features

* Add `gateway_url` to EN config ([#2698](https://github.com/matter-labs/zksync-era/issues/2698)) ([cfdda01](https://github.com/matter-labs/zksync-era/commit/cfdda019afe26810234285411eba79ada472c888))
* add `update-token-multiplier-setter` command ([#2688](https://github.com/matter-labs/zksync-era/issues/2688)) ([0275435](https://github.com/matter-labs/zksync-era/commit/0275435c7f3f35d9ecf7b65b06d217987cd7a505))
* add dapp-portal support to zk_inception ([#2659](https://github.com/matter-labs/zksync-era/issues/2659)) ([835d2d3](https://github.com/matter-labs/zksync-era/commit/835d2d38d533ccd4149fa481e8b66eb9d5815d8a))
* allow specifying what tests to run with zks ([#2841](https://github.com/matter-labs/zksync-era/issues/2841)) ([57f56fb](https://github.com/matter-labs/zksync-era/commit/57f56fb87a5899ddb2e82f4cfe4d182bdba496cf))
* attester committees data extractor (BFT-434) ([#2684](https://github.com/matter-labs/zksync-era/issues/2684)) ([92dde03](https://github.com/matter-labs/zksync-era/commit/92dde039ee8a0bc08e2019b7fa6f243a34d9816f))
* Bump crypto and protocol deps ([#2825](https://github.com/matter-labs/zksync-era/issues/2825)) ([a5ffaf1](https://github.com/matter-labs/zksync-era/commit/a5ffaf1b4e291d6f09ba8c1f224f5900665bffc4))
* Move prover data to /home/popzxc/workspace/current/zksync-era/prover/data ([#2778](https://github.com/matter-labs/zksync-era/issues/2778)) ([62e4d46](https://github.com/matter-labs/zksync-era/commit/62e4d4619dde9d6bd9102f1410eea75b0e2051c5))
* Provide easy prover setup ([#2683](https://github.com/matter-labs/zksync-era/issues/2683)) ([30edda4](https://github.com/matter-labs/zksync-era/commit/30edda404193938fbd55815bed164b5321d7c642))
* **zk_toolbox:** `zk_supervisor prover` subcommand ([#2820](https://github.com/matter-labs/zksync-era/issues/2820)) ([3506731](https://github.com/matter-labs/zksync-era/commit/3506731d1702bdec8c6b5b41cabca9a257f0269b))
* **zk_toolbox:** Add block explorer support to zk_toolbox ([#2768](https://github.com/matter-labs/zksync-era/issues/2768)) ([1559afb](https://github.com/matter-labs/zksync-era/commit/1559afbd14d5fe78c4ab2a5ef593403302e355f1))
* **zk_toolbox:** Add external_node consensus support ([#2821](https://github.com/matter-labs/zksync-era/issues/2821)) ([4a10d7d](https://github.com/matter-labs/zksync-era/commit/4a10d7d9554d6c1aa2f4fc46557d40baaad8ff2f))
* **zk_toolbox:** Add holesky testnet as layer1 network ([#2632](https://github.com/matter-labs/zksync-era/issues/2632)) ([d9266e5](https://github.com/matter-labs/zksync-era/commit/d9266e5ef3910732666c00c1324256fb5b54452d))
* **zk_toolbox:** add multi-chain CI integration test ([#2594](https://github.com/matter-labs/zksync-era/issues/2594)) ([05c940e](https://github.com/matter-labs/zksync-era/commit/05c940efbd93023c315e5e13c98faee2153cc1cd))
* **zk_toolbox:** Add setup keys step to prover init ([#2811](https://github.com/matter-labs/zksync-era/issues/2811)) ([0a9e096](https://github.com/matter-labs/zksync-era/commit/0a9e0961758e0b6274f1ac68d0b50ce5344ef14a))
* **zk_toolbox:** Add zks contracts ([#2781](https://github.com/matter-labs/zksync-era/issues/2781)) ([19ca512](https://github.com/matter-labs/zksync-era/commit/19ca51208db5c739d3f3e66f47d68f451997fa8e))
* **zk_toolbox:** Allow running docker images for provers ([#2800](https://github.com/matter-labs/zksync-era/issues/2800)) ([bcb176b](https://github.com/matter-labs/zksync-era/commit/bcb176b8bb033d9cc0fd6bf7e971930c97c91d81))
* **zk_toolbox:** Deploy ConsensusRegistry (BFT-504)  ([#2713](https://github.com/matter-labs/zksync-era/issues/2713)) ([cab13b8](https://github.com/matter-labs/zksync-era/commit/cab13b8d36fbcd7a49073904f1d04bfc36e81645))
* **zk_toolbox:** deploy legacy bridge ([#2837](https://github.com/matter-labs/zksync-era/issues/2837)) ([93b4e08](https://github.com/matter-labs/zksync-era/commit/93b4e08257802d11108870d867dd59fa35e52733))
* **zk_toolbox:** Ease requirements, add option to download setup keys ([#2784](https://github.com/matter-labs/zksync-era/issues/2784)) ([4a4d87e](https://github.com/matter-labs/zksync-era/commit/4a4d87e6c5ad37598a82cbc377b33ba951869adc))
* **zk_toolbox:** Migrate CI unit tests to zk_toolbox ([#2759](https://github.com/matter-labs/zksync-era/issues/2759)) ([da5cafe](https://github.com/matter-labs/zksync-era/commit/da5cafe41b9ab0d4fd64779382987433903b1226))
* **zk_toolbox:** Move check sql to the lint step ([#2757](https://github.com/matter-labs/zksync-era/issues/2757)) ([dcd3727](https://github.com/matter-labs/zksync-era/commit/dcd3727e0426ff93a79eeec50e8576465a0dff7c))
* **zk_toolbox:** Update lint CI with `zk_toolbox` ([#2694](https://github.com/matter-labs/zksync-era/issues/2694)) ([7e122e9](https://github.com/matter-labs/zksync-era/commit/7e122e993d78656f2088070f09354e332258d686))
* **zk_toolbox:** Update rust for zk_toolbox ([#2730](https://github.com/matter-labs/zksync-era/issues/2730)) ([30e072b](https://github.com/matter-labs/zksync-era/commit/30e072bd695615b0095c8bdcfd62b77c6b0ae5e6))


### Bug Fixes

* **api:** `tx.gas_price` field ([#2734](https://github.com/matter-labs/zksync-era/issues/2734)) ([aea3726](https://github.com/matter-labs/zksync-era/commit/aea3726c88b4e881bcd0f4a60ff32a730f200938))
* count SECP256 precompile to account validation gas limit as well ([#2859](https://github.com/matter-labs/zksync-era/issues/2859)) ([fee0c2a](https://github.com/matter-labs/zksync-era/commit/fee0c2ad08a5ab4a04252765b367eb9fbb1f3db7))
* **lint:** Move ignore files out from the code ([#2756](https://github.com/matter-labs/zksync-era/issues/2756)) ([bf03274](https://github.com/matter-labs/zksync-era/commit/bf0327497d2f2d03dbaa87c831598262cb562c65))
* **zk_toolbox:** fix port offset for new chains ([#2803](https://github.com/matter-labs/zksync-era/issues/2803)) ([9821a20](https://github.com/matter-labs/zksync-era/commit/9821a20018c367ce246dba656daab5c2e7757973))
* **zk_toolbox:** secrets path, artifacts path ([#2850](https://github.com/matter-labs/zksync-era/issues/2850)) ([3b5e4a6](https://github.com/matter-labs/zksync-era/commit/3b5e4a69d7dbc43ea3460f4c7c57cf3ef6847b11))
* **zk_toolbox:** various ways of writing zksync ([#2752](https://github.com/matter-labs/zksync-era/issues/2752)) ([ca9d56b](https://github.com/matter-labs/zksync-era/commit/ca9d56b5fa5c6a27a10b6002f8f3cdf97427eb94))
* **zk-toolbox:** Make token multiplier optional ([#2843](https://github.com/matter-labs/zksync-era/issues/2843)) ([89fcb3a](https://github.com/matter-labs/zksync-era/commit/89fcb3a4a29c9831141234a1b4ca6b1d4df48b98))
* **zk-toolbox:** use chain admin for bridgehub  ([#2857](https://github.com/matter-labs/zksync-era/issues/2857)) ([ccf1b63](https://github.com/matter-labs/zksync-era/commit/ccf1b6352f6db56bcb4b67d53564a3919532efeb))

## [0.1.2](https://github.com/matter-labs/zksync-era/compare/zk_toolbox-v0.1.1...zk_toolbox-v0.1.2) (2024-08-20)


### Features

* Poll the main node API for attestation status - relaxed (BFT-496) ([#2583](https://github.com/matter-labs/zksync-era/issues/2583)) ([b45aa91](https://github.com/matter-labs/zksync-era/commit/b45aa9168dd66d07ca61c8bb4c01f73dda822040))
* update base token rate on L1 ([#2589](https://github.com/matter-labs/zksync-era/issues/2589)) ([f84aaaf](https://github.com/matter-labs/zksync-era/commit/f84aaaf723c876ba8397f74577b8c5a207700f7b))
* **zk_toolbox:** Add installation script ([#2569](https://github.com/matter-labs/zksync-era/issues/2569)) ([009cd97](https://github.com/matter-labs/zksync-era/commit/009cd9771821a7ae356356f97813d74fab8512b5))
* **zk_toolbox:** Add lint command ([#2626](https://github.com/matter-labs/zksync-era/issues/2626)) ([3d02946](https://github.com/matter-labs/zksync-era/commit/3d0294695343e11b62fdc7375e6c3bc3a72ffcd9))
* **zk_toolbox:** Add observability interactive option ([#2592](https://github.com/matter-labs/zksync-era/issues/2592)) ([3aeaaed](https://github.com/matter-labs/zksync-era/commit/3aeaaedcf9b41b3a033acfa0ec08e3bf966ab4a9))
* **zk_toolbox:** Add zk_supervisor run unit tests command ([#2610](https://github.com/matter-labs/zksync-era/issues/2610)) ([fa866cd](https://github.com/matter-labs/zksync-era/commit/fa866cd5c7b1b189901b4f7ce6f91886e7aec7e4))
* **zk_toolbox:** Add zk_supervisor test l1 contracts command ([#2613](https://github.com/matter-labs/zksync-era/issues/2613)) ([931e452](https://github.com/matter-labs/zksync-era/commit/931e4529d964d01268cb5965877f3d81d32c921e))
* **zk_toolbox:** Add zk_supervisor test prover command ([#2614](https://github.com/matter-labs/zksync-era/issues/2614)) ([0fe173b](https://github.com/matter-labs/zksync-era/commit/0fe173bd8b337637f457542e0d675cf42b6ecc65))
* **zk_toolbox:** allow to run `zk_inception chain create` non-interactively ([#2579](https://github.com/matter-labs/zksync-era/issues/2579)) ([555fcf7](https://github.com/matter-labs/zksync-era/commit/555fcf79bc950f79e218697be9f1a316e4723322))
* **zk_toolbox:** Minting base token ([#2571](https://github.com/matter-labs/zksync-era/issues/2571)) ([ae2dd3b](https://github.com/matter-labs/zksync-era/commit/ae2dd3bbccdffc25b040313b2c7983a936f36aac))
* **zk_toolbox:** Run formatters and linterrs ([#2675](https://github.com/matter-labs/zksync-era/issues/2675)) ([caedd1c](https://github.com/matter-labs/zksync-era/commit/caedd1c86eedd94f8628bd2ba1cf875cad9a53d1))


### Bug Fixes

* Bump prover dependencies & rust toolchain ([#2600](https://github.com/matter-labs/zksync-era/issues/2600)) ([849c6a5](https://github.com/matter-labs/zksync-era/commit/849c6a5dcd095e8fead0630a2a403f282c26a2aa))
* **zk_toolbox:** Do not panic during mint ([#2658](https://github.com/matter-labs/zksync-era/issues/2658)) ([1a8ee90](https://github.com/matter-labs/zksync-era/commit/1a8ee90d9d6578492806bd0a337ef203db32f6c9))
* **zk_toolbox:** Get l1-network config param from flag ([#2603](https://github.com/matter-labs/zksync-era/issues/2603)) ([553d307](https://github.com/matter-labs/zksync-era/commit/553d307217282b18c2c3d7cc6f340f529bb4ade2))

## [0.1.1](https://github.com/matter-labs/zksync-era/compare/zk_toolbox-v0.1.0...zk_toolbox-v0.1.1) (2024-08-02)


### Features

* Add recovery tests to zk_supervisor ([#2444](https://github.com/matter-labs/zksync-era/issues/2444)) ([0c0d10a](https://github.com/matter-labs/zksync-era/commit/0c0d10af703d3f8958c49d0ed46d6cda64945fa1))
* add revert tests (external node) to zk_toolbox ([#2408](https://github.com/matter-labs/zksync-era/issues/2408)) ([3fbbee1](https://github.com/matter-labs/zksync-era/commit/3fbbee10be99e8c5a696bfd50d81230141bccbf4))
* add revert tests to zk_toolbox ([#2317](https://github.com/matter-labs/zksync-era/issues/2317)) ([c9ad002](https://github.com/matter-labs/zksync-era/commit/c9ad002d17ed91d1e5f225e19698c12cb3adc665))
* add zk toolbox ([#2005](https://github.com/matter-labs/zksync-era/issues/2005)) ([60a633b](https://github.com/matter-labs/zksync-era/commit/60a633b23eaf25658d86f090e7954843d4daca42))
* Adding SLChainID ([#2547](https://github.com/matter-labs/zksync-era/issues/2547)) ([656e830](https://github.com/matter-labs/zksync-era/commit/656e830e4fd60b5ace87dfc1604a102f06ae59e1))
* Base Token Fundamentals ([#2204](https://github.com/matter-labs/zksync-era/issues/2204)) ([39709f5](https://github.com/matter-labs/zksync-era/commit/39709f58071ac77bfd447145e1c3342b7da70560))
* change `zkSync` occurences to `ZKsync` ([#2227](https://github.com/matter-labs/zksync-era/issues/2227)) ([0b4104d](https://github.com/matter-labs/zksync-era/commit/0b4104dbb996ec6333619ea05f3a99e6d4f3b8fa))
* **configs:** Do not panic if config is only partially filled ([#2545](https://github.com/matter-labs/zksync-era/issues/2545)) ([db13fe3](https://github.com/matter-labs/zksync-era/commit/db13fe3550598c69f59cd66b4bb9618ebea041ca))
* **eth-watch:** Integrate decentralized upgrades ([#2401](https://github.com/matter-labs/zksync-era/issues/2401)) ([5a48e10](https://github.com/matter-labs/zksync-era/commit/5a48e1026260024c6ae2b4d1100ee9b798a83e8d))
* L1 batch signing (BFT-474) ([#2414](https://github.com/matter-labs/zksync-era/issues/2414)) ([ab699db](https://github.com/matter-labs/zksync-era/commit/ab699dbe8cffa8bd291d6054579061b47fd4aa0e))
* Minimal External API Fetcher ([#2383](https://github.com/matter-labs/zksync-era/issues/2383)) ([9f255c0](https://github.com/matter-labs/zksync-era/commit/9f255c073cfdab60832fcf9a6d3a4a9258641ef3))
* Poll the main node for the next batch to sign (BFT-496) ([#2544](https://github.com/matter-labs/zksync-era/issues/2544)) ([22cf820](https://github.com/matter-labs/zksync-era/commit/22cf820abbd14b852dffe60f6b564713fe4c8919))
* Revisit base config values ([#2532](https://github.com/matter-labs/zksync-era/issues/2532)) ([3fac8ac](https://github.com/matter-labs/zksync-era/commit/3fac8ac62cc9ac14845f32240af9241386f4034d))
* Support sending logs via OTLP ([#2556](https://github.com/matter-labs/zksync-era/issues/2556)) ([1d206c0](https://github.com/matter-labs/zksync-era/commit/1d206c0af8f28eb00eb1498d6f2cdbb45ffef72a))
* Switch to using crates.io deps ([#2409](https://github.com/matter-labs/zksync-era/issues/2409)) ([27fabaf](https://github.com/matter-labs/zksync-era/commit/27fabafbec66bf4cb65c4fa9e3fab4c3c981d0f2))
* **toolbox:** add format and clippy to zk_toolbox ci ([#2100](https://github.com/matter-labs/zksync-era/issues/2100)) ([49a5c3a](https://github.com/matter-labs/zksync-era/commit/49a5c3abb8b8eb3de0146286f9b3fffe26f545ae))
* **toolbox:** add verify to zk-toolbox ([#2013](https://github.com/matter-labs/zksync-era/issues/2013)) ([23a545c](https://github.com/matter-labs/zksync-era/commit/23a545c51b537af28c084c0f87ce2ebff5a3bbb8))
* **toolbox:** add zk supervisor database commands ([#2051](https://github.com/matter-labs/zksync-era/issues/2051)) ([f99739b](https://github.com/matter-labs/zksync-era/commit/f99739b225286ed8fae648e9a40c5311efe17648))
* **toolbox:** add zk_toolbox ci ([#1985](https://github.com/matter-labs/zksync-era/issues/1985)) ([4ab4922](https://github.com/matter-labs/zksync-era/commit/4ab492201a1654a254c0b14a382a2cb67e3cb9e5))
* **toolbox:** refactor config to its own crate ([#2063](https://github.com/matter-labs/zksync-era/issues/2063)) ([5cfcc24](https://github.com/matter-labs/zksync-era/commit/5cfcc24e92329ba8452d9cec0eb173a54b1dec2f))
* Update to consensus 0.1.0-rc.4 (BFT-486) ([#2475](https://github.com/matter-labs/zksync-era/issues/2475)) ([ff6b10c](https://github.com/matter-labs/zksync-era/commit/ff6b10c4a994cf70297a034202bcb55152748cba))
* **vlog:** New vlog interface + opentelemtry improvements ([#2472](https://github.com/matter-labs/zksync-era/issues/2472)) ([c0815cd](https://github.com/matter-labs/zksync-era/commit/c0815cdaf878afcd9c41dddd9fe56bcf8d910633))
* **zk toolbox:** External node support ([#2287](https://github.com/matter-labs/zksync-era/issues/2287)) ([6384cad](https://github.com/matter-labs/zksync-era/commit/6384cad26aead4d1bdbb606a97d623dacebf912c))
* **zk_toolbox:** Add check for zksync repo path ([#2447](https://github.com/matter-labs/zksync-era/issues/2447)) ([f1cbb74](https://github.com/matter-labs/zksync-era/commit/f1cbb74b863b6e0bcfa74ad780beef29844bac6e))
* **zk_toolbox:** Add contract verifier support for zk toolbox ([#2420](https://github.com/matter-labs/zksync-era/issues/2420)) ([d10a24b](https://github.com/matter-labs/zksync-era/commit/d10a24b3426b0eb13aef9cedfb1c38cbedfb5a7e))
* **zk_toolbox:** Add grafana support ([#2557](https://github.com/matter-labs/zksync-era/issues/2557)) ([f5aaefe](https://github.com/matter-labs/zksync-era/commit/f5aaefe51d3ff4a3365adde6120b874c7c4c68c0))
* **zk_toolbox:** Add prover generate-sk command ([#2222](https://github.com/matter-labs/zksync-era/issues/2222)) ([40e0a95](https://github.com/matter-labs/zksync-era/commit/40e0a956e86583a713d6aacdc61c625931f68e1c))
* **zk_toolbox:** Add prover init command ([#2298](https://github.com/matter-labs/zksync-era/issues/2298)) ([159af3c](https://github.com/matter-labs/zksync-era/commit/159af3c54cc9beb742b2ab43ce3b89b14c8368b7))
* **zk_toolbox:** Add prover run ([#2272](https://github.com/matter-labs/zksync-era/issues/2272)) ([598ef7b](https://github.com/matter-labs/zksync-era/commit/598ef7b73cf141007d2cf031b21fce4744eec44f))
* **zk_toolbox:** add test upgrade subcommand to zk_toolbox ([#2515](https://github.com/matter-labs/zksync-era/issues/2515)) ([1a12f5f](https://github.com/matter-labs/zksync-era/commit/1a12f5f908add42c090170a2f4fb26b731d6971b))
* **zk_toolbox:** Add update command ([#2440](https://github.com/matter-labs/zksync-era/issues/2440)) ([e2fa86f](https://github.com/matter-labs/zksync-era/commit/e2fa86fd216b04c798939f80517d7cca1a45a5a7))
* **zk_toolbox:** Allow toolbox find Zkstack.yaml in parent dirs ([#2430](https://github.com/matter-labs/zksync-era/issues/2430)) ([0957119](https://github.com/matter-labs/zksync-era/commit/095711920bc2193a8b036c9563fa89dfcea433e5))
* **zk_toolbox:** Clean command ([#2387](https://github.com/matter-labs/zksync-era/issues/2387)) ([52a4680](https://github.com/matter-labs/zksync-era/commit/52a4680ed26e755b860e3b97c79618a0c20cb696))
* **zk_toolbox:** Dev command ([#2347](https://github.com/matter-labs/zksync-era/issues/2347)) ([f508ac1](https://github.com/matter-labs/zksync-era/commit/f508ac1f0edba8d267e6b46346a4227149ac7518))
* **zk_toolbox:** Implement default upgrader deployment ([#2526](https://github.com/matter-labs/zksync-era/issues/2526)) ([6d86959](https://github.com/matter-labs/zksync-era/commit/6d8695922689de22e683fe7c318e64f5c9a2144d))
* **zk_toolbox:** resume functionality ([#2376](https://github.com/matter-labs/zksync-era/issues/2376)) ([e5e0473](https://github.com/matter-labs/zksync-era/commit/e5e047393f7cdf1105a0c65f78cd2ec605e1182d))
* **zk_toolbox:** Small adjustment for zk toolbox ([#2424](https://github.com/matter-labs/zksync-era/issues/2424)) ([ce43c42](https://github.com/matter-labs/zksync-era/commit/ce43c422fddccfe88c07ee22a2b8726dd0bd5f61))
* **zk_toolbox:** Update prover support ([#2533](https://github.com/matter-labs/zksync-era/issues/2533)) ([63c92b6](https://github.com/matter-labs/zksync-era/commit/63c92b6205fb156f4b50dee581674b814f44f874))
* **zk_toolbox:** Update reamde for toolbox  ([#2531](https://github.com/matter-labs/zksync-era/issues/2531)) ([d5ba7d8](https://github.com/matter-labs/zksync-era/commit/d5ba7d89fc8b97257b849f75ba6f7a2ad1aeb0d6))
* **zk_toolbox:** use configs from the main repo ([#2470](https://github.com/matter-labs/zksync-era/issues/2470)) ([4222d13](https://github.com/matter-labs/zksync-era/commit/4222d135b62eb4de103c4aebb35e9c302d94ad63))
* **zk_toolbox:** Use docker compose instead of docker-compose ([#2195](https://github.com/matter-labs/zksync-era/issues/2195)) ([2f528ec](https://github.com/matter-labs/zksync-era/commit/2f528ec8d49cb31ef714b409c703ae9f99cc5551))
* **zk_toolbox:** use low level command for running verbose command" ([#2358](https://github.com/matter-labs/zksync-era/issues/2358)) ([29671c8](https://github.com/matter-labs/zksync-era/commit/29671c81684d605ec3350ded1b7dd55d04ba0859))
* **zk-toolbox:** add balance check ([#2016](https://github.com/matter-labs/zksync-era/issues/2016)) ([a8b8e4b](https://github.com/matter-labs/zksync-era/commit/a8b8e4b1b1a3f91b1a52762f2fd30006d323e348))
* **zk-toolbox:** Deploy custom token ([#2329](https://github.com/matter-labs/zksync-era/issues/2329)) ([3a8fed4](https://github.com/matter-labs/zksync-era/commit/3a8fed4c295fa5c0102820fc0103306e31d03815))


### Bug Fixes

* **api:** correct default fee data in eth call ([#2072](https://github.com/matter-labs/zksync-era/issues/2072)) ([e71f6f9](https://github.com/matter-labs/zksync-era/commit/e71f6f96bda08f8330c643a31df4ef9e82c9afc2))
* disable localhost wallets on external network interaction ([#2212](https://github.com/matter-labs/zksync-era/issues/2212)) ([a00317d](https://github.com/matter-labs/zksync-era/commit/a00317dd05af115b396f2f150289e91882e99759))
* **house-keeper:** Fix queue size queries ([#2106](https://github.com/matter-labs/zksync-era/issues/2106)) ([183502a](https://github.com/matter-labs/zksync-era/commit/183502a17eb47a747f50b6a9d38ab78de984f80e))
* **toolbox:** Temporary disable fast mode for deploying l1 contracts … ([#2011](https://github.com/matter-labs/zksync-era/issues/2011)) ([2a1d37b](https://github.com/matter-labs/zksync-era/commit/2a1d37b16b9ccd1f2ce87f61a1b054cdedfd7d1e))
* update rust toolchain version ([#2047](https://github.com/matter-labs/zksync-era/issues/2047)) ([9fe5212](https://github.com/matter-labs/zksync-era/commit/9fe5212ab7b65a63bc53dcf439a212953845ed13))
* **zk_toolbox:** Add chain id for local wallet ([#2041](https://github.com/matter-labs/zksync-era/issues/2041)) ([8e147c1](https://github.com/matter-labs/zksync-era/commit/8e147c11f3ae51e9bdb0cd3e6bfa6919995b3fba))
* **zk_toolbox:** Fix error with balances ([#2034](https://github.com/matter-labs/zksync-era/issues/2034)) ([5d23a3e](https://github.com/matter-labs/zksync-era/commit/5d23a3e44dbe22f4377c6d1042c7b8c03b14c556))
* **zk_toolbox:** Fix installation guide ([#2035](https://github.com/matter-labs/zksync-era/issues/2035)) ([e9038be](https://github.com/matter-labs/zksync-era/commit/e9038bebddb6079ebd76ac01b7ed6068de4bc979))
* **zk_toolbox:** Fix protocol version ([#2118](https://github.com/matter-labs/zksync-era/issues/2118)) ([67f6080](https://github.com/matter-labs/zksync-era/commit/67f60805084de46945a1ae8dfd4aa6b0debc006d))
* **zk_toolbox:** improve readme to include containers command and cd ([#2073](https://github.com/matter-labs/zksync-era/issues/2073)) ([5e5628f](https://github.com/matter-labs/zksync-era/commit/5e5628fc841daaaad229d637202e9342acc2354f))
* **zk_toolbox:** Move l1 rpc to init stage ([#2074](https://github.com/matter-labs/zksync-era/issues/2074)) ([c127ff1](https://github.com/matter-labs/zksync-era/commit/c127ff172cdce8aa0a81887833334d88f1b2ddac))
* **zk_toolbox:** readme added dependencies section and cleaned up ([#2044](https://github.com/matter-labs/zksync-era/issues/2044)) ([78244c7](https://github.com/matter-labs/zksync-era/commit/78244c7e04813b505a9a4285403b092abd827e04))
* **zk_toolbox:** Set proper pubdata sending mode  ([#2507](https://github.com/matter-labs/zksync-era/issues/2507)) ([21fbd77](https://github.com/matter-labs/zksync-era/commit/21fbd77b8c4379b180abcd296a6c74697967acd8))
* **zk_toolbox:** Show balance ([#2254](https://github.com/matter-labs/zksync-era/issues/2254)) ([f1d9f03](https://github.com/matter-labs/zksync-era/commit/f1d9f03ba32081d34a6a24e94b63fb494a33663e))
* **zk_toolbox:** Some small nit ([#2023](https://github.com/matter-labs/zksync-era/issues/2023)) ([4e96e32](https://github.com/matter-labs/zksync-era/commit/4e96e32861337dfa56f4d3daacdc4a7d8610a331))
* **zk_toolbox:** Use both folders for loading contracts  ([#2030](https://github.com/matter-labs/zksync-era/issues/2030)) ([97c6d5c](https://github.com/matter-labs/zksync-era/commit/97c6d5c9c2d9dddf0b18391077c8828e5dc7042b))
* **zk_toolbox:** Use existing ecosystem ([#2534](https://github.com/matter-labs/zksync-era/issues/2534)) ([99fd2bd](https://github.com/matter-labs/zksync-era/commit/99fd2bd6aa2eaa3490c45dd9ac70298aae80d82f))
* **zk_toolbox:** Use slug crate instead of self written function ([#2309](https://github.com/matter-labs/zksync-era/issues/2309)) ([a61f273](https://github.com/matter-labs/zksync-era/commit/a61f273ca0806754cbad12b1cddb247f22459688))
* **zk_toolbox:** Use the same l2 address for shared and erc20 bridge ([#2260](https://github.com/matter-labs/zksync-era/issues/2260)) ([26f2010](https://github.com/matter-labs/zksync-era/commit/26f2010ea2edd1cb79d80852c626051afc473c48))
* **zk_tool:** Change some texts ([#2027](https://github.com/matter-labs/zksync-era/issues/2027)) ([a6232c5](https://github.com/matter-labs/zksync-era/commit/a6232c51c22e0f5229a0e156dd88b3f9573363c3))
* zk-toolbox integration tests ci ([#2226](https://github.com/matter-labs/zksync-era/issues/2226)) ([3f521ac](https://github.com/matter-labs/zksync-era/commit/3f521ace420d3f65e5612c2b6baf096c391ffd7c))


### Reverts

* "feat: Poll the main node for the next batch to sign (BFT-496)" ([#2574](https://github.com/matter-labs/zksync-era/issues/2574)) ([72d3be8](https://github.com/matter-labs/zksync-era/commit/72d3be87efcb059f70b4633cddd707346612c4db))
