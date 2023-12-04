&nbsp;
## 🪲 Identified Vulnerabilities
Below are the currently identified vulnerabilities that sstan identifies. If you would like to check out a list of patterns that are ready to be implemented and you would like to add them to the repo, you can check out the [Contribution.md](https://github.com/0xKitsune/sstan/blob/main/docs/Contributing.md#potential-optimizations-vulnerability-and-qa-additions)!

| Vulnerability             | Description                                             |
| ------------------------- | ------------------------------------------------------- |
| divide_before_multiply    | Use multiplication symbol before division symbol |
| floating_pragma           | Use locked pragma rather than floating pragma |
| unprotected_selfdestruct  | Add sufficient access control to methods that call `selfdestruct` |
| unsafe_erc20_operation    | Use `safeTransfer()`, `safeTransferFrom()`, `safeApprove()` instead of ERC20 `transfer()`, `transferFrom()`, `approve()`. |
| incorrect_shift_math    | Incorrect order of operations when using `shl` or `shr` in an assembly block |
| double_casting    | Avoid double casting as it may introduce unexpected truncations/rounding errors among other issues. |
| uninitialized_storage_variables    | Uninitialized storage variables |
