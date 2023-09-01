# RELEASE NOTES

## v1.13.7

* increases retransmit-stage deduper capacity and reset-cycle (backport of #30758) (#30857)
* Generalizes deduper to work with any hashable type (backport of #30753) (#30854)
* Adds metrics tracking deduper saturations (backport of #30779) (#30847)
* Panic when shred index exceeds the max per slot (v1.13) (#30623)
* Removes the false_positive_rate field from the Deduper (backport of #30788) (#30839)
* Increases shred-fetch-stage deduper capacity and reset-cycle (backport of #30690) (#30835)
* Dedups packets using an atomic bloom filter (backport of #30726) (#30826)
* ledger-tool: Retain all snapshots by default (#30768)
* Stops nodes from broadcasting slots twice (backport of #30681) (#30684)
* Restrict QUIC to use single self signed client cert (backport #29681) (#29683)

## v1.13.6

* Change SlotMeta is_connected bool to bitflags (backport #29001) (#29020)
* use signed repair request variants (backport #28283) (#28988)
* enforces hash domain for ping-pong protocol (backport #28433) (#28961)
* Fix client get_program_accounts_with_config calls with context (backport #28772) (#28846)
* signed repair request test fixes/cleanup (backport #28691) (#28824)
* bigtable: add timeout to token refresh (backport #28728) (#28807)

## v1.13.5

* Validators on 1.13.4 or 1.13.5 will no longer support the UDP protocol for transaction ingress by default. If you are currently using --tpu-disable-quic, make sure to remove it.
* 1.13.5 contains important QUIC client performance improvements - upgrading RPC nodes is strongly recommended.
* If you are building your own binaries make sure to use the correct Rust version as specified in ci/rust-version.sh (1.59 for v1.13). More recent Rust versions contain backwards-incompatible changes which break the QUIC library that Solana uses. Solana comes with a wrapper script ( ./cargo ) which automatically uses the correct version.
* Commits
* MultiIteratorScanner - improve banking stage performance with high contention
* Delete files older than the lowest_cleanup_slot in LedgerCleanupService::cleanup_ledger (backport #26651) (#28721)
* This commit will significantly improve the ledger's ability to reclaim disk space, and thus, remain closer to expected size in worst case.
* Backport timeout async send tasks to v1.13 (#28750)
* clean_dead_slots_from_accounts_index unrefs correctly (backport #2746… (#28582)
* Limit async tasks spawn to runtime for quic transaction send (backport #28452) (#28603)
* Backport of #28599 to v1.13 (#28602)
* stats for staked/unstaked repair requests (backport #28215) (#28598)
* make ping cache rate limit delay configurable (backport #27955) (#28595)
* ledger-tool: remove inefficient base58 encoding options (backport #28488) (#28494)

## v1.13.4

* This release contains changes to disable the validator's UDP TPU port by default. It can be re-enabled by passing the --tpu-enable-udp flag.

### New v1.13 Commits

* bootstrap: Require known validators to have all snapshot hashes (backport #28284) (#28476)
* Added option to turn on UDP for TPU transaction and make UDP based TPU off by default (backport #27462) (#27658)

## v1.13.3

* Fixed the local-cluster test case test_optimistic_confirmation_violation_detection (backport #27580) (#27649)

## v1.13.2

* Nodes with 0% commission are now included in the getBlock RPC method rewards response field. (backport #28001) (#28010)
* Update CI error message when the version number needs to be bumped (backport #27997) (#28020)
* docs: fix wrong args in 'solana program set-buffer-authority' (backport #27817) (#28045)
* allow unsigned repair requests (backport #27910) (#28026)
* count unsigned repair requests (backport #27953) (#28067)
* Fixed the local-cluster test case test_optimistic_confirmation_violation_detection (backport #27580) (#27649)
* counts gossip packets received before excess packets are dropped (backport #28086) (#28105)
* Allow validators to reset to the slot which matches their last voted slot (backport #28172) (#28186)

## v1.13.1 Pre-release

* Fix local-cluster for QUIC more (backport #27096) (#27103)
* Fix local cluster tests for QUIC usage (backport #27071) (#27077)
* Use batch send in bench-tps to send transactions (backport #27527) (#27560)
* Fix quic staked chunking (backport #27402) (#27408)
* Enable QUIC client by default. Add arg to disable QUIC client. Take 2 (#26927)
* Fix quic client on TestValidator, alternative (backport #27046) (#27053)
* Compute maximum parallel QUIC streams using client stake (#26802)
* Fix transaction chunking on QUIC batch send (#26642)
* Merge pull request from GHSA-x236-qc46-7v8j
* Remove some unchecked math from the quic chunk handling (#26408)
* Fix new 1.62 clippy complaint
* Replace unwraps with expects in quic-client to aid debugging (#26283)
* tpu-client: Refactor to prep for async client (#25432)
