# VIctory Forge status

Generated 2026-11-25 06:24 UTC. Base: upstream `main` at `940d32f69b`. Branch `forge/main` at `48a960cb63`: **239 of 849 open pull requests** merged, building (`cargo check --workspace --all-targets`) and passing the asset policy (`cargo xtask policy`). Rules: [RULES.md](RULES.md).

| Result | Pull requests |
|---|---:|
| in forge | 239 |
| conflicts with pull requests | 281 |
| conflicts with main | 280 |
| breaks the build | 31 |
| changes CI workflows | 5 |
| draft | 13 |

## Tests on the branch

`cargo test --workspace --no-fail-fast`: **1111 passed, 19 failed, 22 ignored** (on the machine that ran the Forge; GPU tests skip where it has no GPU). Test failures do not keep a pull request out; they are listed for the authors.

- `bounded_execution_and_compiler_refusal`
- `draw::tests::a_texture_written_in_one_format_reads_in_another_through_a_placed_view`
- `emit::bvh_tests::intersections_on_nodes_in_title_memory_equal_the_reference`
- `emit::bvh_tests::intersections_on_the_gpu_equal_the_reference`
- `emit::bvh_tests::intersections_with_64_bit_pointers_on_the_gpu_equal_the_reference`
- `emit::bvh_tests::intersections_with_a16_on_the_gpu_equal_the_reference`
- `emit::bvh_tests::unsorted_box_intersections_on_the_gpu_equal_the_reference`
- `emit::shader::tests::compute_explicit_derivative_validates`
- `emit::tests::interpolated_f16_writes_the_op_sel_half_with_modifiers`
- `emit::valu_tests::image_addresses_of_16_bits_and_clamps`
- `emit::valu_tests::s_bfm_b64_builds_a_mask_and_keeps_scc`
- `emit::valu_tests::signed_64_bit_operands_sign_extend_their_literal`
- `emit::waterfall_tests::lanes_sample_the_entry_their_index_names_when_scaled_in_the_loop`
- `emit::waterfall_tests::lanes_sample_the_entry_their_offset_names_in_the_console_waterfall`
- `emit::waterfall_tests::lanes_sample_the_texture_their_arm_loaded_or_picked`
- `emit::waterfall_tests::lanes_sample_the_texture_their_if_else_arm_chose`
- `emit::waterfall_tests::lanes_sample_their_own_texture_in_a_waterfall_narrowed_by_v_cmpx`
- `frame::tests::waits_end_on_ticks_and_count_frames`
- `the_conformance_title_runs_through_victory`

## Since the last run

Joined the branch: #24, #130, #139, #158, #166, #224, #259, #385, #421, #450, #457, #470, #486, #491, #506, #517, #522, #541, #545, #601, #673, #703, #927, #958, #983, #1009, #1017, #1033, #1046, #1065, #1112, #1113, #1152, #1155, #1174, #1194, #1203, #1209, #1212, #1228, #1233, #1259, #1262, #1263, #1264, #1283, #1315, #1324, #1383, #1387, #1395, #1418, #1419, #1424, #1435, #1436, #1449, #1457, #1492, #1505, #1519, #1542, #1547, #1555, #1582, #1593, #1628, #1646, #1659, #1692, #1700, #1701, #1716, #1722, #1728, #1741, #1751, #1759, #1762, #1765, #1766, #1768, #1773, #1775, #1778, #1790, #1792, #1794, #1798, #1802, #1805, #1806, #1809, #1813, #1814, #1822, #1823, #1827, #1833, #1834, #1839, #1849, #1857, #1863, #1865, #1866, #1873, #1884, #1889, #1892, #1893, #1894, #1895, #1896, #1903, #1907, #1912, #1913, #1914, #1916, #1927, #1928, #1930, #1933, #1943, #1944, #1946, #1947, #1966, #1969, #1975, #1979, #1980, #1981, #1984, #1986, #1988, #1992, #1994, #1995, #2001, #2005, #2007, #2008, #2009, #2015, #2018, #2019, #2021, #2026, #2031, #2033, #2036, #2041, #2043, #2044, #2052, #2053, #2057, #2061, #2065, #2066, #2071, #2073, #2074, #2075, #2079, #2082, #2090, #2098, #2099, #2103, #2109, #2110, #2114, #2134, #2139, #2141.

Left the branch (closed, merged upstream, or now conflicting or failing): #12, #52, #96, #98, #114, #115, #122, #126, #133, #153, #156, #162, #169, #172, #174, #178, #183, #186, #194, #197, #202, #207, #211, #213, #214, #217, #218, #219, #221, #238, #246, #247, #263, #270, #286, #288, #290, #299, #311, #322, #323, #335, #337, #345, #351, #353, #357, #359, #361, #363, #368, #370, #373, #388, #412, #447, #449, #451, #455, #460, #474, #475, #476, #480, #488, #494, #504, #528, #529, #531, #532, #536, #538, #543, #547, #549, #553, #560, #570, #603, #615, #617, #618, #624, #631, #649, #658, #660, #661, #664, #669, #670, #674, #697, #922, #926, #928, #937, #941, #957, #963, #974, #990, #993, #1000, #1016, #1021, #1048, #1050, #1052, #1053, #1080, #1082, #1098, #1106, #1109, #1125, #1146, #1158, #1159, #1161, #1167, #1169, #1173, #1177, #1180, #1188, #1193, #1204, #1206, #1220, #1221, #1224, #1251, #1265, #1266, #1282, #1299, #1345, #1358, #1359, #1366, #1367, #1371, #1380, #1389, #1390, #1391, #1394, #1431, #1438, #1443, #1446, #1450, #1453, #1455, #1473, #1474, #1475, #1476, #1477, #1487, #1506, #1510, #1515, #1539, #1549, #1562, #1585, #1592, #1594, #1601, #1611, #1613, #1621, #1635, #1636, #1647, #1660, #1664, #1676, #1677, #1695, #1707, #1724, #1726, #1737.

## in forge (239)

| PR | Title | Author | Detail |
|---|---|---|---|
| [#24](https://github.com/Marvisak/VIctory/pull/24) | GPU: run indirect mesh dispatches (DispatchMeshX) through indirect execution | artor410 |  |
| [#67](https://github.com/Marvisak/VIctory/pull/67) | GPU: implement CopyTextureRegion texture-to-buffer readbacks | alessiodeia |  |
| [#105](https://github.com/Marvisak/VIctory/pull/105) | Shaders: a loop that branches back to the entry instruction translates | tedin7 |  |
| [#130](https://github.com/Marvisak/VIctory/pull/130) | Profile: opt-in VICTORY_PROFILE (frame times, memory, caches, loads; frames.csv, a Perfetto trace and an on-screen overlay) | Sunkiller-qc |  |
| [#139](https://github.com/Marvisak/VIctory/pull/139) | Shaders: reuse SSA sums when tracing memory addresses | TG0000 |  |
| [#151](https://github.com/Marvisak/VIctory/pull/151) | Link victory.exe with a 16 MiB default thread stack | ItsYusei99 |  |
| [#158](https://github.com/Marvisak/VIctory/pull/158) | GPU: the depth bounds test reaches the host | r4tedx |  |
| [#166](https://github.com/Marvisak/VIctory/pull/166) | Shaders: lower s_bitset{0,1}_b64, s_brev_b64 and s_bitreplicate_b64_b32 | n-romanovich |  |
| [#188](https://github.com/Marvisak/VIctory/pull/188) | Loader: find TLS-array load candidates sixteen offsets at a time (8-9x faster scan) | TG0000 |  |
| [#224](https://github.com/Marvisak/VIctory/pull/224) | HLE: report the host's commit headroom before the title starts | WallasSCRIPTS |  |
| [#228](https://github.com/Marvisak/VIctory/pull/228) | XGameSave: a failed create leaves a null handle, as documented | Dufftwt |  |
| [#240](https://github.com/Marvisak/VIctory/pull/240) | DSP: convolve reverb streams on the host | Quat3rnion |  |
| [#259](https://github.com/Marvisak/VIctory/pull/259) | Shaders: query a descriptor's level count only when word 3 is read | desplmfao |  |
| [#280](https://github.com/Marvisak/VIctory/pull/280) | README: a hard drive keeps the first screen black for ~90 s | curnpoo |  |
| [#284](https://github.com/Marvisak/VIctory/pull/284) | Shaders: RDNA2's legacy multiply-adds are the fused ones | Dufftwt |  |
| [#319](https://github.com/Marvisak/VIctory/pull/319) | GPU: end a late flip wait on the tick it missed, not the next | xmvo |  |
| [#324](https://github.com/Marvisak/VIctory/pull/324) | fix(pe): detect incomplete TLS callback arrays | pedrofort |  |
| [#343](https://github.com/Marvisak/VIctory/pull/343) | Shaders: reject overlapping DXBC container parts | pedrofort |  |
| [#364](https://github.com/Marvisak/VIctory/pull/364) | XMA: accelerate VLC decoding with an eight-bit prefix lookup | pedrofort |  |
| [#369](https://github.com/Marvisak/VIctory/pull/369) | Shaders: eliminate heap allocations during CFG traversal | pedrofort |  |
| [#378](https://github.com/Marvisak/VIctory/pull/378) | CLI: include runtime shader hashes in the translation listing | Mesrine67 |  |
| [#379](https://github.com/Marvisak/VIctory/pull/379) | Shaders: lower v_div_fixup_f16 | LouisRiverstone |  |
| [#385](https://github.com/Marvisak/VIctory/pull/385) | Shaders: track every incoming edge of a lane-mask phi | nathanbzrr |  |
| [#398](https://github.com/Marvisak/VIctory/pull/398) | HLE: report the single supported offline user | ToxicmaggotRP |  |
| [#415](https://github.com/Marvisak/VIctory/pull/415) | GPU: use native root type codes in the shader cache | enzoduarte1 |  |
| [#421](https://github.com/Marvisak/VIctory/pull/421) | HLE: clean dot parts in extended virtual-drive paths | enzoduarte1 |  |
| [#430](https://github.com/Marvisak/VIctory/pull/430) | DirectStorage: Close completes what was submitted | marquezinii |  |
| [#446](https://github.com/Marvisak/VIctory/pull/446) | HLE: report required size on string getter buffer errors | enzoduarte1 |  |
| [#450](https://github.com/Marvisak/VIctory/pull/450) | GPU: preserve typed null render target slots | fuzeofficlal |  |
| [#454](https://github.com/Marvisak/VIctory/pull/454) | Loader: opt-in VICTORY_NO_WATCHDOG hook for the title's watchdog (build 1.0.2.1) | ruvenesh |  |
| [#457](https://github.com/Marvisak/VIctory/pull/457) | fix(shader): accept overwritten VGPR inputs in counted indexed writes | jeremygold02 |  |
| [#470](https://github.com/Marvisak/VIctory/pull/470) | shader: lower wave prefixes of min and max | Loggerten |  |
| [#477](https://github.com/Marvisak/VIctory/pull/477) | Shaders: fix DXIL validator UB on null error blobs, unchecked GetStatus, and COM/DLL handle leaks | nathanbzrr |  |
| [#478](https://github.com/Marvisak/VIctory/pull/478) | Audio capture: guard WASAPI timestamp underflow and stream silence writes | nathanbzrr |  |
| [#486](https://github.com/Marvisak/VIctory/pull/486) | shader: opt the test binaries into an app-local Agility SDK runtime | naixue2233 |  |
| [#491](https://github.com/Marvisak/VIctory/pull/491) | GPU: remember pipelines whose creation crashed the process (VICTORY_PIPELINE_JOURNAL) | desplmfao |  |
| [#505](https://github.com/Marvisak/VIctory/pull/505) | Tests: add synthetic Xbox runtime conformance under Wine | pedrofort |  |
| [#506](https://github.com/Marvisak/VIctory/pull/506) | CLI: reject zero shader translation workers | yualk |  |
| [#512](https://github.com/Marvisak/VIctory/pull/512) | HLE: resolve ordinals on pseudo-module handles | Quat3rnion |  |
| [#517](https://github.com/Marvisak/VIctory/pull/517) | CLI: report incomplete shader directory scans | Dathinel |  |
| [#522](https://github.com/Marvisak/VIctory/pull/522) | HLE: validate null package identifier in XPackage methods | pushkarreddyy |  |
| [#535](https://github.com/Marvisak/VIctory/pull/535) | App: enforce asset policy check in default_paths | pushkarreddyy |  |
| [#541](https://github.com/Marvisak/VIctory/pull/541) | GPU: retain D3D12 object names in private data | broccolize |  |
| [#545](https://github.com/Marvisak/VIctory/pull/545) | xtask: add qa command to run policy, rustfmt, and clippy checks | pushkarreddyy |  |
| [#580](https://github.com/Marvisak/VIctory/pull/580) | Docs: add a contributor onboarding guide | lucaseddd |  |
| [#589](https://github.com/Marvisak/VIctory/pull/589) | fix(shader): skip unused image atomic descriptor reconstruction | jeremygold02 |  |
| [#590](https://github.com/Marvisak/VIctory/pull/590) | HLE: reject duplicate blob changes in one save update | Quat3rnion |  |
| [#601](https://github.com/Marvisak/VIctory/pull/601) | GPU: bound texture upload and capture slices to the allocated buffer | AwsAli05 |  |
| [#602](https://github.com/Marvisak/VIctory/pull/602) | GPU: report retained snapshot, texture scratch and upload memory | lucaseddd |  |
| [#613](https://github.com/Marvisak/VIctory/pull/613) | HLE: serialize device callbacks across delivery threads | Quat3rnion |  |
| [#673](https://github.com/Marvisak/VIctory/pull/673) | Make translate failure examples stable across workers | timorjay |  |
| [#677](https://github.com/Marvisak/VIctory/pull/677) | Build: thin LTO for release and diag builds | wbkepler |  |
| [#684](https://github.com/Marvisak/VIctory/pull/684) | Shaders: find the PC test compiler independently of the validator | Dathinel |  |
| [#686](https://github.com/Marvisak/VIctory/pull/686) | docs: check archive integrity after an early boot breakpoint | TomEageer |  |
| [#703](https://github.com/Marvisak/VIctory/pull/703) | Formats: eliminate BCPack per-call allocations and optimize streaming decode | wbkepler |  |
| [#708](https://github.com/Marvisak/VIctory/pull/708) | fix(config): keep the data directory out of the cache root | Loggerten |  |
| [#927](https://github.com/Marvisak/VIctory/pull/927) | GameInput: held keys are reported in the order they were pressed, as documented | BastienRodz |  |
| [#933](https://github.com/Marvisak/VIctory/pull/933) | GPU: report allocated mips from GetDesc | enzoduarte1 |  |
| [#939](https://github.com/Marvisak/VIctory/pull/939) | HLE: XSystemGetXboxLiveSandboxId refuses a buffer under 16 bytes | LouisRiverstone |  |
| [#958](https://github.com/Marvisak/VIctory/pull/958) | GPU: unit tests for the OPTIONS, ARCHITECTURE and MULTISAMPLE feature queries | LouisRiverstone |  |
| [#983](https://github.com/Marvisak/VIctory/pull/983) | GPU: a full early pipeline list drops the oldest runs' keys | LICObr |  |
| [#997](https://github.com/Marvisak/VIctory/pull/997) | HLE: prevent automatic WinHTTP redirects | Hiiva |  |
| [#1009](https://github.com/Marvisak/VIctory/pull/1009) | xmem: keep the space below 4 GiB for the title's 32-bit memory | NuraadGit |  |
| [#1017](https://github.com/Marvisak/VIctory/pull/1017) | Fix the Linux build and clippy gates | Jitheswar |  |
| [#1033](https://github.com/Marvisak/VIctory/pull/1033) | victory-shader: build the per-instruction read/write lists once (ir::build -13%) | pes7 |  |
| [#1046](https://github.com/Marvisak/VIctory/pull/1046) | Shaders: hardware test for the s_cmpk_* scalar comparisons | LouisRiverstone |  |
| [#1065](https://github.com/Marvisak/VIctory/pull/1065) | Shaders: use validator 1.8 for compute derivatives | Halfect |  |
| [#1110](https://github.com/Marvisak/VIctory/pull/1110) | fix(hle): account for save replacements before clamping quota | divyeshganta |  |
| [#1111](https://github.com/Marvisak/VIctory/pull/1111) | fix(loader): preserve protection regions when binding unaligned imports | divyeshganta |  |
| [#1112](https://github.com/Marvisak/VIctory/pull/1112) | Shaders: round normalized MTBUF loads without Int64ShaderOps | Razdva122 |  |
| [#1113](https://github.com/Marvisak/VIctory/pull/1113) | Formats: lay out packed RGB textures as horizontal texel pairs | Razdva122 |  |
| [#1123](https://github.com/Marvisak/VIctory/pull/1123) | Validate XGameSave blob count before reading | EatCodeSleep2 |  |
| [#1152](https://github.com/Marvisak/VIctory/pull/1152) | GPU: no blending on integer render targets in host pipelines (fixes #806) | pablogonz12 |  |
| [#1155](https://github.com/Marvisak/VIctory/pull/1155) | hle(display): fall back to no-op deferral when PowerSetRequest returns ERROR_NOT_SUPPORTED | geiyiliyabumo |  |
| [#1174](https://github.com/Marvisak/VIctory/pull/1174) | HLE: implement IUnknown::QueryInterface and unit tests for xgame objects | geiyiliyabumo |  |
| [#1194](https://github.com/Marvisak/VIctory/pull/1194) | HLE: extract installed_package and add focused xstore license/package tests | geiyiliyabumo |  |
| [#1195](https://github.com/Marvisak/VIctory/pull/1195) | Shaders: add SQ_IMG_SAMP LOD-bias and no-mip range tests for emit/sampler.rs | geiyiliyabumo |  |
| [#1199](https://github.com/Marvisak/VIctory/pull/1199) | HLE: return NOT_SUPPORTED early for null out in QueryApiImpl without marking area missing | geiyiliyabumo |  |
| [#1203](https://github.com/Marvisak/VIctory/pull/1203) | GPU: expand synthetic unit tests for pipeline/make.rs (1 -> 9/12 tests) | geiyiliyabumo |  |
| [#1205](https://github.com/Marvisak/VIctory/pull/1205) | Formats: add 7 invariant unit tests for xma/tables.rs | geiyiliyabumo |  |
| [#1209](https://github.com/Marvisak/VIctory/pull/1209) | Shaders: fix GFX10 independent a16/g16 gradient dword packing in fields.rs with 21 tests | geiyiliyabumo |  |
| [#1212](https://github.com/Marvisak/VIctory/pull/1212) | HLE: expand synthetic unit tests for apu.rs (1 -> 19 tests) | geiyiliyabumo |  |
| [#1218](https://github.com/Marvisak/VIctory/pull/1218) | xtask: wine-test runs the Windows tests on a Linux host under Wine | neonbyte1 |  |
| [#1228](https://github.com/Marvisak/VIctory/pull/1228) | GPU: order shader writes on command list pipeline syncs (#732) | Ctrl-Alt-Haze |  |
| [#1233](https://github.com/Marvisak/VIctory/pull/1233) | Report bound descriptor tables accurately in DRED breadcrumbs | Jitheswar |  |
| [#1253](https://github.com/Marvisak/VIctory/pull/1253) | Shaders: follow scalar index arithmetic in waterfall table offsets | mgunes1311-lab |  |
| [#1259](https://github.com/Marvisak/VIctory/pull/1259) | Shaders: v_med3 orders -0 below +0, as RDNA2 does | pablogonz12 |  |
| [#1262](https://github.com/Marvisak/VIctory/pull/1262) | Shaders: double-precision add, mul, fma, min, max, compares and conversions (stacked on #416) | MamaLeVrai |  |
| [#1263](https://github.com/Marvisak/VIctory/pull/1263) | Shader: the loop limit switch works in every build | pablogonz12 |  |
| [#1264](https://github.com/Marvisak/VIctory/pull/1264) | HLE: map a page scatter in runs, not one page at a time | pablogonz12 |  |
| [#1278](https://github.com/Marvisak/VIctory/pull/1278) | Docs: add testing.md | DrummingBird1 |  |
| [#1283](https://github.com/Marvisak/VIctory/pull/1283) | Clarify DRED reports when GPU progress is unknown | DoubleD17 |  |
| [#1303](https://github.com/Marvisak/VIctory/pull/1303) | Shaders: bounds-check DXIL header offsets and recognize dword zero padding | geiyiliyabumo |  |
| [#1304](https://github.com/Marvisak/VIctory/pull/1304) | Loader: fix thread shutdown targeting, registry lifetime, and CRT detach ordering | geiyiliyabumo |  |
| [#1306](https://github.com/Marvisak/VIctory/pull/1306) | Loader: fix thunk block overflow, capacity enforcement, flush errors, and leaks | geiyiliyabumo |  |
| [#1315](https://github.com/Marvisak/VIctory/pull/1315) | Shaders, GPU: translate single-stream triangle geometry programs | r4tedx |  |
| [#1324](https://github.com/Marvisak/VIctory/pull/1324) | Pipeline: accept existing entry when concurrent cache publication races on Windows | mgunes1311-lab |  |
| [#1329](https://github.com/Marvisak/VIctory/pull/1329) | HLE GameInput supports IUnknown queries | BaggyALPHA |  |
| [#1383](https://github.com/Marvisak/VIctory/pull/1383) | Preserve Win32 errors from DirectStorage OpenFile | EatCodeSleep2 |  |
| [#1387](https://github.com/Marvisak/VIctory/pull/1387) | Round v_fma_f16 once in DXIL lowering | DoubleD17 |  |
| [#1395](https://github.com/Marvisak/VIctory/pull/1395) | Shaders: lower s_bfe_u64, s_bfe_i64 and s_bfm_b64 | AnoHunted |  |
| [#1418](https://github.com/Marvisak/VIctory/pull/1418) | [BUNDLE] GPU fixes (write watch, textures, device, DXGI, objects, primitives, capture): 9 PRs, #553 #153 #270 #1158 #1204 #1220 #488 #1146 #631 (rebuilt on main 70564ae) | MamaLeVrai |  |
| [#1419](https://github.com/Marvisak/VIctory/pull/1419) | HLE: pass non-code LoadLibraryEx mappings outside HostPolicy to the host | fuzeofficlal |  |
| [#1424](https://github.com/Marvisak/VIctory/pull/1424) | GPU skips empty host submissions | BaggyALPHA |  |
| [#1435](https://github.com/Marvisak/VIctory/pull/1435) | HLE: implement XTaskQueueRegisterWaiter and XTaskQueueUnregisterWaiter | fuzeofficlal |  |
| [#1436](https://github.com/Marvisak/VIctory/pull/1436) | Game: check record bounds and offset arithmetic in SGD5 parsing | fuzeofficlal |  |
| [#1437](https://github.com/Marvisak/VIctory/pull/1437) | Shaders: preserve the unwritten high half of packed D16 image loads | EatCodeSleep2 |  |
| [#1441](https://github.com/Marvisak/VIctory/pull/1441) | Shaders: preserve the unwritten high half of packed D16 buffer loads | EatCodeSleep2 |  |
| [#1445](https://github.com/Marvisak/VIctory/pull/1445) | HLE reuses freed physical page numbers | BaggyALPHA |  |
| [#1449](https://github.com/Marvisak/VIctory/pull/1449) | GPU: report host resources held by descriptor heap slots in diagnostics | fuzeofficlal |  |
| [#1457](https://github.com/Marvisak/VIctory/pull/1457) | DirectStorage: keep raw reads running while staging drains | Luizbc2 |  |
| [#1492](https://github.com/Marvisak/VIctory/pull/1492) | [BUNDLE] Shader instruction lowering: 10 PRs, #156 #162 #169 #368 #379 #990 #1021 #1048 #1052 #1053 (rebuilt on main 70564ae) | MamaLeVrai |  |
| [#1505](https://github.com/Marvisak/VIctory/pull/1505) | GPU: pipeline workers at the title's priority, not below it | KevinPratap |  |
| [#1519](https://github.com/Marvisak/VIctory/pull/1519) | Shaders: unpack 16-bit vertex inputs into the low half | Bloh09 |  |
| [#1537](https://github.com/Marvisak/VIctory/pull/1537) | HLE: update shared XMA context words atomically | Texbirds |  |
| [#1542](https://github.com/Marvisak/VIctory/pull/1542) | [BUNDLE] GPU fixes 4: 3 PRs, #1082 #1173 #1265 (rebuilt on main 70564ae) | MamaLeVrai |  |
| [#1547](https://github.com/Marvisak/VIctory/pull/1547) | GPU: invalidate HOST_WRITTEN textures when title memory is written | n-romanovich |  |
| [#1555](https://github.com/Marvisak/VIctory/pull/1555) | GPU: coalesce nearby shadow-buffer refreshes without spanning huge heap gaps | Texbirds |  |
| [#1557](https://github.com/Marvisak/VIctory/pull/1557) | Validate BC7 control-word subset counts | oddmaniv |  |
| [#1582](https://github.com/Marvisak/VIctory/pull/1582) | Shaders: enforce inactive-lane EXEC rules for DPP16, DPP8 and ds_swizzle_b32 | Capt-Speed |  |
| [#1593](https://github.com/Marvisak/VIctory/pull/1593) | [BUNDLE] Shader translation speed: 4 PRs, #288 #369 #1033 #1180 (rebuilt on main 70564ae) | MamaLeVrai |  |
| [#1628](https://github.com/Marvisak/VIctory/pull/1628) | GPU: log initial states and clear format when texture creation fails | AwsAli05 |  |
| [#1646](https://github.com/Marvisak/VIctory/pull/1646) | Shaders: fold constant bin/cmp/select/cast ops and identity ops at emit time | adamdotpy |  |
| [#1657](https://github.com/Marvisak/VIctory/pull/1657) | XSystem: name the analytics family Windows.Xbox | Calvix2k |  |
| [#1659](https://github.com/Marvisak/VIctory/pull/1659) | shader: lower v_frexp_{exp,mant}_{f32,f16} in pure 32-bit DXIL | geiyiliyabumo |  |
| [#1681](https://github.com/Marvisak/VIctory/pull/1681) | GPU: verify exact dirty-page masks on D3D12 | avt0m4t48 |  |
| [#1692](https://github.com/Marvisak/VIctory/pull/1692) | [BUNDLE] GPU diagnostics: 3 PRs, #669 #1251 #602 (rebuilt on main 70564ae) | MamaLeVrai |  |
| [#1700](https://github.com/Marvisak/VIctory/pull/1700) | GPU: say when a draw's render target has no host copy or lost its permission | fiestorra |  |
| [#1701](https://github.com/Marvisak/VIctory/pull/1701) | GPU: VICTORY_MODE overrides the reported console display mode | Calvix2k |  |
| [#1703](https://github.com/Marvisak/VIctory/pull/1703) | README: the GPU timing switch needs DRED for the per-command notes | fiestorra |  |
| [#1712](https://github.com/Marvisak/VIctory/pull/1712) | victory-shader: support line-topology mesh shaders | tjthebigbearcat-hub |  |
| [#1716](https://github.com/Marvisak/VIctory/pull/1716) | victory-shader: fold constants and eliminate dead pure ops in bitcode | bartikele |  |
| [#1719](https://github.com/Marvisak/VIctory/pull/1719) | DirectStorage: clear the factory out pointer on failure | davutsimsekk |  |
| [#1722](https://github.com/Marvisak/VIctory/pull/1722) | GPU: the host pipeline library works under Wine (null blob for an empty library) | Impre-visible |  |
| [#1728](https://github.com/Marvisak/VIctory/pull/1728) | Pipelines: VICTORY_PIPELINE_WORKERS sets the worker count | npanu420 |  |
| [#1741](https://github.com/Marvisak/VIctory/pull/1741) | Docs: AGENTS.md for AI agents working on VIctory | ThiagoAlexsander |  |
| [#1743](https://github.com/Marvisak/VIctory/pull/1743) | GPU tests: replay a world frame's copies of title memory (benchmark for the copy PRs) | marquezinii |  |
| [#1751](https://github.com/Marvisak/VIctory/pull/1751) | GPU: capped view heaps keep the full dynamic range (NVIDIA) | JurajSimicevic |  |
| [#1759](https://github.com/Marvisak/VIctory/pull/1759) | Shaders: lower numerical Wave32 mesh masks at uniform entry | aycow |  |
| [#1762](https://github.com/Marvisak/VIctory/pull/1762) | GPU: structured buffer views are raw views of the same bytes on the host (NVIDIA reads 0 through raw loads of structured views) | claude-GTA6 |  |
| [#1765](https://github.com/Marvisak/VIctory/pull/1765) | GPU tests: the castable pair is two host textures where the host refuses castable formats | AgneyaK |  |
| [#1766](https://github.com/Marvisak/VIctory/pull/1766) | Shaders: cube-map face coordinates read denormals as zeros | alexdeliu |  |
| [#1768](https://github.com/Marvisak/VIctory/pull/1768) | GPU: retry a root signature without the heap-directly-indexed flags when the host refuses them | pablogonz12 |  |
| [#1773](https://github.com/Marvisak/VIctory/pull/1773) | raise the fault of an emulated clzero at the title's instruction | imjustprism |  |
| [#1775](https://github.com/Marvisak/VIctory/pull/1775) | GPU: unordered views only in formats the host can store (#1753) | JurajSimicevic |  |
| [#1778](https://github.com/Marvisak/VIctory/pull/1778) | GPU: bind a sampler heap of our own when the title binds none | fiestorra |  |
| [#1790](https://github.com/Marvisak/VIctory/pull/1790) | Shader tests: dot products take their operands at run time | Ingenioeren |  |
| [#1792](https://github.com/Marvisak/VIctory/pull/1792) | Formats: reject texture mip counts beyond the full chain | VlREX |  |
| [#1794](https://github.com/Marvisak/VIctory/pull/1794) | Shaders: support BVH a16 ray directions and unsorted box nodes | ESAT586 |  |
| [#1798](https://github.com/Marvisak/VIctory/pull/1798) | run: boot a separated exe/dump layout from the dump | Calvix2k |  |
| [#1802](https://github.com/Marvisak/VIctory/pull/1802) | Perf/shader relative shift bounds | VlREX |  |
| [#1805](https://github.com/Marvisak/VIctory/pull/1805) | CLI: write pipeline key listings in stable pack order | EatCodeSleep2 |  |
| [#1806](https://github.com/Marvisak/VIctory/pull/1806) | CLI: discover pipeline packs in archives without shader containers | EatCodeSleep2 |  |
| [#1809](https://github.com/Marvisak/VIctory/pull/1809) | CLI: list pipeline keys without initializing caches | EatCodeSleep2 |  |
| [#1813](https://github.com/Marvisak/VIctory/pull/1813) | GPU tests: verify indirect::bound clamping and budget on the GPU | n-romanovich |  |
| [#1814](https://github.com/Marvisak/VIctory/pull/1814) | PE tests: cover ImportLibrary::parse short imports and archive errors | n-romanovich |  |
| [#1822](https://github.com/Marvisak/VIctory/pull/1822) | Speech: the title's narration plays through the host's SAPI voice | kais7654 |  |
| [#1823](https://github.com/Marvisak/VIctory/pull/1823) | Shaders: bound a signature part's rows by the bytes it has | slatefield |  |
| [#1827](https://github.com/Marvisak/VIctory/pull/1827) | HLE: enumerate the current installation for XPackageKind::Game | n-romanovich |  |
| [#1833](https://github.com/Marvisak/VIctory/pull/1833) | Shaders: walk a live set's registers a bit at a time | slatefield |  |
| [#1834](https://github.com/Marvisak/VIctory/pull/1834) | Shaders: read a VALU opcode's name without a heap vector | slatefield |  |
| [#1839](https://github.com/Marvisak/VIctory/pull/1839) | GPU: an unimplemented method logs the four argument registers | pablogonz12 |  |
| [#1849](https://github.com/Marvisak/VIctory/pull/1849) | GPU: free memory is not the runtime's to watch (fixes a world-load deadlock under Wine) | crashbashash |  |
| [#1857](https://github.com/Marvisak/VIctory/pull/1857) | Shader tests: 64-bit image atomics skip devices without typed 64-bit heap atomics | Ingenioeren |  |
| [#1863](https://github.com/Marvisak/VIctory/pull/1863) | fix(census): report unreadable files during PE detection | Damokles2 |  |
| [#1865](https://github.com/Marvisak/VIctory/pull/1865) | GPU: pool title-memory dirty bitmaps, write page tables directly, and skip zero bitmap words | nathanbzrr |  |
| [#1866](https://github.com/Marvisak/VIctory/pull/1866) | Game: preserve sample masks in prebuilt graphics pipelines | necrobytes |  |
| [#1873](https://github.com/Marvisak/VIctory/pull/1873) | GPU: share a scratch buffer for texture detiling instead of retaining one per Staging | 0xKaji |  |
| [#1884](https://github.com/Marvisak/VIctory/pull/1884) | Perf/memory guard batching | VlREX |  |
| [#1889](https://github.com/Marvisak/VIctory/pull/1889) | Shader tests: VICTORY_ADAPTER picks the GPU the hardware checks run on | alexdeliu |  |
| [#1892](https://github.com/Marvisak/VIctory/pull/1892) | Tests: every GPU test module takes the shared lock, which diff_tests did not | fiestorra |  |
| [#1893](https://github.com/Marvisak/VIctory/pull/1893) | GPU: implement IDXGIAdapter2::GetDesc2 | DoubleD17 |  |
| [#1894](https://github.com/Marvisak/VIctory/pull/1894) | GPU: pool mapped upload buffers across command list resets | n-romanovich |  |
| [#1895](https://github.com/Marvisak/VIctory/pull/1895) | GPU: preserve integer contrast in R32 captures | DoubleD17 |  |
| [#1896](https://github.com/Marvisak/VIctory/pull/1896) | Shader: the packed dot over an inline constant is taken by hand from … | kais7654 |  |
| [#1903](https://github.com/Marvisak/VIctory/pull/1903) | Shader: the ray test's triangle multiplies stay apart from their adds | kais7654 |  |
| [#1907](https://github.com/Marvisak/VIctory/pull/1907) | victory-shader: honor unwritten OSG1 channels and output modes in rectangle_gs | mgunes1311-lab |  |
| [#1912](https://github.com/Marvisak/VIctory/pull/1912) | test(shader): initialize Root UAV buffers before dispatch | Damokles2 |  |
| [#1913](https://github.com/Marvisak/VIctory/pull/1913) | GPU tests: a batch's hold on a copy ends before its pool is freed | Mericas77 |  |
| [#1914](https://github.com/Marvisak/VIctory/pull/1914) | GPU: an empty pipeline library is not asked for pipelines it cannot have | fiestorra |  |
| [#1916](https://github.com/Marvisak/VIctory/pull/1916) | GPU: clear descriptors for null constant buffers (re-lands #1522) | Calvix2k |  |
| [#1927](https://github.com/Marvisak/VIctory/pull/1927) | Shaders: d16 format loads saturate 32-bit integer channels | upipibi |  |
| [#1928](https://github.com/Marvisak/VIctory/pull/1928) | Storage: do not follow directory symlinks or junctions when computing used bytes | ashrafsdrop |  |
| [#1930](https://github.com/Marvisak/VIctory/pull/1930) | XPackage: wire XPackageEnumerateChunkAvailability (slot 13 / 0x68) | ashrafsdrop |  |
| [#1933](https://github.com/Marvisak/VIctory/pull/1933) | victory-gpu: keep depth-twin UAVs in concrete UAV formats and support default views | mgunes1311-lab |  |
| [#1943](https://github.com/Marvisak/VIctory/pull/1943) | Shaders: the null-buffer reads of the heap skip a host that caps its heaps | kais7654 |  |
| [#1944](https://github.com/Marvisak/VIctory/pull/1944) | reject a dxbc part inside the header or part table | imjustprism |  |
| [#1946](https://github.com/Marvisak/VIctory/pull/1946) | Descriptors: null UAV for packed 16-bit formats D3D12 refuses | Biewlhenrique |  |
| [#1947](https://github.com/Marvisak/VIctory/pull/1947) | HLE: a live range takes a guard page, and a mapped range is not a placeholder | fiestorra |  |
| [#1966](https://github.com/Marvisak/VIctory/pull/1966) | keep crash reports when stderr is closed | imjustprism |  |
| [#1969](https://github.com/Marvisak/VIctory/pull/1969) | GPU: a title texture's host texture is made on its first use (the first-use half of #431) | AgneyaK |  |
| [#1975](https://github.com/Marvisak/VIctory/pull/1975) | HLE: match rpcrt4 UuidFromString nil-UUID and canonical string rules | mgunes1311-lab |  |
| [#1979](https://github.com/Marvisak/VIctory/pull/1979) | GPU: copies of single views reuse the buffers of dead ones | 0xkn0wh3r3 |  |
| [#1980](https://github.com/Marvisak/VIctory/pull/1980) | GPU: heap slot writes do not wait for a submission's texture uploads | 0xkn0wh3r3 |  |
| [#1981](https://github.com/Marvisak/VIctory/pull/1981) | Preserve the last complete save when a newer version is missing a blob | WerG0D |  |
| [#1984](https://github.com/Marvisak/VIctory/pull/1984) | GPU: return NV12 and P010 luma and chroma copy footprints | yuniskh |  |
| [#1986](https://github.com/Marvisak/VIctory/pull/1986) | GPU: a withheld range on a marked page does not hold back valid writes there | 0xkn0wh3r3 |  |
| [#1988](https://github.com/Marvisak/VIctory/pull/1988) | CLI: inspect names every DLL characteristics bit | wilu222 |  |
| [#1992](https://github.com/Marvisak/VIctory/pull/1992) | docs: update architecture scheme | IWillFall1337 |  |
| [#1994](https://github.com/Marvisak/VIctory/pull/1994) | Shaders: floor, ceil and fract read denormals as zeros | astrosteveo |  |
| [#1995](https://github.com/Marvisak/VIctory/pull/1995) | CLI: recommend a DXIL validator in translate --help | AlanQiang0418 |  |
| [#2001](https://github.com/Marvisak/VIctory/pull/2001) | HLE: resolve the local machine name to loopback | necrobytes |  |
| [#2005](https://github.com/Marvisak/VIctory/pull/2005) | GPU: log the missing D3D12 1.7 and skip the sharing test there | Calvix2k |  |
| [#2007](https://github.com/Marvisak/VIctory/pull/2007) | GPU: a pipeline library save the host has no memory for waits, not aborts (1.7 GB library on NVIDIA) | KevinPratap |  |
| [#2008](https://github.com/Marvisak/VIctory/pull/2008) | Fix #90: gate view_format with cfg(windows) so victory-gpu builds on Linux | tharun-37 |  |
| [#2009](https://github.com/Marvisak/VIctory/pull/2009) | Fix #721, #722: copy-back writes only changed bytes; mark pages current conditionally | tharun-37 |  |
| [#2015](https://github.com/Marvisak/VIctory/pull/2015) | Shader tests: check 125 more instructions on the hardware | astrosteveo |  |
| [#2018](https://github.com/Marvisak/VIctory/pull/2018) | Docs: rustdoc builds the GPU crate again (shader_cache's private link) | Gruzzberry |  |
| [#2019](https://github.com/Marvisak/VIctory/pull/2019) | Shader tests: serialize remaining Host::new tests with ONE_AT_A_TIME (#1882) | DenRakEiw |  |
| [#2021](https://github.com/Marvisak/VIctory/pull/2021) | GPU watch: ignore zero-length writes and remaps on unaligned addresses (#2011) | DenRakEiw |  |
| [#2026](https://github.com/Marvisak/VIctory/pull/2026) | Formats: bounds-check a detiled row once, not each run in it | 0xkn0wh3r3 |  |
| [#2031](https://github.com/Marvisak/VIctory/pull/2031) | GPU: heap samplers keep point mips when anisotropic, as static samplers do | 0xkn0wh3r3 |  |
| [#2033](https://github.com/Marvisak/VIctory/pull/2033) | Loader: append crash reports as single records | BaggyALPHA |  |
| [#2036](https://github.com/Marvisak/VIctory/pull/2036) | Formats: test BC1 endpoint prediction and quadrant packing | Dathinel |  |
| [#2041](https://github.com/Marvisak/VIctory/pull/2041) | Shaders: the fused multiply-add probe binds a sampler heap | 0xkn0wh3r3 |  |
| [#2043](https://github.com/Marvisak/VIctory/pull/2043) | Shaders: a pixel discard in a loop kills its lanes, as the console does | Deilach |  |
| [#2044](https://github.com/Marvisak/VIctory/pull/2044) | README: keep Windows build paths short (blake3's ml64 fails past ~170 characters) | Creaa |  |
| [#2052](https://github.com/Marvisak/VIctory/pull/2052) | Preserve NaN in HDR diagnostic captures | DoubleD17 |  |
| [#2053](https://github.com/Marvisak/VIctory/pull/2053) | GPU: log the host GPU, and warn when it is not the one to run on | rminzao |  |
| [#2057](https://github.com/Marvisak/VIctory/pull/2057) | Tests: a conformance title checks the runtime against the public GDK reference | 0xkn0wh3r3 |  |
| [#2061](https://github.com/Marvisak/VIctory/pull/2061) | Formats: guard XMA bitstream reads and sign extension at 0 and >=32 bits | fuzeofficlal |  |
| [#2065](https://github.com/Marvisak/VIctory/pull/2065) | HLE: allow oleaut32.dll in default host policy | fuzeofficlal |  |
| [#2066](https://github.com/Marvisak/VIctory/pull/2066) | Net: forward zero-address datagram disconnects | fuzeofficlal |  |
| [#2071](https://github.com/Marvisak/VIctory/pull/2071) | run: free the executable's bytes once its image is mapped | 0xkn0wh3r3 |  |
| [#2073](https://github.com/Marvisak/VIctory/pull/2073) | XUser: finding an unknown local id fails with ERROR_NOT_FOUND | 0xkn0wh3r3 |  |
| [#2074](https://github.com/Marvisak/VIctory/pull/2074) | DirectStorage: a queue submits itself past half its capacity, not at half | 0xkn0wh3r3 |  |
| [#2075](https://github.com/Marvisak/VIctory/pull/2075) | HLE: with no process task queue, a call given no queue fails with E_NO_TASK_QUEUE | 0xkn0wh3r3 |  |
| [#2079](https://github.com/Marvisak/VIctory/pull/2079) | Shaders: a relative move's program is built at its full range only if no bound is proven | 0xkn0wh3r3 |  |
| [#2082](https://github.com/Marvisak/VIctory/pull/2082) | Docs, tools: running under Proton, and the wineserver view-tree patch from #340 | desplmfao |  |
| [#2090](https://github.com/Marvisak/VIctory/pull/2090) | Add CPU-only execution-equivalence tests for Wave32 shader SSA and structured EXEC | chaseb64 |  |
| [#2098](https://github.com/Marvisak/VIctory/pull/2098) | GPU: an acceleration structure's view address is its location | moe-miau |  |
| [#2099](https://github.com/Marvisak/VIctory/pull/2099) | GPU: stop the descriptor view table growing without bound | 0xkn0wh3r3 |  |
| [#2103](https://github.com/Marvisak/VIctory/pull/2103) | Shaders: trace uniform v_mov_b32 and v_readfirstlane_b32 copies in resource tracking | fuzeofficlal |  |
| [#2109](https://github.com/Marvisak/VIctory/pull/2109) | Tools: record bounded Windows playtests | AwsAli05 |  |
| [#2110](https://github.com/Marvisak/VIctory/pull/2110) | GPU diagnostics: keep pipeline drop reasons until they are reported | AwsAli05 |  |
| [#2114](https://github.com/Marvisak/VIctory/pull/2114) | Input: VICTORY_KEYS drives the stand-in pad when no gamepad is connected | fiestorra |  |
| [#2134](https://github.com/Marvisak/VIctory/pull/2134) | GPU tests: a world frame's preparing at the published volumes (benchmark) | r4tedx |  |
| [#2139](https://github.com/Marvisak/VIctory/pull/2139) | HLE: implement synchronous XGameSave slots and unfiltered blob enumeration | fuzeofficlal |  |
| [#2141](https://github.com/Marvisak/VIctory/pull/2141) | HLE: registry lookups through kernelbase.dll reach the device-type overrides | BastienRodz |  |

## conflicts with pull requests (281)

| PR | Title | Author | Detail |
|---|---|---|---|
| [#159](https://github.com/Marvisak/VIctory/pull/159) | GPU: occlusion, statistics and timestamp queries run on the host | r4tedx | files: `README.md`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158 |
| [#171](https://github.com/Marvisak/VIctory/pull/171) | Shaders: lower v_mad_u64_u32 and v_mad_i64_i32 | n-romanovich | files: `crates/victory-shader/src/emit/mod.rs`; with #166 |
| [#200](https://github.com/Marvisak/VIctory/pull/200) | Shaders: lower v_div_scale_f32, v_div_fmas_f32 and v_div_fixup_f32 | n-romanovich | files: `crates/victory-shader/src/emit/mod.rs`; with #166 |
| [#212](https://github.com/Marvisak/VIctory/pull/212) | GPU, HLE: report host display HDR luminance and configure swap chain color space | n-romanovich | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158 |
| [#286](https://github.com/Marvisak/VIctory/pull/286) | Shaders: accept known D3D_MIN_PRECISION signature elements | tarcisioads | files: `crates/victory-shader/src/emit/mod.rs`; with #166 |
| [#328](https://github.com/Marvisak/VIctory/pull/328) | Shaders: lower MSAA image_load and image_get_resinfo | tarcisioads | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #317 |
| [#332](https://github.com/Marvisak/VIctory/pull/332) | Shaders: lower partial-channel image_store | tarcisioads | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #317 |
| [#465](https://github.com/Marvisak/VIctory/pull/465) | Shaders: lower v_dot2_f32_f16 and v_dot2c_f32_f16 | Baconzido | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #395, #457 |
| [#485](https://github.com/Marvisak/VIctory/pull/485) | GPU: preserve compatible read states for read-only depth targets | fuzeofficlal | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450 |
| [#492](https://github.com/Marvisak/VIctory/pull/492) | GPU: substitute R9G9B9E5_SHAREDEXP render targets and UAVs with R16G16B16A16_FLOAT | Yaekai | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450 |
| [#511](https://github.com/Marvisak/VIctory/pull/511) | Shaders: coalesce aligned constant-buffer loads | aycow | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385 |
| [#521](https://github.com/Marvisak/VIctory/pull/521) | Shaders: model the compute thread-group info SGPR's wave count | AnoHunted | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385 |
| [#559](https://github.com/Marvisak/VIctory/pull/559) | GPU: track mip states for 2D color targets | fuzeofficlal | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450 |
| [#577](https://github.com/Marvisak/VIctory/pull/577) | dstorage: retain queued file references and validate null out pointers | DenRakEiw | files: `crates/victory-hle/src/modules/dstorage.rs`; with #130, #430 |
| [#594](https://github.com/Marvisak/VIctory/pull/594) | GPU: track texture subresource states | broccolize | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450 |
| [#606](https://github.com/Marvisak/VIctory/pull/606) | Shaders: ds_swizzle_b32 offsets from 0xC000 are the rotate and FFT modes | upipibi | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385 |
| [#638](https://github.com/Marvisak/VIctory/pull/638) | GPU diagnostics: find what the world hang waits for, and look under a black frame | claude-GTA6 | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450 |
| [#644](https://github.com/Marvisak/VIctory/pull/644) | GPU: spill oversized host root signatures' user data to root CBVs | nathanbzrr | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450 |
| [#660](https://github.com/Marvisak/VIctory/pull/660) | Shaders: drop a wave64 mask whose low dword is seen with two high dwords | coderanonymus | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457 |
| [#662](https://github.com/Marvisak/VIctory/pull/662) | DSP: refuse XDspDisconnect with active streams; XDSP_E_NO_DEVICE_RESOURCES past the stream limit | coderanonymus | files: `crates/victory-hle/src/modules/dsp.rs`; with #240 |
| [#668](https://github.com/Marvisak/VIctory/pull/668) | Shaders: allow abs/neg source modifiers on v_cvt_{u16,i16}_f16 | DenRakEiw | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385 |
| [#688](https://github.com/Marvisak/VIctory/pull/688) | GPU: take back the dynamic descriptor slots of lists that ran when a heap runs short | iurysf | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450 |
| [#926](https://github.com/Marvisak/VIctory/pull/926) | Shaders: 16-bit compares read the low halves whatever op_sel says, as RDNA2 does | BastienRodz | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385 |
| [#929](https://github.com/Marvisak/VIctory/pull/929) | Audio: a silent playback endpoint when the host has no render device | vladbondarenko97 | files: `crates/victory-hle/src/modules/audio.rs`; with #130, #478 |
| [#969](https://github.com/Marvisak/VIctory/pull/969) | GPU: load the Agility SDK through ID3D12SDKConfiguration1 (#749) | mgunes1311-lab | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450 |
| [#970](https://github.com/Marvisak/VIctory/pull/970) | GPU: aspect-aware concrete format resolution for typeless views (fixes #882, #876) | pushkarreddyy | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933 |
| [#1013](https://github.com/Marvisak/VIctory/pull/1013) | perf(shader): speed up control-flow structurisation | matheusfrta | files: `crates/victory-shader/src/structure.rs`; with #385 |
| [#1059](https://github.com/Marvisak/VIctory/pull/1059) | Loader: VICTORY_PATCH writes bytes into a loaded module after relocation | Halfect | files: `crates/victory-loader/src/lib.rs`; with #454, #1017 |
| [#1090](https://github.com/Marvisak/VIctory/pull/1090) | Shaders: lower constant 64-bit integer compares without INT64 | tarcisioads | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1114](https://github.com/Marvisak/VIctory/pull/1114) | CLI: replay selected runtime shader hashes in offline translation | Razdva122 | files: `README.md`, `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`; with #130, #158, #224, #280, #378, #491, #506, #517 |
| [#1133](https://github.com/Marvisak/VIctory/pull/1133) | Descriptors: clamp view indices past a capped host heap to a null slot | rivaldodev | files: `crates/victory-gpu/src/host.rs`, `crates/victory-shader/src/emit/shader.rs`; with #67, #130, #158, #450, #470, #1065 |
| [#1168](https://github.com/Marvisak/VIctory/pull/1168) | GPU: implement SetMarker, BeginEvent, and EndEvent on command lists (slots 0x1c0-0x1d0) | geiyiliyabumo | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1166 |
| [#1178](https://github.com/Marvisak/VIctory/pull/1178) | Diagnostics: trace a DirectStorage request from submission to decoder outcome | Razdva122 | files: `README.md`, `crates/victory-hle/src/modules/dstorage.rs`; with #130, #158, #224, #280, #430, #491, #506, #686 |
| [#1183](https://github.com/Marvisak/VIctory/pull/1183) | Keep shader metadata variants in the offline translation census | Jitheswar | files: `apps/victory/src/translate.rs`; with #378, #517, #673, #1017 |
| [#1185](https://github.com/Marvisak/VIctory/pull/1185) | Stream shader archive scans in bounded batches | Jitheswar | files: `apps/victory/src/translate.rs`; with #378, #517, #673, #1017 |
| [#1192](https://github.com/Marvisak/VIctory/pull/1192) | Loader: skip empty executable sections in tls::redirect and add tls.rs unit tests | geiyiliyabumo | files: `crates/victory-loader/src/tls.rs`; with #188 |
| [#1193](https://github.com/Marvisak/VIctory/pull/1193) | HLE: add synthetic unit tests for storage.rs and host.rs | geiyiliyabumo | files: `crates/victory-hle/src/modules/storage.rs`; with #1017 |
| [#1196](https://github.com/Marvisak/VIctory/pull/1196) | HLE: fix self-closing XML element body parsing in runtime.rs and add unit tests | geiyiliyabumo | files: `crates/victory-hle/src/runtime.rs`; with #1017 |
| [#1201](https://github.com/Marvisak/VIctory/pull/1201) | Loader: free mapped module on TLS redirect error and add windows.rs unit tests | geiyiliyabumo | files: `crates/victory-loader/src/windows.rs`; with #454, #1111 |
| [#1207](https://github.com/Marvisak/VIctory/pull/1207) | GPU: validate input layouts and aligned vertex offsets in pipeline/host.rs with unit tests | geiyiliyabumo | files: `crates/victory-gpu/src/pipeline/host.rs`; with #158, #491, #1152 |
| [#1213](https://github.com/Marvisak/VIctory/pull/1213) | HLE: reject null handle in WinHttpSetOption proxy path and expand net.rs tests (1 -> 17) | geiyiliyabumo | files: `crates/victory-hle/src/modules/net.rs`; with #997 |
| [#1224](https://github.com/Marvisak/VIctory/pull/1224) | fix(hle): coalesce split placeholders when mapping physical pages | matheusfrta | files: `crates/victory-hle/src/modules/xmem.rs`; with #130, #224, #1009 |
| [#1244](https://github.com/Marvisak/VIctory/pull/1244) | Validate VICTORY_PAIRS before translating shaders | DoubleD17 | files: `apps/victory/src/translate.rs`; with #378, #517, #673, #1017 |
| [#1256](https://github.com/Marvisak/VIctory/pull/1256) | Shaders: decode cube-array slice and face in image lowering | fuzeofficlal | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1271](https://github.com/Marvisak/VIctory/pull/1271) | Shaders: infer heap UAVs and mark glc UAV handles globally coherent | fuzeofficlal | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/shader.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1273](https://github.com/Marvisak/VIctory/pull/1273) | GPU: mirror placed unordered texture views | fuzeofficlal | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450, #1166, #1233 |
| [#1288](https://github.com/Marvisak/VIctory/pull/1288) | Shaders: a typed fetch inside a dword reads the aligned pair | pablogonz12 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1312](https://github.com/Marvisak/VIctory/pull/1312) | Shaders: mask v_readlane/v_writelane lane index and bypass EXEC on writelane | ashrafsdrop | files: `crates/victory-shader/src/emit/valu_tests.rs`, `crates/victory-shader/src/ir.rs`; with #105, #284, #369, #379, #385, #1033, #1046, #1259 |
| [#1327](https://github.com/Marvisak/VIctory/pull/1327) | Shaders: apply bitwise modifiers to v_cmp_class_f32 source operands | fuzeofficlal | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262 |
| [#1328](https://github.com/Marvisak/VIctory/pull/1328) | Shaders: bitwise wave prefixes as WaveMultiPrefixOp | paulomatias97-create | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1361](https://github.com/Marvisak/VIctory/pull/1361) | Shaders: hardware test for the scalar bit-scan, reverse and abs ops | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262 |
| [#1362](https://github.com/Marvisak/VIctory/pull/1362) | Shaders: hardware test for s_lshl1..4_add_u32 | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262 |
| [#1364](https://github.com/Marvisak/VIctory/pull/1364) | Shaders: hardware test for s_mul_i32, s_mul_hi_u32 and s_mul_hi_i32 | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262 |
| [#1365](https://github.com/Marvisak/VIctory/pull/1365) | Shaders: hardware test for s_bfm_b32 and s_bitset0/1_b32 | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262 |
| [#1369](https://github.com/Marvisak/VIctory/pull/1369) | GPU: query native format support and multisample quality levels | fuzeofficlal | files: `crates/victory-gpu/src/device.rs`; with #958 |
| [#1374](https://github.com/Marvisak/VIctory/pull/1374) | GPU: read same-list GPU writes in placed buffer-to-texture copies | fuzeofficlal | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/list.rs`, `crates/victory-gpu/src/resource.rs`; with #24, #67, #130, #158, #450, #933, #1017, #1228 |
| [#1385](https://github.com/Marvisak/VIctory/pull/1385) | Shaders: narrow mesh output bounds from GS_ALLOC_REQ M0 analysis (#201) | Ctrl-Alt-Haze | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1396](https://github.com/Marvisak/VIctory/pull/1396) | GPU: preserve sub-64 KiB alignments and check bounds in allocation_info | fuzeofficlal | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1281 |
| [#1407](https://github.com/Marvisak/VIctory/pull/1407) | Shaders: a lane-mask join of masks and numbers, read as a number | marquezinii | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/structure.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1408](https://github.com/Marvisak/VIctory/pull/1408) | Shaders: lower float wave prefix sums via lane reads | fuzeofficlal | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1417](https://github.com/Marvisak/VIctory/pull/1417) | Shaders: lower SV_RenderTargetArrayIndex and SV_ViewportArrayIndex exports | fuzeofficlal | files: `crates/victory-shader/src/emit/shader.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #470, #1046, #1065, #1259, #1262 |
| [#1420](https://github.com/Marvisak/VIctory/pull/1420) | Shaders: a mask built up in a loop every lane runs the same way (stacked on #1407) | marquezinii | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/structure.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1422](https://github.com/Marvisak/VIctory/pull/1422) | GPU: ignore clear values on non-target textures and avoid caching failed host textures in PLACED_TEXTURES | ESAT586 | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1281 |
| [#1428](https://github.com/Marvisak/VIctory/pull/1428) | GPU: guard OMSetRenderTargets handle expansion on empty, null, or oversized calls | fuzeofficlal | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228 |
| [#1429](https://github.com/Marvisak/VIctory/pull/1429) | GPU: persist early-pipeline keys even without a host pipeline library | fuzeofficlal | files: `crates/victory-gpu/src/pipeline/library.rs`; with #983 |
| [#1430](https://github.com/Marvisak/VIctory/pull/1430) | Input: ignore all-zero XInput slots until they report input | fuzeofficlal | files: `crates/victory-hle/src/modules/gameinput.rs`; with #613, #927, #1329 |
| [#1432](https://github.com/Marvisak/VIctory/pull/1432) | HLE: set ERROR_PROC_NOT_FOUND for missing pseudo-module exports | fuzeofficlal | files: `crates/victory-hle/src/modules/kernel.rs`; with #512, #1419 |
| [#1434](https://github.com/Marvisak/VIctory/pull/1434) | Loader: match title DLL filenames ignoring ASCII case | fuzeofficlal | files: `crates/victory-loader/src/windows.rs`; with #454, #1111, #1304 |
| [#1460](https://github.com/Marvisak/VIctory/pull/1460) | Shaders: keep distinct critical-edge blocks for duplicate branch targets | piyonchu | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1473](https://github.com/Marvisak/VIctory/pull/1473) | Shaders: track per-lane export state across mixed pixel-shader joins | Texbirds | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1493](https://github.com/Marvisak/VIctory/pull/1493) | Shaders: declare SamplerComparisonState for immediate comparison samplers and add GPU test | piyonchu | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457, #1017, #1112, #1263, #1315 |
| [#1503](https://github.com/Marvisak/VIctory/pull/1503) | GPU: fail fast when the host D3D12 runtime is unusable | degradka | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424 |
| [#1511](https://github.com/Marvisak/VIctory/pull/1511) | [BUNDLE] DXIL emission and container robustness: 3 PRs, #343 #1303 #477 (rebuilt on main 70564ae) | MamaLeVrai | files: `crates/victory-shader/src/emit/validator.rs`; with #477, #684 |
| [#1525](https://github.com/Marvisak/VIctory/pull/1525) | Reuse fenced GPU upload scratch across command lists | franticfrostin | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424 |
| [#1529](https://github.com/Marvisak/VIctory/pull/1529) | Input: keyboard and mouse played like a PC game (rebindable [controls]) | claude-GTA6 | files: `apps/victory/src/run.rs`, `crates/victory-hle/src/modules/gameinput.rs`; with #130, #224, #613, #927, #1009, #1017, #1329 |
| [#1536](https://github.com/Marvisak/VIctory/pull/1536) | Shaders: assemble dword-straddling global_load_ushort and global_load_sshort | oni-swr | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1541](https://github.com/Marvisak/VIctory/pull/1541) | CLI: list-diff compares two translate listings program by program | sinan-cmyk | files: `README.md`, `apps/victory/src/main.rs`; with #130, #158, #224, #280, #378, #491, #506, #535 |
| [#1546](https://github.com/Marvisak/VIctory/pull/1546) | GPU: route indirect dispatch_mesh_x through State::execute_indirect | n-romanovich | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#1548](https://github.com/Marvisak/VIctory/pull/1548) | GPU: preserve full DYNAMIC descriptor range when host view heap is capped at PORTABLE_VIEWS | n-romanovich | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449 |
| [#1554](https://github.com/Marvisak/VIctory/pull/1554) | Shaders: flush subnormals in f32 MAD and MAC | Calvix2k | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1563](https://github.com/Marvisak/VIctory/pull/1563) | Shaders: link mesh and pixel programs whose mesh outputs pass 32 KiB | claude-GTA6 | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457, #1017, #1112, #1263, #1315 |
| [#1564](https://github.com/Marvisak/VIctory/pull/1564) | GPU: MakeResident succeeds, as the console's memory is always resident | LouisRiverstone | files: `crates/victory-gpu/src/device.rs`; with #958 |
| [#1565](https://github.com/Marvisak/VIctory/pull/1565) | GPU: Evict succeeds, as the console's memory is never evicted | LouisRiverstone | files: `crates/victory-gpu/src/device.rs`; with #958 |
| [#1566](https://github.com/Marvisak/VIctory/pull/1566) | GPU: SetStablePowerState is accepted and ignored | LouisRiverstone | files: `crates/victory-gpu/src/device.rs`; with #958 |
| [#1569](https://github.com/Marvisak/VIctory/pull/1569) | Shaders: lower v_swaprel_b32 | piyonchu | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1575](https://github.com/Marvisak/VIctory/pull/1575) | Shaders: sweep typed buffer formats and f16 ALU against exact references | DovisLe | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1576](https://github.com/Marvisak/VIctory/pull/1576) | HLE: implement IUnknown queries for COM objects | Calvix2k | files: `crates/victory-hle/src/modules/display.rs`, `crates/victory-hle/src/modules/xgame.rs`, `crates/victory-hle/src/modules/xgamesave.rs`; with #228, #590, #1017, #1110, #1123, #1155, #1174 |
| [#1577](https://github.com/Marvisak/VIctory/pull/1577) | Shaders: expand 64-bit inline constants as signed ints and binary64 floats | mgunes1311-lab | files: `crates/victory-shader/src/emit/valu_tests.rs`, `crates/victory-shader/src/structure.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1587](https://github.com/Marvisak/VIctory/pull/1587) | Shaders: ballot divergent-loop masks at reconverged reads (#1498) | Ctrl-Alt-Haze | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`, `crates/victory-shader/src/structure.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1588](https://github.com/Marvisak/VIctory/pull/1588) | Shaders: single-round v_fma_mixlo_f16 and v_fma_mixhi_f16 | Ctrl-Alt-Haze | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1598](https://github.com/Marvisak/VIctory/pull/1598) | Shaders: lower v_mad_f16 and v_mac_f16 with unfused binary16 product rounding | geiyiliyabumo | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1599](https://github.com/Marvisak/VIctory/pull/1599) | Shaders: 64-bit integer compares on their dwords (AMD orders a shared-dword i64 compare wrong) | alexdeliu | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1608](https://github.com/Marvisak/VIctory/pull/1608) | Shaders: single-round v_fma_mixlo_f16 and v_fma_mixhi_f16 to binary16 | geiyiliyabumo | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1613](https://github.com/Marvisak/VIctory/pull/1613) | GPU: render targets keep their slots (null views for targets the host cannot bind), and no RTV or clear without the render-target use | AgneyaK | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1233 |
| [#1614](https://github.com/Marvisak/VIctory/pull/1614) | Shader tests: a cross-device VALU fuzzer (hardware adapters vs WARP) | alexdeliu | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457, #1017, #1112, #1263, #1315, #1582 |
| [#1621](https://github.com/Marvisak/VIctory/pull/1621) | bound the bytes one part's native blobs unpack to and hand out | imjustprism | files: `crates/victory-shader/src/native.rs`; with #589, #1315 |
| [#1630](https://github.com/Marvisak/VIctory/pull/1630) | GPU: adapter choice and logging, presenter backoff, a texture-description ladder, and a display-deferral test a headless session can pass | fiestorra | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628 |
| [#1631](https://github.com/Marvisak/VIctory/pull/1631) | Tests: allow prebuilt HLE probe DLL via VICTORY_HLE_PROBE_DLL | fuzeofficlal | files: `crates/victory-loader/tests/load_probe.rs`; with #1218 |
| [#1639](https://github.com/Marvisak/VIctory/pull/1639) | PE: map header RVAs and read TLS data from the headers | sandivd | files: `crates/victory-pe/src/image.rs`; with #324, #1017 |
| [#1648](https://github.com/Marvisak/VIctory/pull/1648) | GPU: mesh shaders emulated as compute + unpack vertex on SM 6.6 cards without a mesh tier | yoraudev | files: `README.md`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline/host.rs`; with #67, #130, #158, #224, #280, #378, #450, #491 |
| [#1649](https://github.com/Marvisak/VIctory/pull/1649) | shader: lower s_cmov_b32, s_cmov_b64, and s_cmovk_i32 | geiyiliyabumo | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1668](https://github.com/Marvisak/VIctory/pull/1668) | Shaders: read inline floats as f32 bits in 16-bit integer ops | Calvix2k | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1669](https://github.com/Marvisak/VIctory/pull/1669) | shader: handle dword-straddling 16-bit typed buffer loads at byte offset 3 | geiyiliyabumo | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1673](https://github.com/Marvisak/VIctory/pull/1673) | HLE: XTaskQueueTerminate cancels only its own tasks on shared ports | Calvix2k | files: `crates/victory-hle/src/modules/xasync.rs`; with #1435 |
| [#1676](https://github.com/Marvisak/VIctory/pull/1676) | Shaders, CLI: name a host without mesh shaders in the pipeline census | fiestorra | files: `apps/victory/src/translate.rs`; with #378, #517, #673, #1017, #1315 |
| [#1685](https://github.com/Marvisak/VIctory/pull/1685) | Shaders: lower v_cmp_{f,t}_{i64,u64} constant 64-bit compares | Calvix2k | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1686](https://github.com/Marvisak/VIctory/pull/1686) | Shaders: lower pixel shader SV_Coverage input via dx.op.coverage | Calvix2k | files: `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #284, #379, #385, #457, #1017, #1046, #1112 |
| [#1687](https://github.com/Marvisak/VIctory/pull/1687) | Shaders: lower image_gather4_c_lz(_o) and offset sample variants | Calvix2k | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1688](https://github.com/Marvisak/VIctory/pull/1688) | Shaders: lower v_mad_u32_u16, v_mad_i32_i16, and v_lerp_u8 | Calvix2k | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1690](https://github.com/Marvisak/VIctory/pull/1690) | GPU: zero pipeline library description padding | Calvix2k | files: `crates/victory-gpu/src/pipeline/host.rs`; with #158, #491, #1152, #1315 |
| [#1693](https://github.com/Marvisak/VIctory/pull/1693) | victory-shader: lower TFE image loads/samples/gathers via CheckAccessFullyMapped | tjthebigbearcat-hub | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/shader.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1702](https://github.com/Marvisak/VIctory/pull/1702) | HLE: add VICTORY_NO_HOST_PAD to hide host gamepads | Calvix2k | files: `crates/victory-hle/src/modules/gameinput.rs`; with #613, #927, #1329 |
| [#1704](https://github.com/Marvisak/VIctory/pull/1704) | FidelityFX: answer the upscale calls with DLSS through Streamline (stacked on #1868) | marquezinii | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450, #1233, #1449, #1542, #1700 |
| [#1709](https://github.com/Marvisak/VIctory/pull/1709) | Shaders: flush subnormals in the f32 cube coordinates | Calvix2k | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1715](https://github.com/Marvisak/VIctory/pull/1715) | FidelityFX: DLSS frame generation on frames the title upscales (stacked on #1704) | marquezinii | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1233, #1283, #1424, #1449 |
| [#1717](https://github.com/Marvisak/VIctory/pull/1717) | HLE: the retail console ID has the documented dotted form | davutsimsekk | files: `crates/victory-hle/src/modules/xsystem.rs`; with #939, #1657 |
| [#1720](https://github.com/Marvisak/VIctory/pull/1720) | report apu heap allocations that fail | imjustprism | files: `crates/victory-hle/src/modules/xmem.rs`; with #130, #224, #1009, #1264, #1445 |
| [#1723](https://github.com/Marvisak/VIctory/pull/1723) | GPU diagnostics: pipeline statistics per noted command with VICTORY_GPU_TIMING | LarryBebou | files: `README.md`, `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #224, #280, #450, #491, #506 |
| [#1731](https://github.com/Marvisak/VIctory/pull/1731) | GPU: occlusion queries measure on the host instead of reporting every draw visible | fiestorra | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#1732](https://github.com/Marvisak/VIctory/pull/1732) | GPU: f32_to_half keeps NaN and rounds normals to nearest even (#843) | moe-miau | files: `crates/victory-gpu/src/primitive.rs`; with #1418 |
| [#1737](https://github.com/Marvisak/VIctory/pull/1737) | Shaders: honor permlane EXEC and FI/BC flags (on #1014) | jaschinski97-ops | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1749](https://github.com/Marvisak/VIctory/pull/1749) | GPU: indirect mesh dispatches run as an indirect execution of one record | bemalinho | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#1756](https://github.com/Marvisak/VIctory/pull/1756) | Shaders: image atomics on a handle of their own element type | paulomatias97-create | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457, #1017, #1112, #1263, #1315, #1582, #1712 |
| [#1757](https://github.com/Marvisak/VIctory/pull/1757) | Shaders: one arena per program for instruction operands and effects | Luislariosss | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/ir.rs`, `crates/victory-shader/src/resource.rs`; with #105, #139, #166, #259, #284, #369, #379, #385 |
| [#1763](https://github.com/Marvisak/VIctory/pull/1763) | GPU: refuse typed UAVs in unsupported formats regardless of family or castability | n-romanovich | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762 |
| [#1767](https://github.com/Marvisak/VIctory/pull/1767) | translate: report the validator's own reason, not the line that hides it | fiestorra | files: `apps/victory/src/translate.rs`; with #378, #517, #673, #1017, #1315 |
| [#1769](https://github.com/Marvisak/VIctory/pull/1769) | Shaders: LDS accesses outside the program's allocation read 0 and drop their writes | Impre-visible | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/shader.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1770](https://github.com/Marvisak/VIctory/pull/1770) | Descriptor tests: take the capped heap's design into account | kais7654 | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762 |
| [#1772](https://github.com/Marvisak/VIctory/pull/1772) | HLE: handle verbatim/drive-root mounts and UNC/verbatim GetDiskFreeSpaceW roots | n-romanovich | files: `crates/victory-hle/src/modules/files.rs`; with #130, #421, #1383 |
| [#1776](https://github.com/Marvisak/VIctory/pull/1776) | GPU: exclude uncastable formats (68, 69, 89) from castable texture lists | 0xKaji | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628 |
| [#1780](https://github.com/Marvisak/VIctory/pull/1780) | GPU: check every unordered texture view against the host's typed-store support (#1754) | LarryBebou | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778 |
| [#1787](https://github.com/Marvisak/VIctory/pull/1787) | GPU: textures drop unordered access in formats the host cannot write (NVIDIA B5G5R5A1) | Danigamer05 | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628 |
| [#1791](https://github.com/Marvisak/VIctory/pull/1791) | Shader tests: keep the FSPX dot4 test off folded constants | Calvix2k | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1793](https://github.com/Marvisak/VIctory/pull/1793) | FidelityFX: Series X switches to the host upscaler and its render scale (rework of #1284) | AwkGrep256 | files: `README.md`; with #130, #158, #224, #280, #491, #506, #686, #1218 |
| [#1800](https://github.com/Marvisak/VIctory/pull/1800) | GPU: log title slots past a capped heap once per 64 Ki, not each | AgneyaK | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778 |
| [#1803](https://github.com/Marvisak/VIctory/pull/1803) | GPU: copy depth twins from pre-list states in prelude and route depth UAV barriers to twin planes | mgunes1311-lab | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#1804](https://github.com/Marvisak/VIctory/pull/1804) | Shader tests: run v_bfm_b32, v_bcnt_u32_b32, v_ffbl_b32, and v_cvt_pk_u8_f32 on the GPU | n-romanovich | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1810](https://github.com/Marvisak/VIctory/pull/1810) | Shader tests: run 24-bit multiply and multiply-add ops on the GPU | n-romanovich | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1811](https://github.com/Marvisak/VIctory/pull/1811) | Shader tests: run v_perm_b32 on the GPU | n-romanovich | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1812](https://github.com/Marvisak/VIctory/pull/1812) | Shader tests: run v_mul_hi_u32 and v_mul_hi_i32 on the GPU | n-romanovich | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1817](https://github.com/Marvisak/VIctory/pull/1817) | GPU: enforce logical indirect buffer spans | aycow | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#1818](https://github.com/Marvisak/VIctory/pull/1818) | GPU: preserve disjoint shadow refresh ranges | aycow | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#1819](https://github.com/Marvisak/VIctory/pull/1819) | Shaders: translated loops are left after 2^14 iterations (was 2^20) | Exopus-0x65f7ad0 | files: `README.md`, `crates/victory-shader/src/emit/mod.rs`; with #130, #158, #166, #224, #280, #457, #491, #506 |
| [#1824](https://github.com/Marvisak/VIctory/pull/1824) | Shaders: declare each dx.op call once | slatefield | files: `crates/victory-shader/src/emit/shader.rs`; with #470, #1065, #1262, #1716, #1759 |
| [#1825](https://github.com/Marvisak/VIctory/pull/1825) | GPU: keep the view-stride table once per list, not per draw stage | slatefield | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450, #1233, #1449, #1542, #1700, #1762 |
| [#1826](https://github.com/Marvisak/VIctory/pull/1826) | GPU: build an object name only where the log takes it | slatefield | files: `crates/victory-gpu/src/object.rs`; with #541, #1418 |
| [#1830](https://github.com/Marvisak/VIctory/pull/1830) | Config: `victory config` exits nonzero on an asset policy violation | Ingenioeren | files: `apps/victory/src/main.rs`; with #378, #506, #535, #1017 |
| [#1832](https://github.com/Marvisak/VIctory/pull/1832) | GPU: drop the unordered-access permission a format cannot carry, before any call names it | fiestorra | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628 |
| [#1837](https://github.com/Marvisak/VIctory/pull/1837) | Display: the title's screenshots save the last presented frame | kais7654 | files: `crates/victory-gpu/src/host.rs`, `crates/victory-hle/src/modules/display.rs`; with #67, #130, #158, #450, #1017, #1155, #1283, #1424 |
| [#1840](https://github.com/Marvisak/VIctory/pull/1840) | Shaders: discover irreducible loops with iterative SCC decomposition | VlREX | files: `crates/victory-shader/src/ir.rs`; with #105, #369, #1033, #1593, #1833 |
| [#1843](https://github.com/Marvisak/VIctory/pull/1843) | GPU: unordered views only in formats the host stores to (NVIDIA device removal on B5G5R5A1) | KevinPratap | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778 |
| [#1847](https://github.com/Marvisak/VIctory/pull/1847) | Shaders: reconstruct image descriptor dword 0 as a heap-slot proxy | AdamRchd | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1855](https://github.com/Marvisak/VIctory/pull/1855) | victory-gpu: clamp constant buffer views to their registered buffer | mgunes1311-lab | files: `crates/victory-gpu/src/bindless.rs`, `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778 |
| [#1856](https://github.com/Marvisak/VIctory/pull/1856) | Shaders: programs without FMad disable math refactoring | rminzao | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/shader.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1859](https://github.com/Marvisak/VIctory/pull/1859) | GPU: reject overflowing and non-buffer descriptor spans before shadow… | VlREX | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778 |
| [#1867](https://github.com/Marvisak/VIctory/pull/1867) | GPU: merge shadow write-back bytes in AVX2 vectors with scalar fallback | VlREX | files: `crates/victory-gpu/src/shadow.rs`; with #602, #1692, #1743 |
| [#1868](https://github.com/Marvisak/VIctory/pull/1868) | FidelityFX: write an RGB9E5 output NVIDIA cannot store to through an FP16 texture (fixes a device removal) | marquezinii | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450, #1233, #1449, #1542, #1700, #1762 |
| [#1869](https://github.com/Marvisak/VIctory/pull/1869) | Perf/indexed copyback ranges | VlREX | files: `crates/victory-gpu/src/memory.rs`; with #1865 |
| [#1874](https://github.com/Marvisak/VIctory/pull/1874) | Shaders: hardware test for the 32-bit scalar logic ops and s_ashr_i32 | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1875](https://github.com/Marvisak/VIctory/pull/1875) | Shaders: hardware test for the scalar add/sub carry chain and the SOPK forms | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1876](https://github.com/Marvisak/VIctory/pull/1876) | Shaders: hardware test for the VOP2 shifts, min/max and logic ops | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1877](https://github.com/Marvisak/VIctory/pull/1877) | Shaders: hardware test for the 16-bit integer ops | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1878](https://github.com/Marvisak/VIctory/pull/1878) | Shaders: hardware test for v_cvt_f32_ubyte0..3 | LouisRiverstone | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1879](https://github.com/Marvisak/VIctory/pull/1879) | Shader tests: keep dot4 test operands runtime for RDNA1 | n-romanovich | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1880](https://github.com/Marvisak/VIctory/pull/1880) | Audio: refill XMA rings from a decoded reserve on the title's thread | beamdotdev | files: `crates/victory-hle/src/modules/audio.rs`; with #130, #478, #1537 |
| [#1883](https://github.com/Marvisak/VIctory/pull/1883) | Shaders: lane masks a traversal keeps as numbers | powplusplus | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1885](https://github.com/Marvisak/VIctory/pull/1885) | Shaders, GPU: one back edge per loop header, and a large stack for pipeline threads | powplusplus | files: `crates/victory-shader/src/emit/bitcode.rs`, `crates/victory-shader/src/emit/shader.rs`; with #470, #1065, #1262, #1519, #1646, #1716, #1759 |
| [#1886](https://github.com/Marvisak/VIctory/pull/1886) | Shaders: lower pixel SV_Coverage input via dx.op.coverage | n-romanovich | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457, #1017, #1112, #1263, #1315, #1582, #1712 |
| [#1887](https://github.com/Marvisak/VIctory/pull/1887) | Shader tests: the test host's structured buffer views are raw (#1835) | carlos-seiffert | files: `crates/victory-shader/src/emit/diff_tests.rs`, `crates/victory-shader/src/emit/host.rs`; with #684, #1315, #1759, #1762, #1857 |
| [#1888](https://github.com/Marvisak/VIctory/pull/1888) | GPU: cache alias-copy footprint plans and pool temporary copy buffers | nathanbzrr | files: `crates/victory-gpu/src/texture.rs`; with #130, #601, #602, #1547, #1692, #1873 |
| [#1897](https://github.com/Marvisak/VIctory/pull/1897) | end the stack walk where rtlvirtualunwind faults | imjustprism | files: `crates/victory-loader/src/crash.rs`; with #1773 |
| [#1898](https://github.com/Marvisak/VIctory/pull/1898) | GPU: dependency-aware two-stage shader translation and pipeline scheduling (#1509) | nathanbzrr | files: `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/pipeline/make.rs`; with #130, #158, #491, #1017, #1203, #1315, #1505, #1728 |
| [#1900](https://github.com/Marvisak/VIctory/pull/1900) | Tests: the dot products run with operands loaded at run time, not literals | fiestorra | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1904](https://github.com/Marvisak/VIctory/pull/1904) | GPU: VICTORY_LIBRARY_TIMING measures the host FidelityFX upscaler's GPU and CPU cost | AwkGrep256 | files: `README.md`, `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #224, #280, #450, #491, #506 |
| [#1908](https://github.com/Marvisak/VIctory/pull/1908) | Keep the shader cache warm across emitter test edits | necrobytes | files: `crates/victory-shader/build.rs`; with #486 |
| [#1917](https://github.com/Marvisak/VIctory/pull/1917) | FidelityFX: add opt-in AMD frame generation to host upscaling | AwkGrep256 | files: `README.md`, `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #224, #280, #450, #491, #506 |
| [#1920](https://github.com/Marvisak/VIctory/pull/1920) | Shaders: v_mad_f32 and v_mac_f32 multiply apart from their add | kais7654 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1926](https://github.com/Marvisak/VIctory/pull/1926) | GPU: boot on NVIDIA when a format cannot be a typed UAV | QuentinDanblon | files: `README.md`, `crates/victory-gpu/src/descriptor/host.rs`; with #130, #158, #224, #280, #491, #506, #686, #1218 |
| [#1931](https://github.com/Marvisak/VIctory/pull/1931) | GPU: cache shader resource metadata with DXIL | nicolasveilleux | files: `crates/victory-gpu/src/pipeline.rs`; with #130, #158, #491, #1017, #1315 |
| [#1932](https://github.com/Marvisak/VIctory/pull/1932) | DStorage: preserve GetLastError across heap allocation in open_file and permit shared write/delete | ashrafsdrop | files: `crates/victory-hle/src/modules/dstorage.rs`; with #130, #430, #1383, #1457, #1719 |
| [#1934](https://github.com/Marvisak/VIctory/pull/1934) | GPU: resolve initial texture uploads and host_written at submission (#1891) | zondy1i | files: `crates/victory-gpu/src/list.rs`, `crates/victory-gpu/src/resource.rs`, `crates/victory-gpu/src/texture.rs`; with #24, #67, #130, #158, #450, #601, #602, #933 |
| [#1935](https://github.com/Marvisak/VIctory/pull/1935) | XMem: recycle freed physical page numbers on allocation | ashrafsdrop | files: `crates/victory-hle/src/modules/xmem.rs`; with #130, #224, #1009, #1264, #1445 |
| [#1936](https://github.com/Marvisak/VIctory/pull/1936) | GPU: refuse a translated shader the DXIL validator rejects | Luislariosss | files: `crates/victory-gpu/src/shader_cache.rs`; with #130, #415, #1263 |
| [#1937](https://github.com/Marvisak/VIctory/pull/1937) | Codex/fix gpu castable test 2026 11 25 | lollo1977 | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450, #1233, #1449, #1542, #1700, #1762 |
| [#1938](https://github.com/Marvisak/VIctory/pull/1938) | Shaders: return 0 for unused depth and height channels in image_get_resinfo | ashrafsdrop | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1939](https://github.com/Marvisak/VIctory/pull/1939) | Shaders: truncate s_buffer_load terms to dword alignment independently | ashrafsdrop | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1941](https://github.com/Marvisak/VIctory/pull/1941) | XSystem: report analytics device family as Windows.Xbox | ashrafsdrop | files: `crates/victory-hle/src/modules/xsystem.rs`; with #939, #1657 |
| [#1942](https://github.com/Marvisak/VIctory/pull/1942) | GameInput: answer IUnknown and own-IID queries on the COM objects (re-lands #1641) | Calvix2k | files: `crates/victory-hle/src/modules/gameinput.rs`; with #613, #927, #1329 |
| [#1948](https://github.com/Marvisak/VIctory/pull/1948) | Shaders: derivative sampling is pixel-only at validator version 1.6 | Luislariosss | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1950](https://github.com/Marvisak/VIctory/pull/1950) | take the windows folder of test fixtures from the environment | imjustprism | files: `crates/victory-loader/tests/load_probe.rs`; with #1218, #1798 |
| [#1951](https://github.com/Marvisak/VIctory/pull/1951) | gpu: skip barriers whose before and after states are equal | claude-GTA6 | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#1953](https://github.com/Marvisak/VIctory/pull/1953) | Shaders: heap handles are reused only for the same resource properties | RaghavVijayanand | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#1954](https://github.com/Marvisak/VIctory/pull/1954) | GPU: texture to buffer copies (read back) | RaghavVijayanand | files: `crates/victory-gpu/src/list.rs`, `crates/victory-gpu/src/resource.rs`; with #24, #67, #130, #158, #450, #933, #1017, #1228 |
| [#1955](https://github.com/Marvisak/VIctory/pull/1955) | Shaders: image stores of some channels, to mip 0, and to integer views | RaghavVijayanand | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1957](https://github.com/Marvisak/VIctory/pull/1957) | GPU: blocking fence waits, feature queries, device methods, private interfaces | RaghavVijayanand | files: `crates/victory-gpu/src/device.rs`, `crates/victory-gpu/src/object.rs`; with #541, #958, #1418, #1839 |
| [#1958](https://github.com/Marvisak/VIctory/pull/1958) | Input: QueryInterface, a dispatcher, device methods, trigger rumble | RaghavVijayanand | files: `crates/victory-hle/src/modules/gameinput.rs`; with #613, #927, #1329 |
| [#1959](https://github.com/Marvisak/VIctory/pull/1959) | Shaders: LDS sub-dword writes, typed buffers at offsets and of some channels, typed loads by index | RaghavVijayanand | files: `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#1960](https://github.com/Marvisak/VIctory/pull/1960) | Shaders: wave64 compute programs on hosts of 32-lane waves | RaghavVijayanand | files: `README.md`, `crates/victory-gpu/src/host.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`; with #67, #130, #158, #166, #224, #259, #280, #284 |
| [#1963](https://github.com/Marvisak/VIctory/pull/1963) | GPU: what shaders write beside a root UAV in its pages goes back to title memory | oVolkra | files: `crates/victory-gpu/src/memory.rs`; with #1865 |
| [#1964](https://github.com/Marvisak/VIctory/pull/1964) | GPU: support 24-bit depth twins and per-plane active UAV tracking | nathanbzrr | files: `crates/victory-gpu/src/twin.rs`; with #1933 |
| [#1971](https://github.com/Marvisak/VIctory/pull/1971) | GPU and shaders: publish bounded descriptor size metadata | thatirishguyyy | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778, #1916, #1946 |
| [#1972](https://github.com/Marvisak/VIctory/pull/1972) | Diagnostics: find, reproduce and skip pipelines the host's compiler c… | J-DB0 | files: `README.md`, `apps/victory/src/translate.rs`; with #130, #158, #224, #280, #378, #491, #506, #517 |
| [#1974](https://github.com/Marvisak/VIctory/pull/1974) | translate: say which validator engine produced the numbers, and warn before 1.8 | fiestorra | files: `apps/victory/src/translate.rs`, `crates/victory-shader/src/emit/validator.rs`; with #378, #477, #517, #673, #684, #1017, #1315, #1809 |
| [#1978](https://github.com/Marvisak/VIctory/pull/1978) | GPU: no texture layouts or name copies per draw for unchanged answers | 0xkn0wh3r3 | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628, #1969 |
| [#1985](https://github.com/Marvisak/VIctory/pull/1985) | Shaders: compute exact dx.viewIdState and PSV0 input-output dependencies | nathanbzrr | files: `crates/victory-shader/src/emit/bitcode.rs`, `crates/victory-shader/src/emit/container.rs`, `crates/victory-shader/src/emit/shader.rs`; with #470, #1065, #1262, #1315, #1519, #1646, #1716, #1759 |
| [#1989](https://github.com/Marvisak/VIctory/pull/1989) | Shaders: a vertex program writing through views declares UAVs at every stage | BastienRodz | files: `crates/victory-shader/src/emit/shader.rs`; with #470, #1065, #1262, #1716, #1759 |
| [#1991](https://github.com/Marvisak/VIctory/pull/1991) | Shader tests: skip the GPU suites a device cannot run | Calvix2k | files: `crates/victory-shader/src/emit/host.rs`; with #1315, #1759, #1762, #1857, #1889, #1912, #1943, #1956 |
| [#1993](https://github.com/Marvisak/VIctory/pull/1993) | GPU: cache host root signatures and hash DXIL slices zero-copy | nathanbzrr | files: `crates/victory-gpu/src/pipeline/host.rs`; with #158, #491, #1152, #1315 |
| [#1996](https://github.com/Marvisak/VIctory/pull/1996) | Loader: handle thread spawn failure in detach_all without panicking | Androtim | files: `crates/victory-loader/src/threads.rs`; with #1304 |
| [#1997](https://github.com/Marvisak/VIctory/pull/1997) | GPU: composite multiple PresentX planes (Series S UI) | Calvix2k | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#1998](https://github.com/Marvisak/VIctory/pull/1998) | GPU: keep valid marked writes beside withheld ranges | Halfect | files: `crates/victory-gpu/src/shadow.rs`; with #602, #1692, #1743, #1979, #1986 |
| [#2002](https://github.com/Marvisak/VIctory/pull/2002) | translate: name VICTORY_DXIL in --validate's help, with why the version matters | fiestorra | files: `apps/victory/src/main.rs`; with #378, #506, #535, #1017, #1995 |
| [#2003](https://github.com/Marvisak/VIctory/pull/2003) | Shaders: MUBUF sub-dword `_d16` and `_d16_hi` loads | iurysf | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#2004](https://github.com/Marvisak/VIctory/pull/2004) | GPU: a withheld range holds back only its own bytes of a marked page (#1976) | Jitheswar | files: `crates/victory-gpu/src/shadow.rs`; with #602, #1692, #1743, #1979, #1986 |
| [#2010](https://github.com/Marvisak/VIctory/pull/2010) | Fix #726: NaN-safe v_med3_f32 lowering | tharun-37 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2012](https://github.com/Marvisak/VIctory/pull/2012) | Shaders: decode finds instructions in a table by dword, not a tree | 0xkn0wh3r3 | files: `crates/victory-shader/src/gfx10/mod.rs`; with #284 |
| [#2014](https://github.com/Marvisak/VIctory/pull/2014) | GPU: a mesh dispatch with an empty dimension runs nothing | artor410 | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#2016](https://github.com/Marvisak/VIctory/pull/2016) | Shaders: a vertex shader that declares a UAV says so in its flags and SFI0 (fixes #1987) | fiestorra | files: `crates/victory-shader/src/emit/shader.rs`; with #470, #1065, #1262, #1716, #1759 |
| [#2017](https://github.com/Marvisak/VIctory/pull/2017) | GPU: pool HostList upload buffers across resets | lizapropanol | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#2020](https://github.com/Marvisak/VIctory/pull/2020) | Shaders: abs / neg on the shift-add and align ops act on the sign bit | upipibi | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2022](https://github.com/Marvisak/VIctory/pull/2022) | xmem: an aligned XMemAllocDefault block is padded once, not twice | 0xkn0wh3r3 | files: `crates/victory-hle/src/modules/xmem.rs`; with #130, #224, #1009, #1264, #1445, #1947 |
| [#2027](https://github.com/Marvisak/VIctory/pull/2027) | Shaders: a compute shader samples at level 0, not with derivatives (735 programs) | fiestorra | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2030](https://github.com/Marvisak/VIctory/pull/2030) | Shaders: round D16 image reads according to the bound texture format | AlanQiang0418 | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/twin.rs`, `crates/victory-shader/src/emit/lower.rs`; with #130, #166, #259, #284, #379, #385, #457, #470 |
| [#2032](https://github.com/Marvisak/VIctory/pull/2032) | Preserve static view instancing in VS/PS pipeline streams | DoubleD17 | files: `crates/victory-gpu/src/pipeline/host.rs`; with #158, #491, #1152, #1315, #2031 |
| [#2034](https://github.com/Marvisak/VIctory/pull/2034) | Formats: BCPack block histories without per-call allocation or zeroing | StExpeditus | files: `crates/victory-formats/src/bcpack.rs`; with #703, #1557 |
| [#2035](https://github.com/Marvisak/VIctory/pull/2035) | Shaders: drop group sync only for divergent single-console-wave barriers on narrow-wave hosts | rivaldodev | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/lower.rs`; with #67, #130, #158, #166, #259, #284, #379, #385 |
| [#2038](https://github.com/Marvisak/VIctory/pull/2038) | Support isolated P10 reads on pixel shader inputs | DoubleD17 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2039](https://github.com/Marvisak/VIctory/pull/2039) | CLI: resolve same-directory title DLL exports in census | fuzeofficlal | files: `apps/victory/src/census.rs`; with #1863 |
| [#2040](https://github.com/Marvisak/VIctory/pull/2040) | Shaders: 64-bit built BVH pointers and unsplit node loads | mgunes1311-lab | files: `crates/victory-shader/src/emit/lower/bvh.rs`; with #1794, #1903 |
| [#2042](https://github.com/Marvisak/VIctory/pull/2042) | GPU: no blending on integer render targets in host pipelines | Deilach | files: `crates/victory-gpu/src/pipeline/host.rs`, `crates/victory-gpu/src/twin.rs`; with #158, #491, #1152, #1315, #1933, #2031 |
| [#2046](https://github.com/Marvisak/VIctory/pull/2046) | Shaders: buffer descriptor stride bits, merged loads and 64-bit selects | fuzeofficlal | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2047](https://github.com/Marvisak/VIctory/pull/2047) | GPU: guard subresource and region bounds in texture copies and MSAA resolves | fuzeofficlal | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#2048](https://github.com/Marvisak/VIctory/pull/2048) | GPU: stage HostHeap descriptor writes until the GPU is idle | fuzeofficlal | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/memory.rs`; with #67, #130, #158, #450, #1283, #1424, #1449, #1542 |
| [#2049](https://github.com/Marvisak/VIctory/pull/2049) | Shaders: seed uninitialized mesh SGPRs above user_sgprs as zero | fuzeofficlal | files: `crates/victory-shader/src/emit/mod.rs`; with #166, #457, #1017, #1112, #1263, #1315, #1582, #1712 |
| [#2050](https://github.com/Marvisak/VIctory/pull/2050) | README: a short build path on Windows, and what its failures look like | fiestorra | files: `README.md`; with #130, #158, #224, #280, #491, #506, #686, #1218 |
| [#2051](https://github.com/Marvisak/VIctory/pull/2051) | GPU: a submission's refreshes copy on several threads | 0xkn0wh3r3 | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#2054](https://github.com/Marvisak/VIctory/pull/2054) | GPU: Speed up shadow buffer writeback merging | GaussianGuaicai | files: `crates/victory-gpu/src/shadow.rs`; with #602, #1692, #1743, #1979, #1986, #2009 |
| [#2055](https://github.com/Marvisak/VIctory/pull/2055) | GPU: a shader cache miss hands back its translation, not the entry read back | 0xkn0wh3r3 | files: `crates/victory-gpu/src/shader_cache.rs`; with #130, #415, #1263, #2018 |
| [#2056](https://github.com/Marvisak/VIctory/pull/2056) | Shaders: declare 64-bit integer operations only where they are made | 0xkn0wh3r3 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2058](https://github.com/Marvisak/VIctory/pull/2058) | GPU: return hidden output pointer from GetDesc COM methods | fuzeofficlal | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628, #1969, #1984 |
| [#2059](https://github.com/Marvisak/VIctory/pull/2059) | GPU: saturate COM AddRef at u32::MAX instead of wrapping | fuzeofficlal | files: `crates/victory-gpu/src/object.rs`; with #541, #1418, #1839 |
| [#2060](https://github.com/Marvisak/VIctory/pull/2060) | Shaders: cover s_bfe_u32/s_bfe_i32 SCC, literal src1, dirty bits, and width>=32 in valu_tests | fuzeofficlal | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#2064](https://github.com/Marvisak/VIctory/pull/2064) | Shaders: d16 buffer loads of floats truncate on every host | astrosteveo | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2067](https://github.com/Marvisak/VIctory/pull/2067) | XGameUi: mask password text entry input | fuzeofficlal | files: `crates/victory-hle/src/modules/xgameui.rs`; with #1017 |
| [#2068](https://github.com/Marvisak/VIctory/pull/2068) | Shaders, GPU: lower unlisted table constant loads as raw SRVs and separate SRV/UAV buffer guards | fuzeofficlal | files: `crates/victory-gpu/src/descriptor/host.rs`; with #130, #1449, #1751, #1762, #1775, #1778, #1916, #1946 |
| [#2069](https://github.com/Marvisak/VIctory/pull/2069) | GPU: guard non-positive frame counts and test copystat | fuzeofficlal | files: `crates/victory-gpu/src/copystat.rs`; with #602, #1264, #1692 |
| [#2072](https://github.com/Marvisak/VIctory/pull/2072) | HLE: an unknown gamertag component is an invalid argument | 0xkn0wh3r3 | files: `crates/victory-hle/src/modules/xuser.rs`; with #398 |
| [#2076](https://github.com/Marvisak/VIctory/pull/2076) | HLE: implement remaining XAccessibility closed-caption, high-contrast, and speech-to-text slots | fuzeofficlal | files: `crates/victory-hle/src/modules/speech.rs`; with #1822 |
| [#2078](https://github.com/Marvisak/VIctory/pull/2078) | Shaders: keep per-lane descriptors across redundant masked moves | fuzeofficlal | files: `crates/victory-shader/src/emit/waterfall_tests.rs`; with #1253, #1437, #1441 |
| [#2081](https://github.com/Marvisak/VIctory/pull/2081) | Shaders: cover v_cmp_class_f32 VOPC SGPR/literal, mask bits, and v_cmpx_class_* EXEC narrowing | fuzeofficlal | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#2085](https://github.com/Marvisak/VIctory/pull/2085) | Shaders: pixel programs the console tests depth before keep early depth | 0xkn0wh3r3 | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/shader.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2086](https://github.com/Marvisak/VIctory/pull/2086) | HLE: implement DirectStorage CancelRequestsWithTag and Query queue slots | fuzeofficlal | files: `crates/victory-hle/src/modules/dstorage.rs`; with #130, #430, #1383, #1457, #1719, #2074 |
| [#2088](https://github.com/Marvisak/VIctory/pull/2088) | GPU: copy loose title memory in 256 KiB windows instead of 4 MiB (stacked on #402) | desplmfao | files: `crates/victory-gpu/src/shadow.rs`; with #602, #1692, #1743, #1979, #1986, #2009 |
| [#2089](https://github.com/Marvisak/VIctory/pull/2089) | hle: translate \??\ object-manager paths onto virtual drives | Pradhyumna51 | files: `crates/victory-hle/src/modules/files.rs`; with #130, #421, #1383 |
| [#2091](https://github.com/Marvisak/VIctory/pull/2091) | Shaders: preserve scalar mask words across Wave32 compute branches | rezioliver-collab | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-shader/src/emit/lower.rs`; with #130, #166, #259, #284, #379, #385, #457, #470 |
| [#2092](https://github.com/Marvisak/VIctory/pull/2092) | Shaders: lower v_dot4_u32_u8 and v_dot8_u32_u4 in pure 32-bit DXIL | geiyiliyabumo | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#2094](https://github.com/Marvisak/VIctory/pull/2094) | Shaders: lower v_add_nc_i32, v_sub_nc_i32, v_add_nc_i16, and v_sub_nc_i16 in pure 32-bit DXIL | geiyiliyabumo | files: `crates/victory-shader/src/emit/valu_tests.rs`; with #284, #379, #385, #1046, #1259, #1262, #1387, #1395 |
| [#2095](https://github.com/Marvisak/VIctory/pull/2095) | Input: trigger rumble through Windows.Gaming.Input | fylzzz | files: `crates/victory-hle/src/modules/gameinput.rs`; with #613, #927, #1329 |
| [#2096](https://github.com/Marvisak/VIctory/pull/2096) | GPU: guard shadow range arithmetic against overflow and scan all buffers in find() | fuzeofficlal | files: `crates/victory-gpu/src/shadow.rs`; with #602, #1692, #1743, #1979, #1986, #2009 |
| [#2097](https://github.com/Marvisak/VIctory/pull/2097) | Shaders: lower v_qsad_pk_u16_u8, v_mqsad_pk_u16_u8, and v_mqsad_u32_u8 in pure 32-bit DXIL | geiyiliyabumo | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2101](https://github.com/Marvisak/VIctory/pull/2101) | GPU: castable formats the host has | Deilach | files: `crates/victory-gpu/src/resource.rs`; with #67, #130, #450, #933, #1017, #1628, #1969, #1984 |
| [#2102](https://github.com/Marvisak/VIctory/pull/2102) | GPU: per-list and per-submission sets hash with FxHash, not SipHash | 0xkn0wh3r3 | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#2105](https://github.com/Marvisak/VIctory/pull/2105) | Shaders: declare wave operations from the wave intrinsics made | 0xkn0wh3r3 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2106](https://github.com/Marvisak/VIctory/pull/2106) | GPU: validate sub-16 alignments and guard oversized requests in allocate_upload | fuzeofficlal | files: `crates/victory-gpu/src/list.rs`; with #24, #67, #130, #158, #450, #1017, #1228, #1542 |
| [#2111](https://github.com/Marvisak/VIctory/pull/2111) | GPU: the frame's wait for pipelines is one budget for all recording threads | r4tedx | files: `crates/victory-gpu/src/pipeline/make.rs`; with #130, #1203, #1505, #1728 |
| [#2112](https://github.com/Marvisak/VIctory/pull/2112) | GPU: title memory in place instead of copied, opt-in (VICTORY_IN_PLACE=1; stacked on #2088, towards #880) | desplmfao | files: `crates/victory-gpu/src/shadow.rs`, `crates/victory-gpu/src/watch.rs`; with #602, #1692, #1743, #1849, #1884, #1979, #1986, #2009 |
| [#2113](https://github.com/Marvisak/VIctory/pull/2113) | Shaders: lower ds_inc_u32, ds_inc_rtn_u32, ds_dec_u32, and ds_dec_rtn_u32 in pure 32-bit DXIL | geiyiliyabumo | files: `crates/victory-shader/src/emit/semantics_tests.rs`; with #1766, #2015 |
| [#2115](https://github.com/Marvisak/VIctory/pull/2115) | CLI: translate --count counts the instructions of the emitted shaders | r4tedx | files: `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`; with #378, #506, #517, #535, #673, #1017, #1315, #1809 |
| [#2116](https://github.com/Marvisak/VIctory/pull/2116) | Shaders: lower ds_rsub_u32 and ds_rsub_rtn_u32 in pure 32-bit DXIL | geiyiliyabumo | files: `crates/victory-shader/src/emit/semantics_tests.rs`; with #1766, #2015 |
| [#2118](https://github.com/Marvisak/VIctory/pull/2118) | GPU: the BVH builder runs whole or not at all | 0xkn0wh3r3 | files: `apps/victory/src/run.rs`; with #130, #224, #1009, #1017, #1798, #2071 |
| [#2119](https://github.com/Marvisak/VIctory/pull/2119) | Shaders: lower ds_wrxchg2_rtn_b32 and ds_wrxchg2st64_rtn_b32 in pure 32-bit DXIL | geiyiliyabumo | files: `crates/victory-shader/src/emit/semantics_tests.rs`; with #1766, #2015, #2117 |
| [#2121](https://github.com/Marvisak/VIctory/pull/2121) | GPU: page view heaps a host caps below the slots shaders index | 0xkn0wh3r3 | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/shader.rs`, `crates/victory-shader/src/emit/stride_tests.rs`; with #130, #166, #415, #457, #470, #1017, #1065, #1112 |
| [#2122](https://github.com/Marvisak/VIctory/pull/2122) | query a buffer's size through its own handle when the size is read | imjustprism | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2125](https://github.com/Marvisak/VIctory/pull/2125) | Shaders: load D16 integer textures with their bound component type | AlanQiang0418 | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/twin.rs`, `crates/victory-shader/src/emit/lower.rs`; with #130, #166, #259, #284, #379, #385, #457, #470 |
| [#2126](https://github.com/Marvisak/VIctory/pull/2126) | Shaders: compute threads that read other lanes keep unique ids in 2-D groups (NVIDIA) | 0xkn0wh3r3 | files: `crates/victory-shader/src/emit/lower.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2128](https://github.com/Marvisak/VIctory/pull/2128) | GPU: populate unbound table slots with typed null descriptors | fuzeofficlal | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-shader/src/emit/lower.rs`; with #130, #158, #166, #259, #284, #379, #385, #450 |
| [#2129](https://github.com/Marvisak/VIctory/pull/2129) | GPU: map the pipeline library's file instead of reading it into memory | 0xkn0wh3r3 | files: `crates/victory-gpu/src/pipeline/library.rs`; with #983, #1722, #1914, #2007 |
| [#2133](https://github.com/Marvisak/VIctory/pull/2133) | Shaders: lower 64-bit shifts without INT64 | scoresby13 | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`; with #166, #259, #284, #379, #385, #457, #470, #589 |
| [#2136](https://github.com/Marvisak/VIctory/pull/2136) | GPU: a host copy nothing wrote is not asked for at submission | r4tedx | files: `crates/victory-gpu/src/host.rs`; with #67, #130, #158, #450, #1283, #1424, #1542, #1555 |
| [#2137](https://github.com/Marvisak/VIctory/pull/2137) | HLE: implement XPackage monitor update, user locale, and mount path size | fuzeofficlal | files: `crates/victory-hle/src/modules/xpackage.rs`; with #522, #1827, #1928, #1930 |
| [#2142](https://github.com/Marvisak/VIctory/pull/2142) | GPU: read small buffer diagnostics via read_into instead of VirtualQuery | fuzeofficlal | files: `crates/victory-gpu/src/draw.rs`; with #130, #158, #450, #1233, #1449, #1542, #1700, #1762 |

## conflicts with main (280)

| PR | Title | Author | Detail |
|---|---|---|---|
| [#32](https://github.com/Marvisak/VIctory/pull/32) | GPU: fence waits, direct memory-write wakes, queue timeline and host list pool | AndyLatimer | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/list.rs` |
| [#55](https://github.com/Marvisak/VIctory/pull/55) | GPU: create host textures without the uses the host refuses | a55uka | files: `crates/victory-gpu/src/resource.rs` |
| [#72](https://github.com/Marvisak/VIctory/pull/72) | GPU: retry pipeline cache saves after write failures | FIocker | files: `crates/victory-gpu/src/pipeline/library.rs` |
| [#76](https://github.com/Marvisak/VIctory/pull/76) | xmem: map a run of physical pages over several placeholders | SuryaK5125 | files: `crates/victory-hle/src/modules/xmem.rs` |
| [#78](https://github.com/Marvisak/VIctory/pull/78) | Shaders: inline float constants as f16 in v_fma_mix and v_pack_b32_f16 | tedin7 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#82](https://github.com/Marvisak/VIctory/pull/82) | HLE: speak the title's narration with the host's speech synthesizer | permadiego | files: `crates/victory-hle/Cargo.toml` |
| [#83](https://github.com/Marvisak/VIctory/pull/83) | GPU: bound the null sampler staging heap | FIocker | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#89](https://github.com/Marvisak/VIctory/pull/89) | gpu: packed 16-bit render targets are R8G8B8A8 where the GPU cannot render to them | HenriqueSBueno | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/list.rs`, `crates/victory-gpu/src/texture.rs` |
| [#91](https://github.com/Marvisak/VIctory/pull/91) | Shaders: lower s_cmov_b32, s_cmov_b64 and s_cmovk_i32 | tedin7 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#92](https://github.com/Marvisak/VIctory/pull/92) | Shaders: lower v_add_nc_i32, v_sub_nc_i32, v_mul_lo_i32 and the signed 16-bit add and sub | tedin7 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#98](https://github.com/Marvisak/VIctory/pull/98) | Shaders: model v_permlane16_b32 in whole-wave sections | tedin7 | files: `crates/victory-shader/src/emit/wave.rs` |
| [#112](https://github.com/Marvisak/VIctory/pull/112) | Build: Linux workspace, contributor setup and CI | Quat3rnion | files: `apps/victory/src/translate.rs` |
| [#114](https://github.com/Marvisak/VIctory/pull/114) | XAsync: run a failed call's cleanup without the completion lock | AndyLatimer | files: `crates/victory-hle/src/modules/xasync.rs` |
| [#120](https://github.com/Marvisak/VIctory/pull/120) | Shaders: lower s_absdiff_i32, s_sext_i32_i8/i16 and s_pack_ll/lh/hh_b32_b16 | tedin7 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#123](https://github.com/Marvisak/VIctory/pull/123) | GPU: refuse raw buffer views that do not start on 16 bytes | 9hvn9yt2vk-netizen | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#145](https://github.com/Marvisak/VIctory/pull/145) | Shaders: read both ballot halves for wave32 host-wave mask ops | 7ROBE | files: `crates/victory-shader/src/emit/lower.rs` |
| [#148](https://github.com/Marvisak/VIctory/pull/148) | Shaders: lower s_bcnt0_i32_b32/_b64, s_ff0_i32_b32/_b64 and s_flbit_i32_b64/_i64 | Rareuncommon | files: `crates/victory-shader/src/emit/lower.rs` |
| [#150](https://github.com/Marvisak/VIctory/pull/150) | Input: read DualSense pads over HID when XInput has none | Kam471 | files: `README.md`, `crates/victory-hle/src/modules/mod.rs` |
| [#157](https://github.com/Marvisak/VIctory/pull/157) | Shaders: lower 24-bit mul_hi, saturating 16-bit packs, integer dot pr… | 9hvn9yt2vk-netizen | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#168](https://github.com/Marvisak/VIctory/pull/168) | Shaders: SCC of s_min, s_max and s_lshl1-4_add_u32 as the ISA sets it | Dufftwt | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#177](https://github.com/Marvisak/VIctory/pull/177) | Shaders: key heap handles by element type (atomic on a float-read UAV) | Ortomanic | files: `crates/victory-shader/src/emit/lower.rs` |
| [#182](https://github.com/Marvisak/VIctory/pull/182) | Files: `..` in a title path stops at the virtual drive's root | Maraco91 | files: `crates/victory-hle/src/modules/files.rs` |
| [#189](https://github.com/Marvisak/VIctory/pull/189) | Shaders: emulate Wave64 compute shaders on Wave32 and narrower hosts | alfycs | files: `apps/victory/src/translate.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/bvh_tests.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs` |
| [#192](https://github.com/Marvisak/VIctory/pull/192) | Shaders: replace single-wave Wave64 LDS lockstep with barriers and lower 32-lane swizzle/DPP at WaveSize(32) | kxiserino | files: `crates/victory-shader/src/emit/lower.rs` |
| [#202](https://github.com/Marvisak/VIctory/pull/202) | GPU: skip unmapped regions after a failed bulk read into host memory | ifBars | files: `crates/victory-gpu/src/host.rs` |
| [#216](https://github.com/Marvisak/VIctory/pull/216) | Loader & HLE: harden VEH crash handler, PE prep/exports/TLS, and allow oleaut32 | wbkepler | files: `crates/victory-loader/src/crash.rs` |
| [#222](https://github.com/Marvisak/VIctory/pull/222) | apu: warn when the host has no audio output device | a55uka | files: `README.md` |
| [#223](https://github.com/Marvisak/VIctory/pull/223) | Shaders: translate for the host's wave sizes | r4tedx | files: `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/shader_cache.rs` |
| [#230](https://github.com/Marvisak/VIctory/pull/230) | GPU: skip unchanged texture uploads via prelude copy recording | wbkepler | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/resource.rs`, `crates/victory-gpu/src/texture.rs` |
| [#233](https://github.com/Marvisak/VIctory/pull/233) | Shaders: wave64 compute on 32-lane hosts, in halves and in the console flow | r4tedx | files: `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/shader_cache.rs` |
| [#234](https://github.com/Marvisak/VIctory/pull/234) | HLE: overlapped DirectStorage I/O and persistent xmem section views | wbkepler | files: `crates/victory-hle/src/modules/dstorage.rs` |
| [#236](https://github.com/Marvisak/VIctory/pull/236) | XAsync: a canceled pending call's work touches neither freed state nor the next call | Dufftwt | files: `crates/victory-hle/src/modules/xasync.rs` |
| [#237](https://github.com/Marvisak/VIctory/pull/237) | HLE: offload async saves, reuse audio buffers, pin APU heap, and decouple input rumble/dialogs | wbkepler | files: `crates/victory-hle/src/modules/apu.rs`, `crates/victory-hle/src/modules/audio.rs`, `crates/victory-hle/src/modules/gameinput.rs`, `crates/victory-hle/src/modules/xasync.rs`, `crates/victory-hle/src/modules/xgamesave.rs` |
| [#242](https://github.com/Marvisak/VIctory/pull/242) | GPU: make ExecuteIndirect clamping countBuffer-aware and non-destructive | wbkepler | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/indirect.rs` |
| [#244](https://github.com/Marvisak/VIctory/pull/244) | Shaders: share and serialize WARP host operations across threads | AndyLatimer | files: `crates/victory-shader/src/emit/host.rs` |
| [#250](https://github.com/Marvisak/VIctory/pull/250) | GPU & CLI: configurable/uncapped FPS limit, high-res pacing, high-refresh modes, and tearing swapchain | wbkepler | files: `apps/victory/src/main.rs`, `apps/victory/src/run.rs`, `crates/victory-gpu/src/dxgi.rs`, `crates/victory-gpu/src/frame.rs`, `crates/victory-gpu/src/host.rs` |
| [#261](https://github.com/Marvisak/VIctory/pull/261) | GPU: copy-back puts back only what shaders wrote, in two more cases | Maraco91 | files: `crates/victory-gpu/src/shadow.rs` |
| [#269](https://github.com/Marvisak/VIctory/pull/269) | Shaders: scalars leave divergent loops and branches as the wave's last path left them | TheHoracio | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#275](https://github.com/Marvisak/VIctory/pull/275) | GPU: atomic adaptive PSO wait budget and non-blocking pipeline library saves | wbkepler | files: `crates/victory-gpu/src/pipeline/library.rs`, `crates/victory-gpu/src/pipeline/make.rs` |
| [#278](https://github.com/Marvisak/VIctory/pull/278) | GPU: answer format-info and virtual-address feature queries | Quat3rnion | files: `crates/victory-gpu/src/device.rs` |
| [#279](https://github.com/Marvisak/VIctory/pull/279) | GPU: name the commands at a DRED stop, and dump translated shaders | ALFREDOHDEZ | files: `README.md`, `crates/victory-gpu/src/pipeline.rs` |
| [#290](https://github.com/Marvisak/VIctory/pull/290) | HLE tests: cover Windows path rules on every host | Quat3rnion | files: `crates/victory-hle/src/modules/mod.rs` |
| [#292](https://github.com/Marvisak/VIctory/pull/292) | Shader: lower WQM QuadAny, clamp LDS accesses, and guard loop budgets & barriers | wbkepler | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs`, `crates/victory-shader/src/structure.rs` |
| [#301](https://github.com/Marvisak/VIctory/pull/301) | GPU: non-stream graphics PSOs, stream subobjects 25..=28, texture-to-buffer readbacks, and sRGB view preservation | wbkepler | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/pipeline/host.rs` |
| [#308](https://github.com/Marvisak/VIctory/pull/308) | Shaders: follow lane writes and scalar steps in whole-wave sections | AntoninMichelq | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/wave.rs` |
| [#310](https://github.com/Marvisak/VIctory/pull/310) | Policy: resolve cache paths inside Windows AppContainer | melon0918 | files: `crates/victory-core/src/policy.rs` |
| [#312](https://github.com/Marvisak/VIctory/pull/312) | HLE: SDL3 gamepad backend for the gameinput shim | DaviTostes | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#320](https://github.com/Marvisak/VIctory/pull/320) | Shaders: lane reads and writes where lanes branch differently run in the console flow | r4tedx | files: `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/shader_cache.rs` |
| [#321](https://github.com/Marvisak/VIctory/pull/321) | GPU: honour scheduled frame-event offsets | Quat3rnion | files: `crates/victory-gpu/src/frame.rs` |
| [#329](https://github.com/Marvisak/VIctory/pull/329) | Shaders: recover record counts for descriptor-only structured buffers | Raxstreams | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#334](https://github.com/Marvisak/VIctory/pull/334) | GPU: transition host copies of title memory between shader writes and reads | realhenners | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/indirect.rs` |
| [#342](https://github.com/Marvisak/VIctory/pull/342) | Shaders: lower partial-channel typed buffer stores | tarcisioads | files: `crates/victory-shader/src/emit/lower.rs` |
| [#347](https://github.com/Marvisak/VIctory/pull/347) | GPU: ask for an empty pipeline library with a null blob | AugustoZFJ | files: `crates/victory-gpu/src/pipeline/library.rs` |
| [#374](https://github.com/Marvisak/VIctory/pull/374) | XAsync: a wait on a failed call during its cleanup waits for the status | marquezinii | files: `crates/victory-hle/src/modules/xasync.rs` |
| [#388](https://github.com/Marvisak/VIctory/pull/388) | GPU: refresh unwatched textures without a region query; detile into cached memory | ItsYusei99 | files: `crates/victory-gpu/src/texture.rs` |
| [#402](https://github.com/Marvisak/VIctory/pull/402) | GPU: find a buffer holding an address past many smaller ones; reuse loose windows that hold a range | kxiserino | files: `crates/victory-gpu/src/shadow.rs` |
| [#409](https://github.com/Marvisak/VIctory/pull/409) | GPU: match D3D12 copyable texture footprints | ToxicmaggotRP | files: `crates/victory-gpu/src/resource.rs` |
| [#411](https://github.com/Marvisak/VIctory/pull/411) | GPU: require only the descriptor heaps used by the pipeline | FIocker | files: `crates/victory-gpu/src/pipeline/host.rs`, `crates/victory-shader/src/emit/shader.rs` |
| [#412](https://github.com/Marvisak/VIctory/pull/412) | HLE: validate XPackage chunk selectors | enzoduarte1 | files: `crates/victory-hle/src/modules/xpackage.rs` |
| [#416](https://github.com/Marvisak/VIctory/pull/416) | shader: implement v_cvt_f32_f64 instruction lowering | bitzin-dev | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#420](https://github.com/Marvisak/VIctory/pull/420) | GPU: include root-selected entries in rebuilt descriptor tables | FIocker | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#425](https://github.com/Marvisak/VIctory/pull/425) | Game: validate DXBC containers in SGD5 records | enzoduarte1 | files: `crates/victory-game/src/sgd5.rs` |
| [#427](https://github.com/Marvisak/VIctory/pull/427) | GPU: say in device-removed reports which user data is a table | GamingRipperDoc | files: `crates/victory-gpu/src/draw.rs` |
| [#428](https://github.com/Marvisak/VIctory/pull/428) | GameInput: next and previous readings of a disconnected device fail | marquezinii | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#434](https://github.com/Marvisak/VIctory/pull/434) | Shaders: a wave32 program's mask read picks its half of a 64-lane ballot | NolanCotter | files: `crates/victory-shader/src/emit/lower.rs` |
| [#435](https://github.com/Marvisak/VIctory/pull/435) | GPU: coalesce identical write declarations within a submission | melon0918 | files: `crates/victory-gpu/src/host.rs` |
| [#439](https://github.com/Marvisak/VIctory/pull/439) | Shaders: one console wave sharing LDS runs in the console flow on 32-lane hosts | r4tedx | files: `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/shader_cache.rs` |
| [#441](https://github.com/Marvisak/VIctory/pull/441) | run: let victory.toml pick the console | arminkhaleghirod | files: `apps/victory/src/main.rs`, `apps/victory/src/run.rs`, `crates/victory-core/src/config.rs` |
| [#443](https://github.com/Marvisak/VIctory/pull/443) | Audio: keep XMA frames split across rolling inputs | Quat3rnion | files: `crates/victory-formats/src/xma.rs`, `crates/victory-hle/src/modules/audio.rs` |
| [#452](https://github.com/Marvisak/VIctory/pull/452) | Shaders: spin-wait loops stop after 4096 iterations; loop limit in release builds | AntoninMichelq | files: `README.md`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs` |
| [#453](https://github.com/Marvisak/VIctory/pull/453) | Shaders: pixel barycentrics through SV_Barycentrics (values and computed interpolation) | AntoninMichelq | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/io.rs` |
| [#458](https://github.com/Marvisak/VIctory/pull/458) | Fix GPU hangs, memory leak, and mesh shader exports; render intro cre… | techselect | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/indirect.rs`, `crates/victory-gpu/src/resource.rs`, `crates/victory-shader/src/emit/lower.rs` |
| [#463](https://github.com/Marvisak/VIctory/pull/463) | GPU: load an app-local D3D12 Agility SDK runtime when present | Shaun-Fong | files: `README.md` |
| [#472](https://github.com/Marvisak/VIctory/pull/472) | GPU: grow the write snapshot in place, and cap idle pool memory | SaymonMaster14 | files: `crates/victory-gpu/src/shadow.rs`, `crates/victory-gpu/src/texture.rs` |
| [#482](https://github.com/Marvisak/VIctory/pull/482) | App: reject wrapping RVAs in VICTORY_WATCH | enzoduarte1 | files: `apps/victory/src/run.rs` |
| [#498](https://github.com/Marvisak/VIctory/pull/498) | shader: lower v_movrelsd_2_b32 and v_swaprel_b32 | NolanCotter | files: `crates/victory-shader/src/emit/lower.rs` |
| [#499](https://github.com/Marvisak/VIctory/pull/499) | shader: lower v_sat_pk_u8_i16 | NolanCotter | files: `crates/victory-shader/src/emit/lower.rs` |
| [#500](https://github.com/Marvisak/VIctory/pull/500) | GPU diagnostics: pick capture frames by draw count | itsRevela | files: `crates/victory-gpu/src/capture.rs` |
| [#503](https://github.com/Marvisak/VIctory/pull/503) | HLE: pin a call's task queue at XAsyncBegin | Quat3rnion | files: `crates/victory-hle/src/modules/xasync.rs` |
| [#523](https://github.com/Marvisak/VIctory/pull/523) | Build: compile and test the workspace on Linux again | spidercraft | files: `apps/victory/src/translate.rs`, `crates/victory-hle/src/modules/xgame.rs` |
| [#530](https://github.com/Marvisak/VIctory/pull/530) | GPU diagnostics: show one of the title's render targets in the window | AKA0GHOST | files: `README.md`, `crates/victory-gpu/src/capture.rs` |
| [#537](https://github.com/Marvisak/VIctory/pull/537) | GPU: name the switch when a removal has no device-removed data | coderanonymus | files: `README.md` |
| [#542](https://github.com/Marvisak/VIctory/pull/542) | Shaders: keep buffer offsets below 2 GiB (NVIDIA + vkd3d-proton world device loss) | gingertonik | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#544](https://github.com/Marvisak/VIctory/pull/544) | HLE: report queue termination after running callbacks | Quat3rnion | files: `crates/victory-hle/src/modules/xasync.rs` |
| [#551](https://github.com/Marvisak/VIctory/pull/551) | Shaders: lower compute group info, wave id in workgroup, and sample coverage | kilvynsilveira-tech | files: `crates/victory-shader/src/emit/lower.rs` |
| [#561](https://github.com/Marvisak/VIctory/pull/561) | xmem: commit physical pages in pieces, and report the real count | JoshRob297 | files: `crates/victory-hle/src/modules/xmem.rs` |
| [#574](https://github.com/Marvisak/VIctory/pull/574) | HLE: write at-rest gamepad state for non-pad readings | Quat3rnion | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#575](https://github.com/Marvisak/VIctory/pull/575) | GPU: release resources the CPU is done with behind the queue | Len1k | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#578](https://github.com/Marvisak/VIctory/pull/578) | GPU: refuse wave64 mesh and compute programs on hosts without 64-lane waves | wbkepler | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/pipeline/host.rs` |
| [#579](https://github.com/Marvisak/VIctory/pull/579) | GPU: VICTORY_SKIP_WAVE64 diagnostics switch for wave64 programs | wbkepler | files: `README.md`, `crates/victory-gpu/src/pipeline.rs` |
| [#583](https://github.com/Marvisak/VIctory/pull/583) | fix(gpu): respect indirect count buffer and bind null root views | matheusfrta | files: `crates/victory-gpu/src/shadow.rs` |
| [#586](https://github.com/Marvisak/VIctory/pull/586) | Shaders: honor the mesh allocation request counts | Kam471 | files: `README.md`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs` |
| [#587](https://github.com/Marvisak/VIctory/pull/587) | Shaders: guard mesh output bounds, init all primitives, and omit sync on divergent s_barrier | FibbS6 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#595](https://github.com/Marvisak/VIctory/pull/595) | GPU: run submissions asynchronously on a host fence timeline | wbkepler | files: `crates/victory-gpu/src/descriptor.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#596](https://github.com/Marvisak/VIctory/pull/596) | Shaders: lower v_cvt_pknorm_{i16,u16}_{f32,f16} with exact rounding | mercuryVM | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#599](https://github.com/Marvisak/VIctory/pull/599) | Shaders: lane-indexed ops on a wide host wave use the own console wave | NolanCotter | files: `crates/victory-shader/src/emit/lower.rs` |
| [#609](https://github.com/Marvisak/VIctory/pull/609) | GPU: retain and retire released host textures, staging, shadow buffers, and descriptor resources across submissions | LAKSHYA-cmd-tech | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/resource.rs`, `crates/victory-gpu/src/texture.rs` |
| [#616](https://github.com/Marvisak/VIctory/pull/616) | GPU: rectangle draws check their bounds without 32-bit overflow | itzzArshad | files: `crates/victory-gpu/src/draw.rs` |
| [#628](https://github.com/Marvisak/VIctory/pull/628) | Shaders: lower ds_min_f32 / ds_max_f32 (and _rtn) with RDNA2's NaN rules | mercuryVM | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#633](https://github.com/Marvisak/VIctory/pull/633) | HLE: GetModuleFileNameA answers in the ANSI code page | JFIREonFIRE | files: `crates/victory-hle/src/modules/kernel.rs` |
| [#651](https://github.com/Marvisak/VIctory/pull/651) | GPU: select the readable format and plane for depth/stencil shader views | Raxstreams | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#656](https://github.com/Marvisak/VIctory/pull/656) | Input: report the character each held key types | lameloball2813-ui | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#659](https://github.com/Marvisak/VIctory/pull/659) | Shaders: refuse per-lane reads of mesh v5 in v5 writes and after joins | coderanonymus | files: `crates/victory-shader/src/ir.rs` |
| [#663](https://github.com/Marvisak/VIctory/pull/663) | media: apply the local-file rule to the source reader, sink writer and MFCreateFile | coderanonymus | files: `crates/victory-hle/src/modules/media.rs` |
| [#679](https://github.com/Marvisak/VIctory/pull/679) | run: refuse capture and dump paths inside the repo | binarycleric | files: `apps/victory/src/run.rs` |
| [#682](https://github.com/Marvisak/VIctory/pull/682) | Shaders: v_mul_legacy_f32 treats denormal operands as zero | senchsus | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#687](https://github.com/Marvisak/VIctory/pull/687) | hle: track title working-set memory, bound xasync threads, and map XUser devices | wbkepler | files: `crates/victory-hle/src/modules/xasync.rs`, `crates/victory-hle/src/modules/xmem.rs` |
| [#690](https://github.com/Marvisak/VIctory/pull/690) | gpu: expand console primitive topologies into host list index buffers | wbkepler | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/pipeline/host.rs`, `crates/victory-gpu/src/primitive.rs` |
| [#693](https://github.com/Marvisak/VIctory/pull/693) | gpu: move shadow buffers to DEFAULT VRAM with pooled staging transfers | wbkepler | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/shadow.rs`, `crates/victory-gpu/src/watch.rs` |
| [#695](https://github.com/Marvisak/VIctory/pull/695) | gpu: VICTORY_D3D12_WARP=0 opts out of the software adapter | arthercz | files: `README.md`, `crates/victory-formats/src/xma.rs`, `crates/victory-gpu/src/capture.rs`, `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs` |
| [#697](https://github.com/Marvisak/VIctory/pull/697) | Shader: backward dead-code elimination in DXIL bitcode emission | wbkepler | files: `crates/victory-shader/src/emit/bitcode.rs` |
| [#707](https://github.com/Marvisak/VIctory/pull/707) | Shaders: lower tbuffer_store_format_* that need no conversion | mercuryVM | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#943](https://github.com/Marvisak/VIctory/pull/943) | GPU: use VirtualQuery region fallback in copy_mapped and write_mapped | MartinIndra02 | files: `crates/victory-gpu/src/host.rs` |
| [#946](https://github.com/Marvisak/VIctory/pull/946) | victory-shader: preserve helper-lane values on pixel waveReadLaneFirst | Sakura-Byte | files: `crates/victory-shader/src/emit/lower.rs` |
| [#952](https://github.com/Marvisak/VIctory/pull/952) | GPU: log the host's video-memory use and its budget | Ege-Mert | files: `crates/victory-gpu/src/host.rs` |
| [#956](https://github.com/Marvisak/VIctory/pull/956) | GPU: support multi-argument ExecuteIndirect signatures with root constants | MartinIndra02 | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/indirect.rs`, `crates/victory-gpu/src/pipeline/host.rs` |
| [#959](https://github.com/Marvisak/VIctory/pull/959) | victory: a menu when opened from Explorer, instead of help that closes at once | EduBertozzi | files: `Cargo.lock`, `apps/victory/src/main.rs` |
| [#960](https://github.com/Marvisak/VIctory/pull/960) | Game picker | killar4eto | files: `Cargo.lock`, `README.md`, `apps/victory/src/main.rs` |
| [#961](https://github.com/Marvisak/VIctory/pull/961) | Shaders, GPU: draw mesh pipelines without mesh shaders through a compute fallback (#3) | EduBertozzi | files: `apps/victory/src/main.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/pipeline/host.rs` |
| [#967](https://github.com/Marvisak/VIctory/pull/967) | GPU: choose an adapter that can run the title, and name the ones that cannot | zzm1234560621-prog | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/lib.rs` |
| [#982](https://github.com/Marvisak/VIctory/pull/982) | GPU: pool upload buffers across command list resets | chemokita13 | files: `crates/victory-gpu/src/host.rs` |
| [#984](https://github.com/Marvisak/VIctory/pull/984) | Shaders: split flat byte-offset accesses to structured buffers | CitizenCinema | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#988](https://github.com/Marvisak/VIctory/pull/988) | GPU: say when a fault lies past the end of its title memory copy | h0useadv | files: `crates/victory-gpu/src/shadow.rs` |
| [#999](https://github.com/Marvisak/VIctory/pull/999) | Shaders: search lane conditions once and redo a lowering once | TG0000 | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/structure.rs` |
| [#1002](https://github.com/Marvisak/VIctory/pull/1002) | victory-shader: hash the lowering tables with Fx instead of SipHash (lowering -16%) | pes7 | files: `crates/victory-shader/src/structure.rs` |
| [#1005](https://github.com/Marvisak/VIctory/pull/1005) | Fix 12 verified regressions in shader translation, runtime, caching and texture handling | THE-PRO-GAME | files: `apps/victory/src/translate.rs`, `crates/victory-formats/src/texture.rs`, `crates/victory-hle/src/modules/display.rs`, `crates/victory-hle/src/modules/xgamesave.rs`, `crates/victory-pipeline/src/cache.rs` |
| [#1010](https://github.com/Marvisak/VIctory/pull/1010) | GPU: copy title memory into host memory without a system call | TG0000 | files: `crates/victory-gpu/src/host.rs` |
| [#1012](https://github.com/Marvisak/VIctory/pull/1012) | Shaders: descriptors joined by nested control flow | ThiagoAlexsander | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1014](https://github.com/Marvisak/VIctory/pull/1014) | victory-shader: lower v_permlane16_b32 and v_permlanex16_b32 | bartikele | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1023](https://github.com/Marvisak/VIctory/pull/1023) | gpu: implement ordered 32-bit and 64-bit command-list memory waits | KLeek5 | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/list.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#1024](https://github.com/Marvisak/VIctory/pull/1024) | GPU: copy-back writes only the GPU's bytes and marks only unchanged pages current | Dufftwt | files: `crates/victory-gpu/src/shadow.rs`, `crates/victory-gpu/src/watch.rs` |
| [#1032](https://github.com/Marvisak/VIctory/pull/1032) | GPU: cap oversized sampler heaps | divyeshganta | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#1055](https://github.com/Marvisak/VIctory/pull/1055) | Shaders: scalar buffer loads from a UAV read it raw | adamdotpy | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1056](https://github.com/Marvisak/VIctory/pull/1056) | Shaders: scalar loads from a root view's address read its memory | adamdotpy | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1076](https://github.com/Marvisak/VIctory/pull/1076) | Fix NVIDIA Ada Lovelace (`RTX 4060`) D3D12 compatibility, 2s TDR submissions, and 16 GB host OOM | legayclightning22 | files: `crates/victory-gpu/src/capture.rs`, `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline/make.rs` |
| [#1083](https://github.com/Marvisak/VIctory/pull/1083) | Shaders: name what a scalar memory read is, and which descriptor dword it reads | adamdotpy | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1084](https://github.com/Marvisak/VIctory/pull/1084) | Shaders: a select or carry-in on an untracked mask tests this lane's bit | undeadhuman37 | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1088](https://github.com/Marvisak/VIctory/pull/1088) | Fix compute shader signed overflow and paired LDS writes | THE-PRO-GAME | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1095](https://github.com/Marvisak/VIctory/pull/1095) | Shaders: compute signed overflow SCC for s_add_i32, s_addk_i32, and s_sub_i32 | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1106](https://github.com/Marvisak/VIctory/pull/1106) | test(shader): report forced WARP runs as software only | divyeshganta | files: `crates/victory-shader/src/emit/valu_tests.rs`, `crates/victory-shader/src/emit/waterfall_tests.rs` |
| [#1124](https://github.com/Marvisak/VIctory/pull/1124) | Shadows: batch GPU copy-back writes safely | divyeshganta | files: `crates/victory-gpu/src/shadow.rs` |
| [#1126](https://github.com/Marvisak/VIctory/pull/1126) | GPU: release descriptor tables after a rejected bind | EatCodeSleep2 | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/pipeline.rs` |
| [#1129](https://github.com/Marvisak/VIctory/pull/1129) | GPU: give pipeline workers room for NVIDIA's shader compiler | divyeshganta | files: `crates/victory-gpu/src/pipeline/make.rs` |
| [#1135](https://github.com/Marvisak/VIctory/pull/1135) | GPU: detile only texture subresources affected by writes | divyeshganta | files: `crates/victory-gpu/src/texture.rs` |
| [#1138](https://github.com/Marvisak/VIctory/pull/1138) | Shader tests: failures name the adapter that ran them | ayanbarosan | files: `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/waterfall_tests.rs` |
| [#1142](https://github.com/Marvisak/VIctory/pull/1142) | Shaders: support saturating clamp on packed 16-bit integer ops (v_pk_{add,sub,mad}_{u16,i16}) | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1143](https://github.com/Marvisak/VIctory/pull/1143) | GPU tests: verify a synthetic frame through PresentX | Doffi4 | files: `crates/victory-gpu/src/host.rs` |
| [#1144](https://github.com/Marvisak/VIctory/pull/1144) | Tests: show when shader differential checks are skipped | EatCodeSleep2 | files: `crates/victory-shader/src/emit/diff_tests.rs` |
| [#1145](https://github.com/Marvisak/VIctory/pull/1145) | Shaders: fix duplicate latch folding, unreachable phi edges, and replace_const caching in DXIL bitcode | hamdog0203 | files: `crates/victory-shader/src/emit/bitcode.rs` |
| [#1148](https://github.com/Marvisak/VIctory/pull/1148) | Shaders: lower image_atomic_inc, _dec, _fcmpswap, _fmin and _fmax | VermaakLabs | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1156](https://github.com/Marvisak/VIctory/pull/1156) | Loader: continue past a title's int3 asserts, for finding the next blocker | adamdotpy | files: `crates/victory-loader/src/crash.rs` |
| [#1157](https://github.com/Marvisak/VIctory/pull/1157) | Input: the mouse as the stand-in pad's camera (VICTORY_MOUSE) | h0useadv | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#1190](https://github.com/Marvisak/VIctory/pull/1190) | Shaders: lower v_frexp_mant_{f32,f16} and v_frexp_exp_i{32_f32,16_f16} | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1210](https://github.com/Marvisak/VIctory/pull/1210) | GPU: upload a host-copied texture again once the title writes its memory | powplusplus | files: `crates/victory-gpu/src/texture.rs` |
| [#1211](https://github.com/Marvisak/VIctory/pull/1211) | Shaders: lower v_mad_{u16,i16} and saturating clamp on 16-bit integer ops | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1215](https://github.com/Marvisak/VIctory/pull/1215) | GPU: copy back with NtWriteVirtualMemory, no VirtualQuery before a copy into the title (copy back 1.5 s -> 0.13 s) | LooperSalty | files: `crates/victory-gpu/Cargo.toml`, `crates/victory-gpu/src/host.rs` |
| [#1217](https://github.com/Marvisak/VIctory/pull/1217) | GPU: the copies timing no longer counts the flush before a title copy twice | neonbyte1 | files: `crates/victory-gpu/src/host.rs` |
| [#1223](https://github.com/Marvisak/VIctory/pull/1223) | Shaders: lower v_ldexp_f16 (VOP2 0x3b and VOP3 0x13b) | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1227](https://github.com/Marvisak/VIctory/pull/1227) | Improve shader mask handling and GPU diagnostics | chiragmadhuprasad | files: `crates/victory-gpu/src/copystat.rs`, `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#1237](https://github.com/Marvisak/VIctory/pull/1237) | Shaders: implicit-level samples in compute take their gradients from the quad | felicienradio | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1243](https://github.com/Marvisak/VIctory/pull/1243) | HLE: the module tree gates for a host that is not Windows | MIchiel1234 | files: `crates/victory-hle/src/modules/xgame.rs` |
| [#1254](https://github.com/Marvisak/VIctory/pull/1254) | GPU diagnostics: a black frame's trace keeps the draws that end the frame | EduAzevedo | files: `crates/victory-gpu/src/draw.rs` |
| [#1261](https://github.com/Marvisak/VIctory/pull/1261) | Shaders: an SGPR no instruction writes stays refused above the initialized ones | Proudnoob4 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1267](https://github.com/Marvisak/VIctory/pull/1267) | GPU: keep only the bytes shaders may write, each once per submission | TG0000 | files: `crates/victory-gpu/src/shadow.rs` |
| [#1272](https://github.com/Marvisak/VIctory/pull/1272) | GPU: automatic texture mip-drop on E_OUTOFMEMORY and VICTORY_TEXTURE_MIP_DROP | ESAT586 | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#1275](https://github.com/Marvisak/VIctory/pull/1275) | Shaders: apply v_mov_b32 and v_cndmask_b32 abs/neg modifiers bitwise | Sakura-Byte | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1279](https://github.com/Marvisak/VIctory/pull/1279) | GPU: an argument clamp Wine's compiler builds too | Mumogg | files: `crates/victory-gpu/src/indirect.rs` |
| [#1285](https://github.com/Marvisak/VIctory/pull/1285) | Shaders: fix 16-bit VALU SDWA placement/sext and inline float constants | hamdog0203 | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1289](https://github.com/Marvisak/VIctory/pull/1289) | Shader tests: v_pk_mul_lo_u16 keeps each half to 16 bits | bvtterbroot | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1292](https://github.com/Marvisak/VIctory/pull/1292) | Shaders: lower v_dot2_{i32_i16,u32_u16}, v_dot4_{i32_i8,u32_u8}, v_dot4c_i32_i8, and v_dot8_u32_u4 | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1295](https://github.com/Marvisak/VIctory/pull/1295) | Shaders: decode 16-bit integer inline float constants as f32 bits | WALID1618Y | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1298](https://github.com/Marvisak/VIctory/pull/1298) | GPU: replace stale descriptors on null CBV writes | EatCodeSleep2 | files: `crates/victory-gpu/src/draw.rs` |
| [#1300](https://github.com/Marvisak/VIctory/pull/1300) | Shaders: lower v_{min3,max3,med3}_{i16,u16} and v_mad_{u32_u16,i32_i16} | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1313](https://github.com/Marvisak/VIctory/pull/1313) | Shaders: return zero for unused image query extents | EatCodeSleep2 | files: `crates/victory-shader/src/emit/waterfall_tests.rs` |
| [#1314](https://github.com/Marvisak/VIctory/pull/1314) | Shaders: lower v_{fmaak,fmamk}_f16, v_cvt_pk_{i16_i32,u16_u32}, and v_lerp_u8 | ESAT586 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1320](https://github.com/Marvisak/VIctory/pull/1320) | GPU diagnostics: skip host pipeline creation by shader hash | gurvancampion | files: `README.md`, `crates/victory-gpu/src/pipeline/make.rs` |
| [#1331](https://github.com/Marvisak/VIctory/pull/1331) | GPU: rebind replaced descriptor heaps by host identity | EatCodeSleep2 | files: `crates/victory-gpu/src/draw.rs` |
| [#1335](https://github.com/Marvisak/VIctory/pull/1335) | test(shader): translate random GFX10 programs on every test run | Levichu51 | files: `crates/victory-shader/src/emit/mod.rs` |
| [#1336](https://github.com/Marvisak/VIctory/pull/1336) | Shaders: fix RPO resource tracking, SMEM alignment, raw VCC branch, and readlane/writelane semantics (#1249) | wbkepler | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/wave.rs`, `crates/victory-shader/src/structure.rs` |
| [#1338](https://github.com/Marvisak/VIctory/pull/1338) | Shaders: honor op_sel[3], clamp, and omod on v_interp_p2_f16 | AscilCH | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1339](https://github.com/Marvisak/VIctory/pull/1339) | Shaders: support clamp output modifier on v_cvt_pkrtz_f16_f32 | AscilCH | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1340](https://github.com/Marvisak/VIctory/pull/1340) | Shaders: mask M0[7:0] on v_movrel* and allow scalar/const src0 on v_movreld_b32 | AscilCH | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1341](https://github.com/Marvisak/VIctory/pull/1341) | Shaders: lower v_ldexp_f32 (VOP3 0x362) | AscilCH | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1342](https://github.com/Marvisak/VIctory/pull/1342) | Shaders: lower buffer_atomic_{sub,csub,inc,dec,fcmpswap,fmin,fmax} | AscilCH | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1349](https://github.com/Marvisak/VIctory/pull/1349) | GPU: reserve 2^18 dynamic slots on capped view heaps and trim fresh tables (#747) | wbkepler | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs` |
| [#1355](https://github.com/Marvisak/VIctory/pull/1355) | HLE & Formats: replace miniz_oxide with SIMD-accelerated zlib-rs | wbkepler | files: `crates/victory-game/src/sgd5.rs`, `crates/victory-shader/src/native.rs` |
| [#1356](https://github.com/Marvisak/VIctory/pull/1356) | GPU: use RwLock for read-mostly tables and add per-HostList VA/view caches | wbkepler | files: `crates/victory-gpu/src/descriptor.rs`, `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/resource.rs` |
| [#1357](https://github.com/Marvisak/VIctory/pull/1357) | HLE: native 4-motor Xbox impulse trigger haptics via Windows.Gaming.Input with XInput fallback | wbkepler | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#1363](https://github.com/Marvisak/VIctory/pull/1363) | [BUNDLE] XAsync, kernel, lifecycle and DirectStorage HLE: #114 #122 #247 #1098 #186 #373 #512 #451 #1080 #430 #577 #1238 | MamaLeVrai | files: `crates/victory-hle/src/modules/xasync.rs` |
| [#1372](https://github.com/Marvisak/VIctory/pull/1372) | Loader: lazily NOP-patch clwb/monitorx on #UD, emulate rdpid, batch IAT protection, and content-hash cached DLLs | wbkepler | files: `crates/victory-loader/src/crash.rs`, `crates/victory-loader/src/windows.rs` |
| [#1382](https://github.com/Marvisak/VIctory/pull/1382) | GPU: batch resource barriers, reuse draw scratch buffers, range-lock copy_simple, and pool list uploads | wbkepler | files: `crates/victory-gpu/src/host.rs` |
| [#1388](https://github.com/Marvisak/VIctory/pull/1388) | GPU: honor command-list 64-bit memory waits without blocking submissions (#700) | wbkepler | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/list.rs` |
| [#1393](https://github.com/Marvisak/VIctory/pull/1393) | Shaders: vectorize wide s_buffer_load and global_load rawBufferLoad calls | wbkepler | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1397](https://github.com/Marvisak/VIctory/pull/1397) | Shader: propagate float values across selects/phis and cache bools across dominators | wbkepler | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1398](https://github.com/Marvisak/VIctory/pull/1398) | Shaders: honor clamp on unsigned VOP3 add and subtract | EatCodeSleep2 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1399](https://github.com/Marvisak/VIctory/pull/1399) | Shader & GPU: support merged ES+GS geometry stages (vs_6_6 / gs_6_6) | wbkepler | files: `crates/victory-shader/src/emit/bvh_tests.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/memory_tests.rs`, `crates/victory-shader/src/emit/mod.rs` |
| [#1403](https://github.com/Marvisak/VIctory/pull/1403) | CLI: report shader worker initialization failures | Luizbc2 | files: `apps/victory/src/translate.rs` |
| [#1405](https://github.com/Marvisak/VIctory/pull/1405) | GPU: a view's own copy that shaders feed reads its buffer's copy on the GPU (stacked on #334 and #1019) | SantannaCarlos | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/indirect.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#1406](https://github.com/Marvisak/VIctory/pull/1406) | GPU: fix NULL/UNKNOWN SRV/UAVs on promoted-typeless textures and MSAA/array/3D NULL RTV/DSVs | ESAT586 | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#1412](https://github.com/Marvisak/VIctory/pull/1412) | Shaders: keep scalar comparisons between lane masks uniform | EatCodeSleep2 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1414](https://github.com/Marvisak/VIctory/pull/1414) | GPU: avoid clobbering HostList prelude allocator when a list executes twice in one submission | ESAT586 | files: `crates/victory-gpu/src/host.rs` |
| [#1415](https://github.com/Marvisak/VIctory/pull/1415) | CLI: reuse the D3D12 host across translation workers | gurvancampion | files: `apps/victory/src/translate.rs` |
| [#1416](https://github.com/Marvisak/VIctory/pull/1416) | Shaders: return zero for inactive compute swizzle sources | EatCodeSleep2 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1425](https://github.com/Marvisak/VIctory/pull/1425) | Shaders: a refusal names the instruction that made its missing value | ThomasSAMP | files: `apps/victory/src/translate.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs` |
| [#1426](https://github.com/Marvisak/VIctory/pull/1426) | [BUNDLE] GPU host copies, descriptors, sampler heaps and footprints: 5 PRs, #322 #982 #409 #474 #1206 (rebuilt on main 70564ae) | MamaLeVrai | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/resource.rs` |
| [#1433](https://github.com/Marvisak/VIctory/pull/1433) | Shaders: console flow for groups of several console waves on 32-lane hosts | anasssssssss1 | files: `apps/victory/src/main.rs`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/shader_cache.rs` |
| [#1439](https://github.com/Marvisak/VIctory/pull/1439) | [BUNDLE] GPU pipeline library, shader cache, draw state and frame wait: 5 PRs, #1000 #626 #642 #174 #1279 (rebuilt on main 70564ae) | MamaLeVrai | files: `crates/victory-gpu/src/indirect.rs`, `crates/victory-gpu/src/list.rs` |
| [#1448](https://github.com/Marvisak/VIctory/pull/1448) | GPU: invalidate root bindings when a root signature changes | Luizbc2 | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/list.rs` |
| [#1461](https://github.com/Marvisak/VIctory/pull/1461) | GPU: handle DXGI Present failures and occlusion | Luizbc2 | files: `crates/victory-gpu/src/host.rs` |
| [#1462](https://github.com/Marvisak/VIctory/pull/1462) | CLI: report emitted shader wave sizes in translate | gurvancampion | files: `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/lower.rs` |
| [#1463](https://github.com/Marvisak/VIctory/pull/1463) | GPU: cap command lists per submission (VICTORY_SUBMIT_MAX_RUNS) | Bebrik229 | files: `crates/victory-gpu/src/host.rs` |
| [#1466](https://github.com/Marvisak/VIctory/pull/1466) | GPU: avoid faulting when zero-filling protected title memory | fluffyflattershy-afk | files: `crates/victory-gpu/src/host.rs` |
| [#1469](https://github.com/Marvisak/VIctory/pull/1469) | Shaders: lower binary64 class comparisons | Luizbc2 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1472](https://github.com/Marvisak/VIctory/pull/1472) | GPU: flush only overlapping shadow refreshes before Step::GpuWrite | Sakura-Byte | files: `crates/victory-gpu/src/host.rs` |
| [#1483](https://github.com/Marvisak/VIctory/pull/1483) | Pick the discrete GPU on laptops; fix driver-dependent and wrong shader lowering | spy0eye-beep | files: `crates/victory-gpu/src/host.rs`, `crates/victory-hle/src/modules/xasync.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/sampler.rs` |
| [#1486](https://github.com/Marvisak/VIctory/pull/1486) | Shaders: track source lane in whole-wave Sym::Own to prevent miscompiled shuffles | AscilCH | files: `crates/victory-shader/src/emit/wave.rs` |
| [#1487](https://github.com/Marvisak/VIctory/pull/1487) | Shader tests: prefer high-performance hardware adapter in emit::host::Host::new | AscilCH | files: `crates/victory-shader/src/emit/host.rs` |
| [#1488](https://github.com/Marvisak/VIctory/pull/1488) | Shaders: lower ds_add{_rtn}_f32 and ds_cmpst{_rtn}_f32 LDS atomics | AscilCH | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1489](https://github.com/Marvisak/VIctory/pull/1489) | Shaders: lower image_gather4_c_lz and image_gather4_c_lz_o (MIMG 0x4f, 0x5f) | AscilCH | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1514](https://github.com/Marvisak/VIctory/pull/1514) | Add opt-in GPU memory synchronization tracing | draggygaggy-maker | files: `README.md` |
| [#1516](https://github.com/Marvisak/VIctory/pull/1516) | Shaders: zero missing typed-buffer components | venglov | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1517](https://github.com/Marvisak/VIctory/pull/1517) | Input: add Windows DualSense / DualSense Edge support | Asem912 | files: `README.md`, `crates/victory-core/src/config.rs`, `crates/victory-hle/Cargo.toml`, `crates/victory-hle/src/modules/gameinput/mod.rs`, `crates/victory-hle/src/modules/lifecycle.rs` |
| [#1540](https://github.com/Marvisak/VIctory/pull/1540) | Shaders: guard both back edges when a conditional branch targets two … | Texbirds | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1543](https://github.com/Marvisak/VIctory/pull/1543) | GPU: avoid shift and arithmetic overflow panics in subresource and copyable_footprints (victory-gpu) | Texbirds | files: `crates/victory-gpu/src/resource.rs` |
| [#1560](https://github.com/Marvisak/VIctory/pull/1560) | PE: refuse an import address table inside the headers | default-cybe | files: `crates/victory-pe/src/image.rs` |
| [#1568](https://github.com/Marvisak/VIctory/pull/1568) | Shaders: model 64-bit carry sums and f16 reductions in whole-wave sections | MartinIndra02 | files: `crates/victory-shader/src/emit/valu_tests.rs`, `crates/victory-shader/src/emit/wave.rs` |
| [#1570](https://github.com/Marvisak/VIctory/pull/1570) | Shaders: govern pixel discard by last exp VM (Valid Mask) and guard pixel MRT exports | ESAT586 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1571](https://github.com/Marvisak/VIctory/pull/1571) | GPU: add diagnostics for shader dependency wait times | srijann07 | files: `crates/victory-gpu/src/pipeline.rs`, `crates/victory-gpu/src/pipeline/make.rs` |
| [#1578](https://github.com/Marvisak/VIctory/pull/1578) | GPU: the "shader does not translate" warning names where its value was made | ThomasSAMP | files: `apps/victory/src/translate.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs` |
| [#1584](https://github.com/Marvisak/VIctory/pull/1584) | GPU: add opt-in idle shader cache trimming | srijann07 | files: `README.md`, `crates/victory-gpu/src/pipeline.rs` |
| [#1590](https://github.com/Marvisak/VIctory/pull/1590) | XMA tests: count frames only in the selected stream | zhaibingye | files: `crates/victory-formats/src/xma.rs` |
| [#1602](https://github.com/Marvisak/VIctory/pull/1602) | Shader tests: f32/int conversions saturate, take NaN as 0, round to even | vladbondarenko97 | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1606](https://github.com/Marvisak/VIctory/pull/1606) | Shaders: lower remaining 32-bit integer and bitwise LDS atomics | HumanlyDev | files: `crates/victory-shader/src/emit/mod.rs` |
| [#1609](https://github.com/Marvisak/VIctory/pull/1609) | victory doctor: name the known problems a run shows, in plain words (stacked on #959) | EduBertozzi | files: `Cargo.lock`, `apps/victory/src/main.rs` |
| [#1618](https://github.com/Marvisak/VIctory/pull/1618) | victory menu: speak Portuguese where Windows does (stacked on #1609) | EduBertozzi | files: `Cargo.lock`, `apps/victory/src/main.rs` |
| [#1619](https://github.com/Marvisak/VIctory/pull/1619) | Shader tests: pin IR reachability and PHI invariants | ArnabSaga | files: `crates/victory-shader/src/ir.rs` |
| [#1625](https://github.com/Marvisak/VIctory/pull/1625) | CLI: compare runtime shader failures with offline coverage | srijann07 | files: `README.md` |
| [#1642](https://github.com/Marvisak/VIctory/pull/1642) | Shaders: preserve signaling NaNs in typed buffer loads | zhaibingye | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1653](https://github.com/Marvisak/VIctory/pull/1653) | victory-gpu: split the pipeline library into shards so a save writes only what changed (was 600 MB per save) | pes7 | files: `crates/victory-gpu/src/pipeline/library.rs` |
| [#1661](https://github.com/Marvisak/VIctory/pull/1661) | GPU: route non-overlapping CopyMemoryToMemoryX through Step::Copy and scope invalidation to written ranges | anlnimus | files: `crates/victory-gpu/src/host.rs` |
| [#1662](https://github.com/Marvisak/VIctory/pull/1662) | Shaders: lower RDNA 2 quad-byte SAD instructions to DXIL | THE-PRO-GAME | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1663](https://github.com/Marvisak/VIctory/pull/1663) | Shaders: lower RDNA 2 ADDTID memory instructions to DXIL | THE-PRO-GAME | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1666](https://github.com/Marvisak/VIctory/pull/1666) | GPU: small first scratch chunk per list; log host memory against budget | cyudev | files: `crates/victory-gpu/src/host.rs` |
| [#1682](https://github.com/Marvisak/VIctory/pull/1682) | Shaders: interpolations with any i/j by hand; barycentrics read as data | federalcontraband | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/shader.rs`, `crates/victory-shader/src/io.rs` |
| [#1691](https://github.com/Marvisak/VIctory/pull/1691) | GPU: track D3D12 shadow-buffer states (#1670) | Elensito | files: `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/indirect.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#1710](https://github.com/Marvisak/VIctory/pull/1710) | GPU: keep dropped draws in the full frame reporting interval | zhaibingye | files: `crates/victory-gpu/src/host.rs` |
| [#1727](https://github.com/Marvisak/VIctory/pull/1727) | GPU: 1D textures are 2D on the host, one texel high (GBV id=940) | Bullet18x | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1734](https://github.com/Marvisak/VIctory/pull/1734) | victory-shader: make the test dispatches cheap - an event wait and reused command objects (emit suite -33% wall, -48% CPU) | pes7 | files: `crates/victory-shader/src/emit/host.rs` |
| [#1735](https://github.com/Marvisak/VIctory/pull/1735) | GPU: time commands by pipeline in VICTORY_GPU_TIMING | Nixitov | files: `README.md`, `apps/victory/src/translate.rs`, `crates/victory-gpu/src/host.rs` |
| [#1738](https://github.com/Marvisak/VIctory/pull/1738) | Shaders: preserve live interpolation first halves (on #453) | jaschinski97-ops | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/io.rs` |
| [#1739](https://github.com/Marvisak/VIctory/pull/1739) | GPU: clear final pipeline stream padding after moves (on #1690) | jaschinski97-ops | files: `crates/victory-gpu/src/pipeline/host.rs` |
| [#1747](https://github.com/Marvisak/VIctory/pull/1747) | Shaders: lower v_div_scale_f32 and v_div_fmas_f32 | ESAT586 | files: `crates/victory-shader/src/structure.rs` |
| [#1750](https://github.com/Marvisak/VIctory/pull/1750) | GPU: a wait for a submission ends when the device is lost or the GPU stalls | x3ero0 | files: `crates/victory-gpu/src/host.rs` |
| [#1761](https://github.com/Marvisak/VIctory/pull/1761) | Input: trigger rumble through Windows.Gaming.Input with one pad conne… | bemalinho | files: `crates/victory-hle/src/modules/gameinput.rs` |
| [#1774](https://github.com/Marvisak/VIctory/pull/1774) | victory: meshdump surveys mesh and compute programs for group shape, LDS and divergent barriers | ryosen0 | files: `README.md` |
| [#1777](https://github.com/Marvisak/VIctory/pull/1777) | GPU: size the view heap clamp from what the host takes (NVIDIA 1,000,000 cap) | antoinerz1 | files: `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/shader.rs` |
| [#1779](https://github.com/Marvisak/VIctory/pull/1779) | GPU: view heaps on hosts capped below 2^21 descriptors | wbkepler | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/shader.rs` |
| [#1784](https://github.com/Marvisak/VIctory/pull/1784) | Test Wave32 spilled lane masks at control-flow joins | rezioliver-collab | files: `crates/victory-shader/src/emit/valu_tests.rs` |
| [#1786](https://github.com/Marvisak/VIctory/pull/1786) | Input: support DualShock 4 over HID and preserve keyboard controls | delacor | files: `README.md`, `crates/victory-hle/src/modules/gameinput.rs` |
| [#1796](https://github.com/Marvisak/VIctory/pull/1796) | Core: report unreadable implicit configuration files | VlREX | files: `crates/victory-core/src/config.rs` |
| [#1797](https://github.com/Marvisak/VIctory/pull/1797) | XMA: reject frame starts in packet headers | VlREX | files: `crates/victory-formats/src/xma.rs` |
| [#1799](https://github.com/Marvisak/VIctory/pull/1799) | fix(shader,gpu): eliminate 22s s_barrier world TDRs, zero invalid indirect mesh args, and reach Vice City opening credits & world frames | legayclightning22 | files: `crates/victory-gpu/src/capture.rs`, `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/pipeline/make.rs` |
| [#1807](https://github.com/Marvisak/VIctory/pull/1807) | Refactor/iterative shader bdd | VlREX | files: `crates/victory-shader/src/structure.rs` |
| [#1821](https://github.com/Marvisak/VIctory/pull/1821) | CLI: add offline shader and GPU regression comparison report with syn… | VlREX | files: `apps/victory/src/main.rs` |
| [#1838](https://github.com/Marvisak/VIctory/pull/1838) | XGameSave: an old version another process holds open no longer fails the save | adamMasMusic | files: `crates/victory-hle/src/modules/xgamesave.rs` |
| [#1845](https://github.com/Marvisak/VIctory/pull/1845) | NVIDIA D3D12 fixes & shader translation improvements (dynamic descriptors, Wave32 compute, untracked masks) | mrsqarvc-byte | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-gpu/src/draw.rs`, `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/lower.rs` |
| [#1846](https://github.com/Marvisak/VIctory/pull/1846) | Shaders: worklist register liveness with 96 independent synthetic CFG… | VlREX | files: `README.md` |
| [#1852](https://github.com/Marvisak/VIctory/pull/1852) | GPU: publish descriptor address snapshots before releasing writer lock | VlREX | files: `crates/victory-gpu/src/descriptor/host.rs` |
| [#1858](https://github.com/Marvisak/VIctory/pull/1858) | Shader tests: the permute test skips a device with shorter waves | kais7654 | files: `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/synthetic_tests.rs` |
| [#1861](https://github.com/Marvisak/VIctory/pull/1861) | fix | Yeetbot159 | files: `README.md`, `apps/victory/src/main.rs`, `apps/victory/src/shader.rs`, `apps/victory/src/translate.rs`, `crates/victory-gpu/Cargo.toml` |
| [#1862](https://github.com/Marvisak/VIctory/pull/1862) | GPU: index bindless texture slots for targeted release and preserve u… | VlREX | files: `README.md` |
| [#1864](https://github.com/Marvisak/VIctory/pull/1864) | Shaders: global byte and half stores keep the rest of their dword | TinkerBell-90 | files: `crates/victory-shader/src/emit/memory_tests.rs` |
| [#1871](https://github.com/Marvisak/VIctory/pull/1871) | Shaders: lower indexed tbuffer_load_format_* and indexed buffer_store_{byte,short} | ESAT586 | files: `crates/victory-shader/src/emit/lower.rs` |
| [#1890](https://github.com/Marvisak/VIctory/pull/1890) | Shaders: console wave64 compute on 32-lane hosts (split waves) | beamdotdev | files: `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/bvh_tests.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/memory_tests.rs` |
| [#1899](https://github.com/Marvisak/VIctory/pull/1899) | Shaders: lower 64-bit texture atomics to DXIL i64 atomics | TansuoTro | files: `crates/victory-gpu/src/descriptor/host.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/mod.rs`, `crates/victory-shader/src/emit/shader.rs` |
| [#1915](https://github.com/Marvisak/VIctory/pull/1915) | Shaders: lower 32-bit and 64-bit global atomics (cmpswap, csub, inc, dec, float, _x2) | ESAT586 | files: `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/memory_tests.rs` |
| [#1923](https://github.com/Marvisak/VIctory/pull/1923) | GPU: run submissions asynchronously on a host fence timeline (rebase of #595) | slatefield | files: `crates/victory-gpu/src/host.rs`, `crates/victory-gpu/src/shadow.rs` |
| [#1924](https://github.com/Marvisak/VIctory/pull/1924) | Shader &amp; GPU: merged geometry stages (rebase of #1399, with the batched GSVS write fix) | slatefield | files: `crates/victory-shader/src/emit/bvh_tests.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs`, `crates/victory-shader/src/emit/memory_tests.rs`, `crates/victory-shader/src/emit/mod.rs` |
| [#1929](https://github.com/Marvisak/VIctory/pull/1929) | Shaders: wave64 compute on 32-lane hosts, in halves and in the console flow (rebase of #233) | slatefield | files: `apps/victory/src/translate.rs`, `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/bvh_tests.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/lower.rs` |
| [#1962](https://github.com/Marvisak/VIctory/pull/1962) | GPU: make placed-texture heap slots that shaders actually read | antocorvo3000 | files: `crates/victory-gpu/src/shader_cache.rs`, `crates/victory-shader/src/emit/host.rs`, `crates/victory-shader/src/emit/shader.rs` |

## breaks the build (31)

| PR | Title | Author | Detail |
|---|---|---|---|
| [#317](https://github.com/Marvisak/VIctory/pull/317) | Shaders: support cube dimension in descriptor_data | tarcisioads | crates\victory-shader\src\emit\lower.rs:10901:26: error[E0063]: missing field `view_marks` in initializer of `Parts<'_>`: missing `view_marks` / error: could not compile `victory-shader` (lib test) due to 1 previous error |
| [#395](https://github.com/Marvisak/VIctory/pull/395) | shader: unpack compressed f16 pixel exports | Loggerten | crates\victory-shader\src\emit\lower.rs:10500:16: error[E0063]: missing field `view_marks` in initializer of `Parts<'_>`: missing `view_marks` / error: could not compile `victory-shader` (lib test) due to 1 previous error |
| [#529](https://github.com/Marvisak/VIctory/pull/529) | PE: preallocate symbol_key in CodeViewInfo | pushkarreddyy | crates\victory-pe\src\image.rs:879:1: error: unexpected closing delimiter: `}`: unexpected closing delimiter / error: could not compile `victory-pe` (lib test) due to 1 previous error / error: could not compile `victory-pe` (lib) due to 1 previous error |
| [#573](https://github.com/Marvisak/VIctory/pull/573) | Shaders: scope wave64 ds_bpermute_b32 to 32-lane halves | banbanzhige | crates\victory-shader\src\emit\lower.rs:11513:38: error[E0063]: missing field `view_marks` in initializer of `Parts<'_>`: missing `view_marks` / error: could not compile `victory-shader` (lib test) due to 1 previous error |
| [#626](https://github.com/Marvisak/VIctory/pull/626) | GPU: fall back to standard root signature flags when host runtime rejects 0xC01/0xC00 | LAKSHYA-cmd-tech | crates\victory-gpu\src\pipeline\host.rs:950:1: error[E0428]: the name `tests` is defined multiple times: `tests` redefined here / error: could not compile `victory-gpu` (lib test) due to 1 previous error |
| [#642](https://github.com/Marvisak/VIctory/pull/642) | GPU: translate enhanced texture barriers in ID3D12GraphicsCommandList7::Barrier | nathanbzrr | crates\victory-gpu\src\list.rs:2737:30: error[E0308]: mismatched types: expected `Option<&ID3D12Resource>`, found `&ID3D12Resource` / error: could not compile `victory-gpu` (lib test) due to 1 previous error |
| [#652](https://github.com/Marvisak/VIctory/pull/652) | GPU: use selected mip extents for MSAA resolves | fuzeofficlal | crates\victory-gpu\src\list.rs:2016:39: error[E0308]: mismatched types: expected `Option<&ID3D12Resource>`, found `&ID3D12Resource` / crates\victory-gpu\src\list.rs:2063:36: error[E0308]: mismatched types: expected `Option<&ID3D12Resource>`, found `&ID3D12Resource` / error: could not compile `victory-gpu` (lib test) due to 2 previous errors |
| [#1004](https://github.com/Marvisak/VIctory/pull/1004) | Shaders: lower image_get_lod on cube textures | tarcisioads | crates\victory-shader\src\emit\lower.rs:11586:16: error[E0063]: missing field `view_marks` in initializer of `Parts<'_>`: missing `view_marks` / error: could not compile `victory-shader` (lib test) due to 1 previous error |
| [#1166](https://github.com/Marvisak/VIctory/pull/1166) | GPU: implement ID3D12GraphicsCommandList::ClearState (slot 0x58) | geiyiliyabumo | crates\victory-gpu\src\draw.rs:2782:25: error[E0063]: missing field `depth_bounds` in initializer of `draw::State`: missing `depth_bounds` / error: could not compile `victory-gpu` (lib test) due to 1 previous error |
| [#1281](https://github.com/Marvisak/VIctory/pull/1281) | GPU/Shader: declare wave64 wave size and strip unsupported video-texture flags | kss306 | crates\victory-gpu\src\resource.rs:1227:45: error[E0425]: cannot find value `strip_rtv` in this scope: not found in this scope / error: could not compile `victory-gpu` (lib) due to 1 previous error / error: could not compile `victory-gpu` (lib test) due to 1 previous error |
| [#1623](https://github.com/Marvisak/VIctory/pull/1623) | gfx10: keep an instruction's words inline instead of in a heap vector (decode -18..-45%, translation -1..-5%) | pes7 | crates\victory-shader\src\geometry.rs:217:36: error[E0616]: field `words` of struct `Instruction` is private: private field / crates\victory-shader\src\geometry.rs:218:30: error[E0616]: field `words` of struct `Instruction` is private: private field / crates\victory-shader\src\geometry.rs:219:32: error[E0616]: field `words` of struct `Instruction` is private: private field |
| [#1795](https://github.com/Marvisak/VIctory/pull/1795) | Shaders: LDS halfwords straddling two dwords, and hardware tests of byte and halfword LDS accesses | dragoso | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 524803 bytes exceeds the 524288-byte limit |
| [#1816](https://github.com/Marvisak/VIctory/pull/1816) | GPU: account for delivered bytes in protected bulk copyback | aycow | crates\victory-gpu\src\shadow.rs:1090:14: error[E0061]: this method takes 3 arguments but 2 arguments were supplied / error: could not compile `victory-gpu` (lib test) due to 1 previous error |
| [#1841](https://github.com/Marvisak/VIctory/pull/1841) | Shaders: honor FP32 rounding when packing a byte | AlanQiang0418 | crates\victory-shader\src\emit\lower.rs:7445:44: error[E0609]: no field `mode16` on type `&mut lower::Lower<'a>`: unknown field / error: could not compile `victory-shader` (lib) due to 1 previous error / error: could not compile `victory-shader` (lib test) due to 1 previous error |
| [#1853](https://github.com/Marvisak/VIctory/pull/1853) | Shaders: extend image_gather4_l(_o) to all address modes, borders, a16, d16, unorm, and 2D arrays | ESAT586 | crates\victory-shader\src\emit\lower.rs:9065:21: error[E0425]: cannot find value `extra` in this scope: not found in this scope / crates\victory-shader\src\emit\lower.rs:9065:38: error[E0425]: cannot find value `extra` in this scope: not found in this scope / crates\victory-shader\src\emit\lower.rs:9072:48: error[E0425]: cannot find value `extra` in this scope: not found in this scope |
| [#1922](https://github.com/Marvisak/VIctory/pull/1922) | GPU: refuse wave64 mesh and compute programs on hosts without 64-lane waves (rebase of #578) | slatefield | crates\victory-gpu\src\pipeline\make.rs:399:18: error[E0063]: missing field `wave64` in initializer of `pipeline::Shader`: missing `wave64` / error: could not compile `victory-gpu` (lib test) due to 1 previous error |
| [#1940](https://github.com/Marvisak/VIctory/pull/1940) | Shaders: d16 format loads of UNORM / SNORM data round once to nearest | upipibi | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 527592 bytes exceeds the 524288-byte limit |
| [#1949](https://github.com/Marvisak/VIctory/pull/1949) | Shaders: LDS accesses past the program's allocation read 0 and drop writes (#1870) | wwwwwwww3333333 | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 525279 bytes exceeds the 524288-byte limit |
| [#1956](https://github.com/Marvisak/VIctory/pull/1956) | Shaders: cube face and multisampled image loads | RaghavVijayanand | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 528463 bytes exceeds the 524288-byte limit |
| [#1961](https://github.com/Marvisak/VIctory/pull/1961) | Shaders: refuse a mesh program over the output budget | Luislariosss | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 524578 bytes exceeds the 524288-byte limit |
| [#2037](https://github.com/Marvisak/VIctory/pull/2037) | Shaders: byte and half format loads at a constant offset load the aligned dword directly (NVIDIA) | desplmfao | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 524753 bytes exceeds the 524288-byte limit |
| [#2062](https://github.com/Marvisak/VIctory/pull/2062) | Shaders: lane conditions along live phi edges are values, and each set is met once | 0xkn0wh3r3 | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 525228 bytes exceeds the 524288-byte limit |
| [#2063](https://github.com/Marvisak/VIctory/pull/2063) | Shaders: lower s_ff0_i32_b32 and s_ff0_i32_b64 in pure 32-bit DXIL | geiyiliyabumo | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 524602 bytes exceeds the 524288-byte limit |
| [#2070](https://github.com/Marvisak/VIctory/pull/2070) | Shaders: refuse the image instructions DXIL has no form of | 0xkn0wh3r3 | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 528700 bytes exceeds the 524288-byte limit |
| [#2077](https://github.com/Marvisak/VIctory/pull/2077) | Shaders: lower s_sext_i32_i8 and s_sext_i32_i16 in pure 32-bit DXIL | geiyiliyabumo | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 524427 bytes exceeds the 524288-byte limit |
| [#2083](https://github.com/Marvisak/VIctory/pull/2083) | Shaders: lower s_bitreplicate_b64_b32 in pure 32-bit DXIL | geiyiliyabumo | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 525455 bytes exceeds the 524288-byte limit |
| [#2084](https://github.com/Marvisak/VIctory/pull/2084) | Shaders: buffer loads of a table in the program's code | rminzao | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 527887 bytes exceeds the 524288-byte limit |
| [#2093](https://github.com/Marvisak/VIctory/pull/2093) | GPU: our compute shaders compiled by the build on Windows | Deilach | crates\victory-gpu\src\twin.rs:706:17: error: cannot find macro `s` in this scope / crates\victory-gpu\src\twin.rs:705:17: error: cannot find macro `s` in this scope / crates\victory-gpu\src\twin.rs:702:17: error: cannot find macro `s` in this scope |
| [#2100](https://github.com/Marvisak/VIctory/pull/2100) | Shaders: lower v_cvt_pknorm_{i16,u16}_{f32,f16} in pure 32-bit DXIL | geiyiliyabumo | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 525947 bytes exceeds the 524288-byte limit |
| [#2104](https://github.com/Marvisak/VIctory/pull/2104) | Shaders: lower s_wqm_b32 and s_wqm_b64 in pure 32-bit DXIL | geiyiliyabumo | asset policy: POLICY: crates/victory-shader/src/emit/lower.rs: 525547 bytes exceeds the 524288-byte limit |
| [#2117](https://github.com/Marvisak/VIctory/pull/2117) | Shaders: compute reading lanes only within aligned groups declares no wave size | rminzao | crates\victory-shader\src\emit\crosslane_tests.rs:417:18: error[E0063]: missing field `min_lanes` in initializer of `emit::HostShader`: missing `min_lanes` / crates\victory-shader\src\emit\valu_tests.rs:2907:18: error[E0063]: missing field `min_lanes` in initializer of `emit::HostShader`: missing `min_lanes` / crates\victory-shader\src\emit\mod.rs:2366:22: error[E0063]: missing field `min_lanes` in initializer of `emit::HostShader`: missing `min_lanes` |

## changes CI workflows (5)

| PR | Title | Author | Detail |
|---|---|---|---|
| [#113](https://github.com/Marvisak/VIctory/pull/113) | GitHub: rebase issue forms and PR template on main | Quat3rnion | files: `.github/workflows/pr-description.yml` |
| [#509](https://github.com/Marvisak/VIctory/pull/509) | GPU: add an opt-in execution performance census | aycow | files: `.github/workflows/ci.yml` |
| [#1058](https://github.com/Marvisak/VIctory/pull/1058) | CI: serialize Windows workspace tests | divyeshganta | files: `.github/workflows/ci.yml` |
| [#1241](https://github.com/Marvisak/VIctory/pull/1241) | CI: build and test the host-independent crates on Linux | MIchiel1234 | files: `.github/workflows/ci.yml` |
| [#2135](https://github.com/Marvisak/VIctory/pull/2135) | ci: Automated GitHub Actions Build Workflow | 7d4d9e33 | files: `.github/workflows/build-windows.yml` |

## draft (13)

| PR | Title | Author | Detail |
|---|---|---|---|
| [#387](https://github.com/Marvisak/VIctory/pull/387) | GPU: preserve execution ordering for buffer copies | aycow |  |
| [#390](https://github.com/Marvisak/VIctory/pull/390) | Crash log: a panic carries our Rust stack | Kaairim |  |
| [#391](https://github.com/Marvisak/VIctory/pull/391) | Shaders: refuse LDS past the host's group limit by name | Kaairim |  |
| [#562](https://github.com/Marvisak/VIctory/pull/562) | GPU: reuse immutable descriptor tables across command lists | aycow |  |
| [#1179](https://github.com/Marvisak/VIctory/pull/1179) | Input: choose the loose controls diagram by active gamepad (stacked on #312) | Razdva122 |  |
| [#1789](https://github.com/Marvisak/VIctory/pull/1789) | fix(core): keep persistent data outside the cache | dereoalex |  |
| [#1872](https://github.com/Marvisak/VIctory/pull/1872) | GPU: the depth twin's converter builds its root signature when the compiler drops it (Wine) | BastienRodz |  |
| [#1906](https://github.com/Marvisak/VIctory/pull/1906) | GPU: view addresses for tables made afresh (dynamic slots) | Slay326 |  |
| [#1965](https://github.com/Marvisak/VIctory/pull/1965) | GTA VI: all local work to reach the lit world (draft, for reference) | antocorvo3000 |  |
| [#1970](https://github.com/Marvisak/VIctory/pull/1970) | GTA VI: first approach with the title memory mirror (draft, for reference) | antocorvo3000 |  |
| [#2028](https://github.com/Marvisak/VIctory/pull/2028) | GPU: pace opening movie presents that skip a flip wait | Chezzuss |  |
| [#2120](https://github.com/Marvisak/VIctory/pull/2120) | GPU: copies of read-only title buffers in GPU upload heaps (opt-in) | 0xkn0wh3r3 |  |
| [#2130](https://github.com/Marvisak/VIctory/pull/2130) | GPU: copies of written title buffers in video memory with a mirror (opt-in) | 0xkn0wh3r3 |  |
