window.BENCHMARK_DATA = {
  "lastUpdate": 1770602424516,
  "repoUrl": "https://github.com/horizonanalytic/astrora",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/cachemcclure/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1761528345846,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5284140.556098101,
            "unit": "iter/sec",
            "range": "stddev: 1.220575639992347e-8",
            "extra": "mean: 189.24553375967753 nsec\nrounds: 53320"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4873055.036823655,
            "unit": "iter/sec",
            "range": "stddev: 1.2313366724529601e-8",
            "extra": "mean: 205.21007713712788 nsec\nrounds: 49781"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3148863.4058174654,
            "unit": "iter/sec",
            "range": "stddev: 1.1248506526586253e-7",
            "extra": "mean: 317.5749059651635 nsec\nrounds: 189754"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 710273.6059281536,
            "unit": "iter/sec",
            "range": "stddev: 1.1180036658384566e-7",
            "extra": "mean: 1.407908152089163 usec\nrounds: 72276"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 70746.33215591662,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015597722362876572",
            "extra": "mean: 14.135008409992441 usec\nrounds: 74317"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 394873.1299940816,
            "unit": "iter/sec",
            "range": "stddev: 3.3857188093338593e-7",
            "extra": "mean: 2.5324589698341464 usec\nrounds: 138256"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 390834.7826873822,
            "unit": "iter/sec",
            "range": "stddev: 5.846271998817571e-7",
            "extra": "mean: 2.558625906128388 usec\nrounds: 137118"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 345839.4285511672,
            "unit": "iter/sec",
            "range": "stddev: 4.565189673376724e-7",
            "extra": "mean: 2.8915153028945317 usec\nrounds: 182482"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 208756.49835177136,
            "unit": "iter/sec",
            "range": "stddev: 5.542528058767285e-7",
            "extra": "mean: 4.790270041390137 usec\nrounds: 109087"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39651.05450375745,
            "unit": "iter/sec",
            "range": "stddev: 0.00000229315872062057",
            "extra": "mean: 25.220010224576427 usec\nrounds: 41077"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3653976.1810554904,
            "unit": "iter/sec",
            "range": "stddev: 3.2653022713379706e-8",
            "extra": "mean: 273.6744714386373 nsec\nrounds: 197668"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3406435.013436119,
            "unit": "iter/sec",
            "range": "stddev: 3.5113025486270805e-8",
            "extra": "mean: 293.56203657364887 nsec\nrounds: 193051"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2190192.837286016,
            "unit": "iter/sec",
            "range": "stddev: 5.3193488014364854e-8",
            "extra": "mean: 456.58080100341084 nsec\nrounds: 186916"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 511656.9907867358,
            "unit": "iter/sec",
            "range": "stddev: 1.2791699992077263e-7",
            "extra": "mean: 1.9544343534960407 usec\nrounds: 52149"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1526759.3560007708,
            "unit": "iter/sec",
            "range": "stddev: 7.610692825574368e-8",
            "extra": "mean: 654.9820677826959 nsec\nrounds: 157928"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1519051.5230896699,
            "unit": "iter/sec",
            "range": "stddev: 7.849201212554682e-8",
            "extra": "mean: 658.3055181472807 nsec\nrounds: 155958"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1264090.2425150424,
            "unit": "iter/sec",
            "range": "stddev: 8.428481971487567e-8",
            "extra": "mean: 791.0827616313162 nsec\nrounds: 129467"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 444787.06191453966,
            "unit": "iter/sec",
            "range": "stddev: 1.4729586988711036e-7",
            "extra": "mean: 2.2482668351359014 usec\nrounds: 45307"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2628211.5862187934,
            "unit": "iter/sec",
            "range": "stddev: 4.458460810979715e-8",
            "extra": "mean: 380.48686994745776 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 502540.6741201557,
            "unit": "iter/sec",
            "range": "stddev: 1.586555992825026e-7",
            "extra": "mean: 1.9898886826440287 usec\nrounds: 51664"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2863654.781098064,
            "unit": "iter/sec",
            "range": "stddev: 4.284014663516987e-8",
            "extra": "mean: 349.2041033020578 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2828881.7916936595,
            "unit": "iter/sec",
            "range": "stddev: 9.00234851246522e-8",
            "extra": "mean: 353.49656635931086 nsec\nrounds: 193837"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1775648.0413973283,
            "unit": "iter/sec",
            "range": "stddev: 7.094792284205049e-8",
            "extra": "mean: 563.1746701407548 nsec\nrounds: 183487"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 496025.1754784889,
            "unit": "iter/sec",
            "range": "stddev: 1.7677085553727725e-7",
            "extra": "mean: 2.016026704764258 usec\nrounds: 50901"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36236.91663579657,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021757724126619358",
            "extra": "mean: 27.596166915927714 usec\nrounds: 37468"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1229634.648154203,
            "unit": "iter/sec",
            "range": "stddev: 1.0229795190561585e-7",
            "extra": "mean: 813.2496929076124 nsec\nrounds: 128469"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1218199.9085372475,
            "unit": "iter/sec",
            "range": "stddev: 9.113648825605249e-8",
            "extra": "mean: 820.8833320310389 nsec\nrounds: 128288"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 876124.3348484722,
            "unit": "iter/sec",
            "range": "stddev: 1.8215706022980757e-7",
            "extra": "mean: 1.1413905084293297 usec\nrounds: 92166"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 359727.7560361788,
            "unit": "iter/sec",
            "range": "stddev: 3.966256152389125e-7",
            "extra": "mean: 2.7798800154287435 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 44508.79889351281,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020791255323386916",
            "extra": "mean: 22.467467666168606 usec\nrounds: 46020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2743232.4590473683,
            "unit": "iter/sec",
            "range": "stddev: 4.891114301258928e-8",
            "extra": "mean: 364.5334527528281 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2245510.698750869,
            "unit": "iter/sec",
            "range": "stddev: 5.389331517118981e-8",
            "extra": "mean: 445.3329928716376 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 780814.0370408336,
            "unit": "iter/sec",
            "range": "stddev: 9.638320859679183e-8",
            "extra": "mean: 1.2807146805273313 usec\nrounds: 79854"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107198.00725393042,
            "unit": "iter/sec",
            "range": "stddev: 8.485273372647893e-7",
            "extra": "mean: 9.328531617488018 usec\nrounds: 109338"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 366949.22499643144,
            "unit": "iter/sec",
            "range": "stddev: 4.0668449439217565e-7",
            "extra": "mean: 2.725172672076702 usec\nrounds: 191939"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 360155.7335690925,
            "unit": "iter/sec",
            "range": "stddev: 4.486628401900991e-7",
            "extra": "mean: 2.776576649468109 usec\nrounds: 189036"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 291624.5136296,
            "unit": "iter/sec",
            "range": "stddev: 4.7598912130845817e-7",
            "extra": "mean: 3.429067013447732 usec\nrounds: 152626"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 130366.68602827599,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010371244617374075",
            "extra": "mean: 7.6706713230641155 usec\nrounds: 135081"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2906638.775767096,
            "unit": "iter/sec",
            "range": "stddev: 4.918134045508e-7",
            "extra": "mean: 344.0399984810938 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2676516.2504909975,
            "unit": "iter/sec",
            "range": "stddev: 5.88770347680534e-7",
            "extra": "mean: 373.6199994364142 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2085462.2501617575,
            "unit": "iter/sec",
            "range": "stddev: 1.5830685863057384e-7",
            "extra": "mean: 479.50999828572094 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 509525.58121333615,
            "unit": "iter/sec",
            "range": "stddev: 1.7817967211256615e-7",
            "extra": "mean: 1.9626099981451262 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 54995.457372667806,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019859564839345085",
            "extra": "mean: 18.183320000844105 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 887453.1867803232,
            "unit": "iter/sec",
            "range": "stddev: 7.643228496236709e-7",
            "extra": "mean: 1.1268200000813522 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 980930.7082100064,
            "unit": "iter/sec",
            "range": "stddev: 3.878025120524086e-7",
            "extra": "mean: 1.0194399987994984 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 746324.3544922385,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010035472673349145",
            "extra": "mean: 1.339899996537497 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 441536.1931330124,
            "unit": "iter/sec",
            "range": "stddev: 9.733255538615334e-7",
            "extra": "mean: 2.2648199978902994 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 55275.09585578529,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025571342382562367",
            "extra": "mean: 18.09133000165275 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1644466.1653458427,
            "unit": "iter/sec",
            "range": "stddev: 7.491385667821797e-8",
            "extra": "mean: 608.1000759232006 nsec\nrounds: 168606"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 352728.47555439535,
            "unit": "iter/sec",
            "range": "stddev: 3.738742760667338e-7",
            "extra": "mean: 2.8350418786809484 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43388.67356586219,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013926987984253472",
            "extra": "mean: 23.047489536227513 usec\nrounds: 44009"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4410.670485199245,
            "unit": "iter/sec",
            "range": "stddev: 0.00000574850572762801",
            "extra": "mean: 226.7228992407549 usec\nrounds: 4476"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 204413.68554115397,
            "unit": "iter/sec",
            "range": "stddev: 5.885368267273014e-7",
            "extra": "mean: 4.892040360960436 usec\nrounds: 108496"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 130345.86017944207,
            "unit": "iter/sec",
            "range": "stddev: 0.000001036855096932803",
            "extra": "mean: 7.671896895101531 usec\nrounds: 137307"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31934.115572377956,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020019827228643313",
            "extra": "mean: 31.314473004067466 usec\nrounds: 32801"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3734.5981717591085,
            "unit": "iter/sec",
            "range": "stddev: 0.000007390054776673118",
            "extra": "mean: 267.76642466168454 usec\nrounds: 3836"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 132650.083759913,
            "unit": "iter/sec",
            "range": "stddev: 9.553452254908264e-7",
            "extra": "mean: 7.538630746814509 usec\nrounds: 142187"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16072.928776014489,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032586698383869127",
            "extra": "mean: 62.21641456486092 usec\nrounds: 16656"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1571.8790429430069,
            "unit": "iter/sec",
            "range": "stddev: 0.000011299430370455583",
            "extra": "mean: 636.1812662936928 usec\nrounds: 1611"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1285657.7825697449,
            "unit": "iter/sec",
            "range": "stddev: 8.464030959992256e-8",
            "extra": "mean: 777.8119601945871 nsec\nrounds: 132381"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104132.52528941797,
            "unit": "iter/sec",
            "range": "stddev: 9.148245179178431e-7",
            "extra": "mean: 9.603147500943402 usec\nrounds: 106406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17044.163994014114,
            "unit": "iter/sec",
            "range": "stddev: 0.000002408356517265293",
            "extra": "mean: 58.67110879425935 usec\nrounds: 17363"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2369.0511618851647,
            "unit": "iter/sec",
            "range": "stddev: 0.00000590529407971429",
            "extra": "mean: 422.10992151146843 usec\nrounds: 2408"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 724604.2501786479,
            "unit": "iter/sec",
            "range": "stddev: 1.192308929976637e-7",
            "extra": "mean: 1.380063668897116 usec\nrounds: 74322"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 103756.32388335532,
            "unit": "iter/sec",
            "range": "stddev: 9.826470533869348e-7",
            "extra": "mean: 9.63796675298768 usec\nrounds: 106987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 22271.714461079027,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029365231242354423",
            "extra": "mean: 44.900000929320086 usec\nrounds: 23670"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4293.72200760999,
            "unit": "iter/sec",
            "range": "stddev: 0.000015472137609379207",
            "extra": "mean: 232.89817045156795 usec\nrounds: 4535"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2066475.1730716918,
            "unit": "iter/sec",
            "range": "stddev: 6.172300703511803e-8",
            "extra": "mean: 483.91580650524423 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 969255.0721575321,
            "unit": "iter/sec",
            "range": "stddev: 1.0721205158107222e-7",
            "extra": "mean: 1.0317201619322243 usec\nrounds: 99911"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 360093.6262986283,
            "unit": "iter/sec",
            "range": "stddev: 3.297985845612642e-7",
            "extra": "mean: 2.7770555404685022 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 118854.07218626858,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010361012080748731",
            "extra": "mean: 8.413678905614576 usec\nrounds: 124767"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 883960.0815492885,
            "unit": "iter/sec",
            "range": "stddev: 1.0486581801615679e-7",
            "extra": "mean: 1.1312728039113873 usec\nrounds: 91241"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 713714.5170616739,
            "unit": "iter/sec",
            "range": "stddev: 1.2100850895585634e-7",
            "extra": "mean: 1.4011204425502695 usec\nrounds: 73660"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 494012.10342661175,
            "unit": "iter/sec",
            "range": "stddev: 1.4602312780523205e-7",
            "extra": "mean: 2.0242419023009983 usec\nrounds: 50539"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 167718.5953747275,
            "unit": "iter/sec",
            "range": "stddev: 9.675710481596253e-7",
            "extra": "mean: 5.962368083072343 usec\nrounds: 176960"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5473231.250783031,
            "unit": "iter/sec",
            "range": "stddev: 1.2155506769259617e-8",
            "extra": "mean: 182.70742714496663 nsec\nrounds: 56105"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 397310.7112255394,
            "unit": "iter/sec",
            "range": "stddev: 3.3604664097574536e-7",
            "extra": "mean: 2.5169218240188167 usec\nrounds: 139412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54822.12152618428,
            "unit": "iter/sec",
            "range": "stddev: 0.000001363360432988315",
            "extra": "mean: 18.24081177745698 usec\nrounds: 57007"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4141.549527858471,
            "unit": "iter/sec",
            "range": "stddev: 0.000007612273855623135",
            "extra": "mean: 241.4555212423317 usec\nrounds: 4190"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4386845.361633427,
            "unit": "iter/sec",
            "range": "stddev: 1.4146957135581692e-8",
            "extra": "mean: 227.95423990684137 nsec\nrounds: 44659"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 373361.1343529405,
            "unit": "iter/sec",
            "range": "stddev: 4.0638673309579576e-7",
            "extra": "mean: 2.678371978200318 usec\nrounds: 198060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2346501.5018924233,
            "unit": "iter/sec",
            "range": "stddev: 6.445544827088762e-8",
            "extra": "mean: 426.16635838225307 nsec\nrounds: 196079"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48268.018095237945,
            "unit": "iter/sec",
            "range": "stddev: 0.00000216959369738783",
            "extra": "mean: 20.71765196629564 usec\nrounds: 50383"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 110867.0819754259,
            "unit": "iter/sec",
            "range": "stddev: 0.000001036563858447984",
            "extra": "mean: 9.019809867654438 usec\nrounds: 117981"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13217.936893256048,
            "unit": "iter/sec",
            "range": "stddev: 0.0000041463038202036025",
            "extra": "mean: 75.6547718509847 usec\nrounds: 13798"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1280.9823067350674,
            "unit": "iter/sec",
            "range": "stddev: 0.000011978959893685772",
            "extra": "mean: 780.6509073093856 usec\nrounds: 1327"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 50890.36724295043,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020032983589805716",
            "extra": "mean: 19.65008417459445 usec\nrounds: 53009"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49079.22071602205,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021469809303604816",
            "extra": "mean: 20.375221639848636 usec\nrounds: 50925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35865.63418425861,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024922816043480984",
            "extra": "mean: 27.881843518018677 usec\nrounds: 37148"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 58978.757704324504,
            "unit": "iter/sec",
            "range": "stddev: 0.000005791471241832344",
            "extra": "mean: 16.95525709465184 usec\nrounds: 145709"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 248919.43480894793,
            "unit": "iter/sec",
            "range": "stddev: 5.480875635482075e-7",
            "extra": "mean: 4.017364095204241 usec\nrounds: 129300"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 26801.077575593656,
            "unit": "iter/sec",
            "range": "stddev: 0.000011693951796598616",
            "extra": "mean: 37.311932595973225 usec\nrounds: 47193"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32255.4771252755,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018547169593624928",
            "extra": "mean: 31.002486682064813 usec\nrounds: 32888"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8099.610845041003,
            "unit": "iter/sec",
            "range": "stddev: 0.00004421695463729587",
            "extra": "mean: 123.46272174449605 usec\nrounds: 11529"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3103.2674950986852,
            "unit": "iter/sec",
            "range": "stddev: 0.000007547060881463839",
            "extra": "mean: 322.2409932689994 usec\nrounds: 3120"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 16182.326981954971,
            "unit": "iter/sec",
            "range": "stddev: 0.000011576088658120793",
            "extra": "mean: 61.79580978156647 usec\nrounds: 22532"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28057.760109081748,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021697755027344206",
            "extra": "mean: 35.64076377131472 usec\nrounds: 28519"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 23842.031539012823,
            "unit": "iter/sec",
            "range": "stddev: 0.000013478382113266515",
            "extra": "mean: 41.94273455110969 usec\nrounds: 47173"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 22506.049346252326,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024173715581245044",
            "extra": "mean: 44.432498330344174 usec\nrounds: 23653"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 25162.51535021635,
            "unit": "iter/sec",
            "range": "stddev: 0.000008170819422150762",
            "extra": "mean: 39.74165484180825 usec\nrounds: 38478"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22433.875821059682,
            "unit": "iter/sec",
            "range": "stddev: 0.000002140716749484315",
            "extra": "mean: 44.57544509813392 usec\nrounds: 22768"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 15433.716865075017,
            "unit": "iter/sec",
            "range": "stddev: 0.00001037331224788767",
            "extra": "mean: 64.79320624721979 usec\nrounds: 20970"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 20877.208188846696,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024696479059957224",
            "extra": "mean: 47.89912477542057 usec\nrounds: 21102"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 32052.486247348737,
            "unit": "iter/sec",
            "range": "stddev: 0.000004836063003442152",
            "extra": "mean: 31.1988278314203 usec\nrounds: 52646"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 46323.101568401915,
            "unit": "iter/sec",
            "range": "stddev: 0.000001555270548671868",
            "extra": "mean: 21.58750096910876 usec\nrounds: 47464"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4491.562877142361,
            "unit": "iter/sec",
            "range": "stddev: 0.00006795007739610184",
            "extra": "mean: 222.63965291213373 usec\nrounds: 6301"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 915.2456694693477,
            "unit": "iter/sec",
            "range": "stddev: 0.000009397444739498413",
            "extra": "mean: 1.0926028205954716 msec\nrounds: 903"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1037.0757943534545,
            "unit": "iter/sec",
            "range": "stddev: 0.0001569073493446273",
            "extra": "mean: 964.2496772605047 usec\nrounds: 1227"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 275.66771124340005,
            "unit": "iter/sec",
            "range": "stddev: 0.00003257950585435658",
            "extra": "mean: 3.627555782610509 msec\nrounds: 276"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 48353.63247178501,
            "unit": "iter/sec",
            "range": "stddev: 0.000004324348086450919",
            "extra": "mean: 20.680969533850707 usec\nrounds: 87474"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 17559.906435468598,
            "unit": "iter/sec",
            "range": "stddev: 0.0000144497369615985",
            "extra": "mean: 56.9479116346621 usec\nrounds: 31698"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5580.253921882562,
            "unit": "iter/sec",
            "range": "stddev: 0.00005388969742976074",
            "extra": "mean: 179.2033147593109 usec\nrounds: 7968"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1749.2807042399625,
            "unit": "iter/sec",
            "range": "stddev: 0.00008895258757678552",
            "extra": "mean: 571.6635400917463 usec\nrounds: 1933"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 29299.44854024731,
            "unit": "iter/sec",
            "range": "stddev: 0.0000072073358873695646",
            "extra": "mean: 34.130335204990146 usec\nrounds: 49510"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1721.306959350013,
            "unit": "iter/sec",
            "range": "stddev: 0.00008269316374026268",
            "extra": "mean: 580.9539051521715 usec\nrounds: 1961"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1163.1509780253264,
            "unit": "iter/sec",
            "range": "stddev: 0.00012849680148722324",
            "extra": "mean: 859.7336191881928 usec\nrounds: 1355"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/cachemcclure/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1762133034058,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5343172.299611262,
            "unit": "iter/sec",
            "range": "stddev: 1.3547699954625226e-8",
            "extra": "mean: 187.15473578733832 nsec\nrounds: 53839"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4900940.674080488,
            "unit": "iter/sec",
            "range": "stddev: 1.4004087525488276e-8",
            "extra": "mean: 204.0424617438514 nsec\nrounds: 50058"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3186221.320710532,
            "unit": "iter/sec",
            "range": "stddev: 3.752035867646317e-8",
            "extra": "mean: 313.85139302783887 nsec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 710499.4169895903,
            "unit": "iter/sec",
            "range": "stddev: 1.342898585798486e-7",
            "extra": "mean: 1.407460690449279 usec\nrounds: 72649"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 72377.29799413247,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017397339439579915",
            "extra": "mean: 13.816487043783654 usec\nrounds: 76488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 414468.5675062208,
            "unit": "iter/sec",
            "range": "stddev: 4.3235414257759184e-7",
            "extra": "mean: 2.4127281979832733 usec\nrounds: 42730"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 410176.62172413245,
            "unit": "iter/sec",
            "range": "stddev: 1.8366497028322084e-7",
            "extra": "mean: 2.4379741482988964 usec\nrounds: 42767"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 361324.1022308562,
            "unit": "iter/sec",
            "range": "stddev: 4.616125216845573e-7",
            "extra": "mean: 2.767598380030244 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 214095.01594995515,
            "unit": "iter/sec",
            "range": "stddev: 5.510253143037602e-7",
            "extra": "mean: 4.670823351785782 usec\nrounds: 111272"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39552.754067352915,
            "unit": "iter/sec",
            "range": "stddev: 0.000002389161777102842",
            "extra": "mean: 25.282689501144148 usec\nrounds: 41195"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3638539.3439931152,
            "unit": "iter/sec",
            "range": "stddev: 3.326581723154178e-8",
            "extra": "mean: 274.83556049803684 nsec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3379963.7784680105,
            "unit": "iter/sec",
            "range": "stddev: 3.533983074262597e-8",
            "extra": "mean: 295.8611587406818 nsec\nrounds: 193051"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2207757.382957899,
            "unit": "iter/sec",
            "range": "stddev: 5.3961032071989376e-8",
            "extra": "mean: 452.94832109686956 nsec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 515684.6135339472,
            "unit": "iter/sec",
            "range": "stddev: 1.7010402312223932e-7",
            "extra": "mean: 1.9391697439779931 usec\nrounds: 52535"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1511258.6920833436,
            "unit": "iter/sec",
            "range": "stddev: 7.946483381268468e-8",
            "extra": "mean: 661.7000816858007 nsec\nrounds: 155473"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1501762.8875362226,
            "unit": "iter/sec",
            "range": "stddev: 7.958031463257215e-8",
            "extra": "mean: 665.8840808355035 nsec\nrounds: 155473"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1282881.4424179082,
            "unit": "iter/sec",
            "range": "stddev: 9.374859592469105e-8",
            "extra": "mean: 779.4952572665571 nsec\nrounds: 131148"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 442255.39880440966,
            "unit": "iter/sec",
            "range": "stddev: 1.5532909644233895e-7",
            "extra": "mean: 2.261136896696791 usec\nrounds: 45023"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2732700.697912508,
            "unit": "iter/sec",
            "range": "stddev: 4.407291616425548e-8",
            "extra": "mean: 365.9383556947715 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 524733.8999196304,
            "unit": "iter/sec",
            "range": "stddev: 1.6231949829698457e-7",
            "extra": "mean: 1.9057278368200359 usec\nrounds: 53810"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3078386.1844164445,
            "unit": "iter/sec",
            "range": "stddev: 4.2335931612821016e-8",
            "extra": "mean: 324.84553272173855 nsec\nrounds: 199204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2941173.4015893284,
            "unit": "iter/sec",
            "range": "stddev: 4.306674529028205e-8",
            "extra": "mean: 340.0003547766438 nsec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1838227.632467959,
            "unit": "iter/sec",
            "range": "stddev: 8.284886217792166e-8",
            "extra": "mean: 544.0022673674332 nsec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 502692.37626253924,
            "unit": "iter/sec",
            "range": "stddev: 1.533130206298232e-7",
            "extra": "mean: 1.9892881754740193 usec\nrounds: 51241"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36060.033853376095,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023634432043495884",
            "extra": "mean: 27.73153247903498 usec\nrounds: 37455"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1197917.0262024007,
            "unit": "iter/sec",
            "range": "stddev: 1.0032940484984374e-7",
            "extra": "mean: 834.7823581489619 nsec\nrounds: 124301"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1184965.8895220093,
            "unit": "iter/sec",
            "range": "stddev: 9.301983284083401e-8",
            "extra": "mean: 843.9061485587484 nsec\nrounds: 122175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 860417.9749839864,
            "unit": "iter/sec",
            "range": "stddev: 1.1567063480875207e-7",
            "extra": "mean: 1.162225835668529 usec\nrounds: 88881"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 356283.07989641896,
            "unit": "iter/sec",
            "range": "stddev: 4.2652122570731034e-7",
            "extra": "mean: 2.8067569200612246 usec\nrounds: 185186"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34402.02739149161,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024769456340719577",
            "extra": "mean: 29.068054292850263 usec\nrounds: 44481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2878326.9546533786,
            "unit": "iter/sec",
            "range": "stddev: 4.481110162892319e-8",
            "extra": "mean: 347.42404728669914 nsec\nrounds: 195695"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2274150.937977405,
            "unit": "iter/sec",
            "range": "stddev: 5.842718628942819e-8",
            "extra": "mean: 439.7245509523635 nsec\nrounds: 185186"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 793010.3639314619,
            "unit": "iter/sec",
            "range": "stddev: 1.0775858981156437e-7",
            "extra": "mean: 1.2610175673396933 usec\nrounds: 80496"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107647.08800596025,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010513952764215247",
            "extra": "mean: 9.28961496798345 usec\nrounds: 110048"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 381210.33780935104,
            "unit": "iter/sec",
            "range": "stddev: 4.0785628093935817e-7",
            "extra": "mean: 2.6232237188177066 usec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 375374.23356174526,
            "unit": "iter/sec",
            "range": "stddev: 3.9298676589191097e-7",
            "extra": "mean: 2.6640081033572334 usec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 300496.32447794516,
            "unit": "iter/sec",
            "range": "stddev: 4.376362170802982e-7",
            "extra": "mean: 3.3278277254715465 usec\nrounds: 154991"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 122947.93446480733,
            "unit": "iter/sec",
            "range": "stddev: 9.953915533022398e-7",
            "extra": "mean: 8.133524197482476 usec\nrounds: 127162"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2802847.698582346,
            "unit": "iter/sec",
            "range": "stddev: 4.832043816992946e-7",
            "extra": "mean: 356.7799993220433 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2912564.793230169,
            "unit": "iter/sec",
            "range": "stddev: 1.5810750895451726e-7",
            "extra": "mean: 343.34000133640075 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2020446.9253708352,
            "unit": "iter/sec",
            "range": "stddev: 1.564540670008727e-7",
            "extra": "mean: 494.93999938476924 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 416684.02853073634,
            "unit": "iter/sec",
            "range": "stddev: 9.156178944355818e-7",
            "extra": "mean: 2.399899999829813 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 47380.565438089536,
            "unit": "iter/sec",
            "range": "stddev: 0.000002561531411836761",
            "extra": "mean: 21.10570000070311 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 661835.2684249082,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020832061794076684",
            "extra": "mean: 1.510950001772926 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 877116.041544493,
            "unit": "iter/sec",
            "range": "stddev: 5.359016265888307e-7",
            "extra": "mean: 1.1401000011801443 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 742159.0890647859,
            "unit": "iter/sec",
            "range": "stddev: 4.707399776648653e-7",
            "extra": "mean: 1.3474200002860925 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 364860.96953795763,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011267315924592627",
            "extra": "mean: 2.740770001423698 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 55314.32363697242,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012242719991447836",
            "extra": "mean: 18.07850000233202 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1634177.7091735848,
            "unit": "iter/sec",
            "range": "stddev: 7.334167385893548e-8",
            "extra": "mean: 611.9285524373972 nsec\nrounds: 167758"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 365658.53699736146,
            "unit": "iter/sec",
            "range": "stddev: 3.2880636668721884e-7",
            "extra": "mean: 2.7347918859261195 usec\nrounds: 186602"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43409.63973899973,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013483867584989997",
            "extra": "mean: 23.03635796133061 usec\nrounds: 44069"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4414.813723595593,
            "unit": "iter/sec",
            "range": "stddev: 0.000005054275457296706",
            "extra": "mean: 226.5101231010856 usec\nrounds: 4476"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 213298.5820279116,
            "unit": "iter/sec",
            "range": "stddev: 5.834389521741442e-7",
            "extra": "mean: 4.688263702892986 usec\nrounds: 112020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 136557.06977866936,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010025320765579273",
            "extra": "mean: 7.322945649176511 usec\nrounds: 141985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31933.173054688796,
            "unit": "iter/sec",
            "range": "stddev: 0.000002526094649271575",
            "extra": "mean: 31.31539726062921 usec\nrounds: 33074"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3731.64051062788,
            "unit": "iter/sec",
            "range": "stddev: 0.0000056597156133321",
            "extra": "mean: 267.97865366504493 usec\nrounds: 3820"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 141597.11523071656,
            "unit": "iter/sec",
            "range": "stddev: 8.565311011454057e-7",
            "extra": "mean: 7.062290770335347 usec\nrounds: 148987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16775.325200526913,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028526273386192225",
            "extra": "mean: 59.611363001689526 usec\nrounds: 17617"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1636.927669360346,
            "unit": "iter/sec",
            "range": "stddev: 0.000012780300617084207",
            "extra": "mean: 610.9005417391259 usec\nrounds: 1713"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1256972.8634916316,
            "unit": "iter/sec",
            "range": "stddev: 7.854808679472721e-8",
            "extra": "mean: 795.5621231330388 nsec\nrounds: 127001"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104798.43311365077,
            "unit": "iter/sec",
            "range": "stddev: 8.752092441242763e-7",
            "extra": "mean: 9.542127399133246 usec\nrounds: 106861"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17084.480614155837,
            "unit": "iter/sec",
            "range": "stddev: 0.00000247614445251926",
            "extra": "mean: 58.53265443559469 usec\nrounds: 17360"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2359.6615948603217,
            "unit": "iter/sec",
            "range": "stddev: 0.000006378604988229158",
            "extra": "mean: 423.7895815985403 usec\nrounds: 2402"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 702075.7836311075,
            "unit": "iter/sec",
            "range": "stddev: 1.169536754193411e-7",
            "extra": "mean: 1.424347660630149 usec\nrounds: 71814"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 103029.98546330382,
            "unit": "iter/sec",
            "range": "stddev: 9.914045463135057e-7",
            "extra": "mean: 9.70591226916333 usec\nrounds: 106519"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21073.118220794724,
            "unit": "iter/sec",
            "range": "stddev: 0.000003238620370350699",
            "extra": "mean: 47.45382195090667 usec\nrounds: 22286"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4236.9990534051185,
            "unit": "iter/sec",
            "range": "stddev: 0.00003394362721778987",
            "extra": "mean: 236.0161018200694 usec\nrounds: 4616"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2139912.337292286,
            "unit": "iter/sec",
            "range": "stddev: 5.737260429705687e-8",
            "extra": "mean: 467.30886241135096 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 985259.229985396,
            "unit": "iter/sec",
            "range": "stddev: 8.68696551479096e-8",
            "extra": "mean: 1.0149613112630542 usec\nrounds: 100422"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 365372.07707682514,
            "unit": "iter/sec",
            "range": "stddev: 3.431390450465317e-7",
            "extra": "mean: 2.736936024231908 usec\nrounds: 186602"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 125793.47544859916,
            "unit": "iter/sec",
            "range": "stddev: 9.255202954577943e-7",
            "extra": "mean: 7.949537894822001 usec\nrounds: 129635"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 860891.6239888114,
            "unit": "iter/sec",
            "range": "stddev: 1.0695420054519432e-7",
            "extra": "mean: 1.1615863973291332 usec\nrounds: 88262"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 696405.247738042,
            "unit": "iter/sec",
            "range": "stddev: 1.1463774916519875e-7",
            "extra": "mean: 1.43594552632687 usec\nrounds: 71809"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 444267.8536857704,
            "unit": "iter/sec",
            "range": "stddev: 1.5218001959681094e-7",
            "extra": "mean: 2.2508943460655915 usec\nrounds: 45455"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 166312.5261520971,
            "unit": "iter/sec",
            "range": "stddev: 0.000001031138832778957",
            "extra": "mean: 6.012776205957417 usec\nrounds: 187266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5500162.434379756,
            "unit": "iter/sec",
            "range": "stddev: 1.1956192098833463e-8",
            "extra": "mean: 181.81281224518634 nsec\nrounds: 55237"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 422032.62220687687,
            "unit": "iter/sec",
            "range": "stddev: 1.7661464426598044e-7",
            "extra": "mean: 2.369485076226673 usec\nrounds: 43682"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 52013.61822637372,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012698865972163598",
            "extra": "mean: 19.225734223060563 usec\nrounds: 53955"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4224.3526507045035,
            "unit": "iter/sec",
            "range": "stddev: 0.000006771579990610197",
            "extra": "mean: 236.72266088704222 usec\nrounds: 4326"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4392735.607802564,
            "unit": "iter/sec",
            "range": "stddev: 1.348634632956153e-8",
            "extra": "mean: 227.64857466582117 nsec\nrounds: 45021"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 406081.44389359927,
            "unit": "iter/sec",
            "range": "stddev: 1.7996689469264944e-7",
            "extra": "mean: 2.4625601958360437 usec\nrounds: 41973"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2484003.628162982,
            "unit": "iter/sec",
            "range": "stddev: 4.7694418507035985e-8",
            "extra": "mean: 402.5759015253645 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48757.84532862581,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022118477535044137",
            "extra": "mean: 20.509519919513313 usec\nrounds: 51081"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 115151.74156463459,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011149603270675832",
            "extra": "mean: 8.68419345128793 usec\nrounds: 121581"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13762.515538935573,
            "unit": "iter/sec",
            "range": "stddev: 0.0000031112223945418394",
            "extra": "mean: 72.66113503529911 usec\nrounds: 14337"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1314.290343998688,
            "unit": "iter/sec",
            "range": "stddev: 0.00000929841340177584",
            "extra": "mean: 760.8668849818455 usec\nrounds: 1365"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51922.5098072798,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018979463406662207",
            "extra": "mean: 19.25946961561929 usec\nrounds: 54189"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49590.72956589336,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019394790181922656",
            "extra": "mean: 20.16505925106943 usec\nrounds: 51611"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35374.4208935395,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025618237034331065",
            "extra": "mean: 28.269014014661426 usec\nrounds: 36819"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 68411.13213445539,
            "unit": "iter/sec",
            "range": "stddev: 0.000003056644977268309",
            "extra": "mean: 14.61750403479068 usec\nrounds: 146349"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 244752.97087916004,
            "unit": "iter/sec",
            "range": "stddev: 5.745246842589277e-7",
            "extra": "mean: 4.085752244019632 usec\nrounds: 128453"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 31337.173044819865,
            "unit": "iter/sec",
            "range": "stddev: 0.000008293718458274033",
            "extra": "mean: 31.910983118028994 usec\nrounds: 48691"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 31582.157639458703,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021718460299770674",
            "extra": "mean: 31.66344780543434 usec\nrounds: 32695"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8589.490094691604,
            "unit": "iter/sec",
            "range": "stddev: 0.00003294367759476007",
            "extra": "mean: 116.4213462005167 usec\nrounds: 12305"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3080.260438683711,
            "unit": "iter/sec",
            "range": "stddev: 0.000006498397909297414",
            "extra": "mean: 324.64787309586404 usec\nrounds: 3152"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 17224.123695401653,
            "unit": "iter/sec",
            "range": "stddev: 0.000010609573400238159",
            "extra": "mean: 58.058106042687754 usec\nrounds: 22142"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28151.85565680643,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021171130046572994",
            "extra": "mean: 35.52163708818336 usec\nrounds: 29109"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 29685.83390484516,
            "unit": "iter/sec",
            "range": "stddev: 0.00000904399153420254",
            "extra": "mean: 33.68610102735855 usec\nrounds: 47215"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24210.176949865574,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022190664933032627",
            "extra": "mean: 41.30494387012534 usec\nrounds: 25067"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 27856.974846179488,
            "unit": "iter/sec",
            "range": "stddev: 0.0000054937935611524884",
            "extra": "mean: 35.89765240202122 usec\nrounds: 38884"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22997.562990967017,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020946648954486753",
            "extra": "mean: 43.482868180110216 usec\nrounds: 23608"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 15713.345516177058,
            "unit": "iter/sec",
            "range": "stddev: 0.000013511532764875317",
            "extra": "mean: 63.640171278006285 usec\nrounds: 20674"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21060.3350802685,
            "unit": "iter/sec",
            "range": "stddev: 0.000002499456563425803",
            "extra": "mean: 47.482625332818344 usec\nrounds: 21411"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 35901.73965608319,
            "unit": "iter/sec",
            "range": "stddev: 0.0000036360626779582562",
            "extra": "mean: 27.853803452963316 usec\nrounds: 54964"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 46975.39111773761,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015187370097440791",
            "extra": "mean: 21.2877418624069 usec\nrounds: 48478"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4610.423731808945,
            "unit": "iter/sec",
            "range": "stddev: 0.0000687104157827733",
            "extra": "mean: 216.89980317875037 usec\nrounds: 6605"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 895.3985621637032,
            "unit": "iter/sec",
            "range": "stddev: 0.000012115955431252311",
            "extra": "mean: 1.116821092032503 msec\nrounds: 891"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1055.2269839916346,
            "unit": "iter/sec",
            "range": "stddev: 0.00013568134051814002",
            "extra": "mean: 947.6634081297599 usec\nrounds: 1230"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 279.5062465062237,
            "unit": "iter/sec",
            "range": "stddev: 0.000029067419393716687",
            "extra": "mean: 3.577737572951641 msec\nrounds: 281"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 53496.68464511277,
            "unit": "iter/sec",
            "range": "stddev: 0.000003577739257303397",
            "extra": "mean: 18.69274716057298 usec\nrounds: 89191"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 23083.841811240858,
            "unit": "iter/sec",
            "range": "stddev: 0.000007990395630968961",
            "extra": "mean: 43.320345381722476 usec\nrounds: 33974"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5535.98486645647,
            "unit": "iter/sec",
            "range": "stddev: 0.00006254320256174651",
            "extra": "mean: 180.63633194866196 usec\nrounds: 8185"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1696.54360514297,
            "unit": "iter/sec",
            "range": "stddev: 0.00009659372570080533",
            "extra": "mean: 589.4337150949497 usec\nrounds: 1941"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 31474.09123353559,
            "unit": "iter/sec",
            "range": "stddev: 0.00000812581462793116",
            "extra": "mean: 31.77216436783095 usec\nrounds: 48361"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1690.07622596158,
            "unit": "iter/sec",
            "range": "stddev: 0.00010505281855136644",
            "extra": "mean: 591.6892887071075 usec\nrounds: 1957"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1141.338710515676,
            "unit": "iter/sec",
            "range": "stddev: 0.00014041188840158923",
            "extra": "mean: 876.1640964128721 usec\nrounds: 1338"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/cachemcclure/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1762737879006,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5407857.468331132,
            "unit": "iter/sec",
            "range": "stddev: 1.442300659181298e-8",
            "extra": "mean: 184.91611619870628 nsec\nrounds: 53839"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4985223.211391645,
            "unit": "iter/sec",
            "range": "stddev: 1.16509614638381e-8",
            "extra": "mean: 200.59282354990788 nsec\nrounds: 50564"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3180579.6100709816,
            "unit": "iter/sec",
            "range": "stddev: 1.422558906206229e-7",
            "extra": "mean: 314.40810248338795 nsec\nrounds: 191205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 691672.8838958849,
            "unit": "iter/sec",
            "range": "stddev: 1.0660480668516615e-7",
            "extra": "mean: 1.4457701368419276 usec\nrounds: 70592"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 76032.27720827473,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012783991033945724",
            "extra": "mean: 13.15230894979913 usec\nrounds: 79152"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 411412.5954859858,
            "unit": "iter/sec",
            "range": "stddev: 1.5896732477830592e-7",
            "extra": "mean: 2.430649938703858 usec\nrounds: 42422"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 408975.8785919425,
            "unit": "iter/sec",
            "range": "stddev: 2.1784997128285052e-7",
            "extra": "mean: 2.4451319805043212 usec\nrounds: 41641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 356464.2674927565,
            "unit": "iter/sec",
            "range": "stddev: 9.503522569486866e-7",
            "extra": "mean: 2.805330270642963 usec\nrounds: 185186"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 212399.39891406064,
            "unit": "iter/sec",
            "range": "stddev: 4.984450812930621e-7",
            "extra": "mean: 4.708111252257414 usec\nrounds: 109818"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39632.551742611795,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021374143915683797",
            "extra": "mean: 25.231784380030934 usec\nrounds: 41383"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3730753.3324658875,
            "unit": "iter/sec",
            "range": "stddev: 2.995264770737985e-8",
            "extra": "mean: 268.04237934940784 nsec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3381257.8015627735,
            "unit": "iter/sec",
            "range": "stddev: 3.298430517189912e-8",
            "extra": "mean: 295.74793129848945 nsec\nrounds: 194553"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2256147.745462177,
            "unit": "iter/sec",
            "range": "stddev: 5.55656468585234e-8",
            "extra": "mean: 443.23338398885926 nsec\nrounds: 190840"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 514798.58371339337,
            "unit": "iter/sec",
            "range": "stddev: 1.218307061455511e-7",
            "extra": "mean: 1.942507286610482 usec\nrounds: 52370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1452412.7415290603,
            "unit": "iter/sec",
            "range": "stddev: 6.969449327205889e-8",
            "extra": "mean: 688.50952033592 nsec\nrounds: 148083"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1456597.5962336964,
            "unit": "iter/sec",
            "range": "stddev: 7.099588313325981e-8",
            "extra": "mean: 686.5314089393296 nsec\nrounds: 148324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1246552.3550581436,
            "unit": "iter/sec",
            "range": "stddev: 1.15210214816559e-7",
            "extra": "mean: 802.2125953573391 nsec\nrounds: 129635"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 436475.8810763861,
            "unit": "iter/sec",
            "range": "stddev: 1.4137415031353027e-7",
            "extra": "mean: 2.291077338646863 usec\nrounds: 44481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2780534.192782833,
            "unit": "iter/sec",
            "range": "stddev: 3.9300993755064555e-8",
            "extra": "mean: 359.6431227481812 nsec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 520781.1961221993,
            "unit": "iter/sec",
            "range": "stddev: 1.4334885872419867e-7",
            "extra": "mean: 1.920192217856808 usec\nrounds: 53263"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3038838.3505742466,
            "unit": "iter/sec",
            "range": "stddev: 3.848131090522204e-8",
            "extra": "mean: 329.07311434023165 nsec\nrounds: 197239"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2941863.1481830697,
            "unit": "iter/sec",
            "range": "stddev: 3.839332800197455e-8",
            "extra": "mean: 339.92063859857393 nsec\nrounds: 189790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1796343.7154413282,
            "unit": "iter/sec",
            "range": "stddev: 6.346057544185435e-8",
            "extra": "mean: 556.6863353622766 nsec\nrounds: 185186"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 495087.4464207172,
            "unit": "iter/sec",
            "range": "stddev: 1.2862694504320592e-7",
            "extra": "mean: 2.0198451954894185 usec\nrounds: 50744"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47534.601558466435,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017622800080822121",
            "extra": "mean: 21.03730687150125 usec\nrounds: 49291"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1232224.4577802254,
            "unit": "iter/sec",
            "range": "stddev: 2.0024252158374418e-7",
            "extra": "mean: 811.5404573297922 nsec\nrounds: 126343"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1219060.6595207092,
            "unit": "iter/sec",
            "range": "stddev: 7.980495324069556e-8",
            "extra": "mean: 820.3037249952034 nsec\nrounds: 125235"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 904419.1925629276,
            "unit": "iter/sec",
            "range": "stddev: 9.579999508821703e-8",
            "extra": "mean: 1.1056819760383525 usec\nrounds: 93642"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 355886.21019946743,
            "unit": "iter/sec",
            "range": "stddev: 3.64864342508164e-7",
            "extra": "mean: 2.809886900196327 usec\nrounds: 185186"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 44534.25288892746,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017109594024367051",
            "extra": "mean: 22.454626161442345 usec\nrounds: 46060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2820117.6595887034,
            "unit": "iter/sec",
            "range": "stddev: 4.225692959858269e-8",
            "extra": "mean: 354.5951342136733 nsec\nrounds: 195734"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2259053.21902937,
            "unit": "iter/sec",
            "range": "stddev: 4.9978772305917285e-8",
            "extra": "mean: 442.66332088877385 nsec\nrounds: 194175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 775708.0559511805,
            "unit": "iter/sec",
            "range": "stddev: 9.316622030562798e-8",
            "extra": "mean: 1.2891447914303493 usec\nrounds: 79158"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107912.76850906381,
            "unit": "iter/sec",
            "range": "stddev: 8.025733660059881e-7",
            "extra": "mean: 9.266743999029252 usec\nrounds: 109566"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 371782.1497776741,
            "unit": "iter/sec",
            "range": "stddev: 3.7870536807967103e-7",
            "extra": "mean: 2.6897472097517334 usec\nrounds: 193799"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 360781.44469980453,
            "unit": "iter/sec",
            "range": "stddev: 3.7704281190676415e-7",
            "extra": "mean: 2.771761172008361 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 299255.8685690421,
            "unit": "iter/sec",
            "range": "stddev: 3.9630631131157026e-7",
            "extra": "mean: 3.3416220199180064 usec\nrounds: 154274"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 120962.23795620244,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010116292359804828",
            "extra": "mean: 8.26704281349421 usec\nrounds: 125708"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2756643.5127930413,
            "unit": "iter/sec",
            "range": "stddev: 4.959261950225307e-7",
            "extra": "mean: 362.75999974577644 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2916217.1008398114,
            "unit": "iter/sec",
            "range": "stddev: 1.5294021292384287e-7",
            "extra": "mean: 342.9099979257444 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1910183.187835329,
            "unit": "iter/sec",
            "range": "stddev: 4.4945665074593144e-7",
            "extra": "mean: 523.5099996525605 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 499847.5460341776,
            "unit": "iter/sec",
            "range": "stddev: 1.7461987217790487e-7",
            "extra": "mean: 2.0006100018576944 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 50553.919289131714,
            "unit": "iter/sec",
            "range": "stddev: 0.000003864528760824502",
            "extra": "mean: 19.780860001787914 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 797842.6333145306,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014237837453380272",
            "extra": "mean: 1.2533800003211581 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 770920.8642135705,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022271272745954805",
            "extra": "mean: 1.297150001278169 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 743931.3795419057,
            "unit": "iter/sec",
            "range": "stddev: 4.6254601181357155e-7",
            "extra": "mean: 1.3442100004112945 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 363241.42124636436,
            "unit": "iter/sec",
            "range": "stddev: 5.452001514348654e-7",
            "extra": "mean: 2.752989999237343 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 49029.38968964074,
            "unit": "iter/sec",
            "range": "stddev: 0.000004355442080703313",
            "extra": "mean: 20.395929998926476 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1610968.1237314125,
            "unit": "iter/sec",
            "range": "stddev: 6.745585398137412e-8",
            "extra": "mean: 620.74474675749 nsec\nrounds: 168039"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 369493.05107288325,
            "unit": "iter/sec",
            "range": "stddev: 2.9927354399505857e-7",
            "extra": "mean: 2.7064108434416756 usec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 44815.59821508941,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013207201232400489",
            "extra": "mean: 22.313659525430587 usec\nrounds: 45557"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4525.532145591675,
            "unit": "iter/sec",
            "range": "stddev: 0.000007471259338105306",
            "extra": "mean: 220.9684889707614 usec\nrounds: 4624"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 209522.90516019892,
            "unit": "iter/sec",
            "range": "stddev: 5.20105433673763e-7",
            "extra": "mean: 4.77274787324761 usec\nrounds: 108850"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 136403.97856492773,
            "unit": "iter/sec",
            "range": "stddev: 9.079183503081298e-7",
            "extra": "mean: 7.3311644610424915 usec\nrounds: 141383"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 32123.577048897463,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018894127218329023",
            "extra": "mean: 31.129783538048468 usec\nrounds: 32888"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3725.2174990192907,
            "unit": "iter/sec",
            "range": "stddev: 0.000005824628778852767",
            "extra": "mean: 268.440701855197 usec\nrounds: 3827"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 138077.78286881218,
            "unit": "iter/sec",
            "range": "stddev: 8.007908395930379e-7",
            "extra": "mean: 7.242294735787444 usec\nrounds: 144655"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16588.757862883067,
            "unit": "iter/sec",
            "range": "stddev: 0.000002903740958881647",
            "extra": "mean: 60.28178892389978 usec\nrounds: 17136"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1586.7144753030318,
            "unit": "iter/sec",
            "range": "stddev: 0.000017445976761834853",
            "extra": "mean: 630.2331109754447 usec\nrounds: 1640"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1222445.3146772063,
            "unit": "iter/sec",
            "range": "stddev: 7.731500379531195e-8",
            "extra": "mean: 818.032502553313 nsec\nrounds: 122775"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104757.04464769065,
            "unit": "iter/sec",
            "range": "stddev: 8.306149557318007e-7",
            "extra": "mean: 9.545897398720143 usec\nrounds: 106987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17178.551508295062,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023532744194844732",
            "extra": "mean: 58.21212571485592 usec\nrounds: 17484"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2364.809071940183,
            "unit": "iter/sec",
            "range": "stddev: 0.000006341572789120585",
            "extra": "mean: 422.86711932289757 usec\nrounds: 2422"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 725877.3564016938,
            "unit": "iter/sec",
            "range": "stddev: 1.1603849274350104e-7",
            "extra": "mean: 1.3776431943781362 usec\nrounds: 74600"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 104781.42608851304,
            "unit": "iter/sec",
            "range": "stddev: 9.463820890797712e-7",
            "extra": "mean: 9.543676177448283 usec\nrounds: 107562"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21098.119570987947,
            "unit": "iter/sec",
            "range": "stddev: 0.000003158197180134797",
            "extra": "mean: 47.39758899532929 usec\nrounds: 22445"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4386.185956157015,
            "unit": "iter/sec",
            "range": "stddev: 0.000028572964418980836",
            "extra": "mean: 227.9885098342151 usec\nrounds: 4576"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2125068.00105032,
            "unit": "iter/sec",
            "range": "stddev: 5.379179242366683e-8",
            "extra": "mean: 470.5731767200529 nsec\nrounds: 198020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 974940.0315632516,
            "unit": "iter/sec",
            "range": "stddev: 8.415176860986764e-8",
            "extra": "mean: 1.0257041126894113 usec\nrounds: 99424"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 363455.8777749604,
            "unit": "iter/sec",
            "range": "stddev: 3.274033050013444e-7",
            "extra": "mean: 2.7513656021245203 usec\nrounds: 185840"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 123812.71981278139,
            "unit": "iter/sec",
            "range": "stddev: 8.781004855670769e-7",
            "extra": "mean: 8.076714585642828 usec\nrounds: 128140"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 895839.7212883823,
            "unit": "iter/sec",
            "range": "stddev: 9.526953390295581e-8",
            "extra": "mean: 1.1162711099278113 usec\nrounds: 91997"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 724025.0136262155,
            "unit": "iter/sec",
            "range": "stddev: 2.0080689191956952e-7",
            "extra": "mean: 1.3811677513620657 usec\nrounds: 75104"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 457984.39134135196,
            "unit": "iter/sec",
            "range": "stddev: 1.3276757671434062e-7",
            "extra": "mean: 2.1834805266423336 usec\nrounds: 46751"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 179699.2042756264,
            "unit": "iter/sec",
            "range": "stddev: 8.444434477007326e-7",
            "extra": "mean: 5.56485491424981 usec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5617683.566339147,
            "unit": "iter/sec",
            "range": "stddev: 1.0944776605424792e-8",
            "extra": "mean: 178.00931437143333 nsec\nrounds: 57101"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 408502.80862480745,
            "unit": "iter/sec",
            "range": "stddev: 1.5501674994163738e-7",
            "extra": "mean: 2.44796358528457 usec\nrounds: 42546"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54869.91273441131,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012241632205131438",
            "extra": "mean: 18.224924191885155 usec\nrounds: 56234"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4098.939228573722,
            "unit": "iter/sec",
            "range": "stddev: 0.000007313468065684328",
            "extra": "mean: 243.965558949739 usec\nrounds: 4190"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4450295.464699904,
            "unit": "iter/sec",
            "range": "stddev: 1.2430453090986792e-8",
            "extra": "mean: 224.70418153850045 nsec\nrounds: 45225"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 388146.22237719904,
            "unit": "iter/sec",
            "range": "stddev: 2.8063829082667916e-7",
            "extra": "mean: 2.5763486602433074 usec\nrounds: 135796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2426304.3268755665,
            "unit": "iter/sec",
            "range": "stddev: 4.975141338266037e-8",
            "extra": "mean: 412.14945253284367 nsec\nrounds: 191939"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48997.2439342532,
            "unit": "iter/sec",
            "range": "stddev: 0.000001858533868166851",
            "extra": "mean: 20.409311212317306 usec\nrounds: 50721"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 112002.62977449562,
            "unit": "iter/sec",
            "range": "stddev: 9.50275570155365e-7",
            "extra": "mean: 8.928361789481055 usec\nrounds: 117151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13650.769037639082,
            "unit": "iter/sec",
            "range": "stddev: 0.000003337780801679599",
            "extra": "mean: 73.25594603811064 usec\nrounds: 14121"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1300.422706818822,
            "unit": "iter/sec",
            "range": "stddev: 0.000013408073502876956",
            "extra": "mean: 768.9807281558968 usec\nrounds: 1339"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51393.150794832414,
            "unit": "iter/sec",
            "range": "stddev: 0.000001762278308914737",
            "extra": "mean: 19.457845734971947 usec\nrounds: 53434"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49234.39919187093,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017751081309008437",
            "extra": "mean: 20.3110023969808 usec\nrounds: 50899"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35168.407437097536,
            "unit": "iter/sec",
            "range": "stddev: 0.000002362999343474009",
            "extra": "mean: 28.434611427560583 usec\nrounds: 37051"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 64968.219838222896,
            "unit": "iter/sec",
            "range": "stddev: 0.0000037046430238945083",
            "extra": "mean: 15.392140995860686 usec\nrounds: 116741"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 249264.91261949067,
            "unit": "iter/sec",
            "range": "stddev: 4.436554625017695e-7",
            "extra": "mean: 4.011796082694262 usec\nrounds: 130141"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 34148.82813643012,
            "unit": "iter/sec",
            "range": "stddev: 0.0000036059699552399185",
            "extra": "mean: 29.283581738290916 usec\nrounds: 46863"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32541.256765471946,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020354789125229433",
            "extra": "mean: 30.7302206306013 usec\nrounds: 33019"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8636.01513619784,
            "unit": "iter/sec",
            "range": "stddev: 0.00004146892682059983",
            "extra": "mean: 115.7941462849575 usec\nrounds: 12045"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3144.802573960786,
            "unit": "iter/sec",
            "range": "stddev: 0.0000066864139807972235",
            "extra": "mean: 317.9849852197652 usec\nrounds: 3180"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 16155.442865548583,
            "unit": "iter/sec",
            "range": "stddev: 0.000015047987523712094",
            "extra": "mean: 61.89864359165888 usec\nrounds: 23431"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28187.139961632194,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019083496565374315",
            "extra": "mean: 35.47717155274289 usec\nrounds: 28790"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 32567.566657912605,
            "unit": "iter/sec",
            "range": "stddev: 0.0000036109391415429846",
            "extra": "mean: 30.70539504851341 usec\nrounds: 46169"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24314.976469404133,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021120316018213073",
            "extra": "mean: 41.12691621389449 usec\nrounds: 25219"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28191.32328449619,
            "unit": "iter/sec",
            "range": "stddev: 0.000005152628789453469",
            "extra": "mean: 35.47190707964921 usec\nrounds: 39593"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23217.314660796394,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021549266235567234",
            "extra": "mean: 43.07130323252027 usec\nrounds: 23698"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16475.802368692923,
            "unit": "iter/sec",
            "range": "stddev: 0.000008755830009461492",
            "extra": "mean: 60.69507133080118 usec\nrounds: 21169"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21342.974098891984,
            "unit": "iter/sec",
            "range": "stddev: 0.000002323078070380945",
            "extra": "mean: 46.853826245889266 usec\nrounds: 21870"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 36550.05074476136,
            "unit": "iter/sec",
            "range": "stddev: 0.0000030030886094825604",
            "extra": "mean: 27.359743136425816 usec\nrounds: 55146"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47386.9928658235,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014447970303444997",
            "extra": "mean: 21.102837287681556 usec\nrounds: 49437"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4448.639250447013,
            "unit": "iter/sec",
            "range": "stddev: 0.00008310979275220392",
            "extra": "mean: 224.7878381910866 usec\nrounds: 6347"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 909.3236573879614,
            "unit": "iter/sec",
            "range": "stddev: 0.00003240663004468556",
            "extra": "mean: 1.099718446644737 msec\nrounds: 909"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1025.4144150620891,
            "unit": "iter/sec",
            "range": "stddev: 0.00016120682044963626",
            "extra": "mean: 975.2154692885312 usec\nrounds: 1221"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 275.9512277662433,
            "unit": "iter/sec",
            "range": "stddev: 0.00006820801512894322",
            "extra": "mean: 3.623828776174514 msec\nrounds: 277"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 52670.6662224586,
            "unit": "iter/sec",
            "range": "stddev: 0.000003835071430783058",
            "extra": "mean: 18.985899965199287 usec\nrounds: 86200"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 23743.613325160655,
            "unit": "iter/sec",
            "range": "stddev: 0.000007675077053958857",
            "extra": "mean: 42.11658884034803 usec\nrounds: 34033"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5719.576456943272,
            "unit": "iter/sec",
            "range": "stddev: 0.00005945908599731749",
            "extra": "mean: 174.83812088674 usec\nrounds: 8115"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1739.3660824639994,
            "unit": "iter/sec",
            "range": "stddev: 0.00009284679469884236",
            "extra": "mean: 574.9220995406512 usec\nrounds: 1959"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 32238.203602355727,
            "unit": "iter/sec",
            "range": "stddev: 0.0000056704671317511705",
            "extra": "mean: 31.0190980966113 usec\nrounds: 48126"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1698.6798544986902,
            "unit": "iter/sec",
            "range": "stddev: 0.00009834413953936722",
            "extra": "mean: 588.6924468737622 usec\nrounds: 1967"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1138.7734389404443,
            "unit": "iter/sec",
            "range": "stddev: 0.00014717987388714727",
            "extra": "mean: 878.1377979191681 usec\nrounds: 1346"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/cachemcclure/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1763342559463,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5329674.9772315165,
            "unit": "iter/sec",
            "range": "stddev: 1.157945917218737e-8",
            "extra": "mean: 187.62870236411192 nsec\nrounds: 53374"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5014387.236057983,
            "unit": "iter/sec",
            "range": "stddev: 1.4304656427965606e-8",
            "extra": "mean: 199.42616174692535 nsec\nrounds: 50437"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3206612.412198744,
            "unit": "iter/sec",
            "range": "stddev: 3.6395759302215105e-8",
            "extra": "mean: 311.85558822014053 nsec\nrounds: 192679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 700149.89110173,
            "unit": "iter/sec",
            "range": "stddev: 9.939530001953376e-8",
            "extra": "mean: 1.4282655938522766 usec\nrounds: 70842"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 72514.45582259848,
            "unit": "iter/sec",
            "range": "stddev: 0.000003403444704175331",
            "extra": "mean: 13.79035378058176 usec\nrounds: 78902"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 404213.2125474839,
            "unit": "iter/sec",
            "range": "stddev: 2.0223496959255485e-7",
            "extra": "mean: 2.47394189244255 usec\nrounds: 41745"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 399698.5535659283,
            "unit": "iter/sec",
            "range": "stddev: 1.8638716831896147e-7",
            "extra": "mean: 2.501885461126753 usec\nrounds: 41193"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 352741.12597353285,
            "unit": "iter/sec",
            "range": "stddev: 4.145688988545962e-7",
            "extra": "mean: 2.834940205058575 usec\nrounds: 183151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 210567.38029142324,
            "unit": "iter/sec",
            "range": "stddev: 5.435980621594364e-7",
            "extra": "mean: 4.7490736628627355 usec\nrounds: 109207"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 38883.19948060968,
            "unit": "iter/sec",
            "range": "stddev: 0.000002896567440121284",
            "extra": "mean: 25.718048240826512 usec\nrounds: 41127"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3659499.062716288,
            "unit": "iter/sec",
            "range": "stddev: 3.451414238665209e-8",
            "extra": "mean: 273.26144449356093 nsec\nrounds: 197668"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3408840.0644954755,
            "unit": "iter/sec",
            "range": "stddev: 3.377009324054821e-8",
            "extra": "mean: 293.3549187051876 nsec\nrounds: 194970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2216144.1298270887,
            "unit": "iter/sec",
            "range": "stddev: 5.4302754480542544e-8",
            "extra": "mean: 451.2341893927237 nsec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 514354.5280239807,
            "unit": "iter/sec",
            "range": "stddev: 1.3071704646692792e-7",
            "extra": "mean: 1.9441843038531454 usec\nrounds: 52395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1417128.8919671434,
            "unit": "iter/sec",
            "range": "stddev: 8.055025332731827e-8",
            "extra": "mean: 705.6521151099859 nsec\nrounds: 145075"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1387772.784828583,
            "unit": "iter/sec",
            "range": "stddev: 9.645035861945462e-8",
            "extra": "mean: 720.5790536694522 nsec\nrounds: 142593"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1209800.2099652577,
            "unit": "iter/sec",
            "range": "stddev: 8.70486095592335e-8",
            "extra": "mean: 826.582762809001 nsec\nrounds: 124301"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 432116.8275097025,
            "unit": "iter/sec",
            "range": "stddev: 1.5497442577589748e-7",
            "extra": "mean: 2.3141889793161785 usec\nrounds: 44108"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2685871.2086191573,
            "unit": "iter/sec",
            "range": "stddev: 4.983737728386332e-8",
            "extra": "mean: 372.318671420625 nsec\nrounds: 197668"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 495721.97831609705,
            "unit": "iter/sec",
            "range": "stddev: 2.0114049849099296e-7",
            "extra": "mean: 2.017259762007897 usec\nrounds: 50927"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3031702.982282898,
            "unit": "iter/sec",
            "range": "stddev: 4.0713251070990975e-8",
            "extra": "mean: 329.84761562855726 nsec\nrounds: 196079"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2893253.722067596,
            "unit": "iter/sec",
            "range": "stddev: 4.3983184044419966e-8",
            "extra": "mean: 345.63163001323363 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1817355.0919979822,
            "unit": "iter/sec",
            "range": "stddev: 1.2009981375161488e-7",
            "extra": "mean: 550.25019843572 nsec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 502049.28537810827,
            "unit": "iter/sec",
            "range": "stddev: 1.3456571887557777e-7",
            "extra": "mean: 1.99183631791617 usec\nrounds: 51107"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 35634.60679614137,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023695956998398403",
            "extra": "mean: 28.06260795077113 usec\nrounds: 48499"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1187935.7124042104,
            "unit": "iter/sec",
            "range": "stddev: 9.678540634347589e-8",
            "extra": "mean: 841.796394837021 nsec\nrounds: 120993"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1165438.9938001942,
            "unit": "iter/sec",
            "range": "stddev: 8.992595346785503e-8",
            "extra": "mean: 858.0457710097043 nsec\nrounds: 120395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 899026.4454273924,
            "unit": "iter/sec",
            "range": "stddev: 1.0556065479902093e-7",
            "extra": "mean: 1.1123143318933255 usec\nrounds: 92507"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 360814.31980364467,
            "unit": "iter/sec",
            "range": "stddev: 4.048808776670174e-7",
            "extra": "mean: 2.771508626775679 usec\nrounds: 187266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34378.09529709318,
            "unit": "iter/sec",
            "range": "stddev: 0.000002328722298617282",
            "extra": "mean: 29.08828983566621 usec\nrounds: 44125"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2873795.6801319625,
            "unit": "iter/sec",
            "range": "stddev: 4.1874452420459505e-8",
            "extra": "mean: 347.9718502305533 nsec\nrounds: 195734"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2317156.5435340446,
            "unit": "iter/sec",
            "range": "stddev: 5.655481421975911e-8",
            "extra": "mean: 431.5634188766684 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 777379.8349969665,
            "unit": "iter/sec",
            "range": "stddev: 9.880325197027186e-8",
            "extra": "mean: 1.2863724462365815 usec\nrounds: 79089"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 108097.19233219976,
            "unit": "iter/sec",
            "range": "stddev: 8.155363251785101e-7",
            "extra": "mean: 9.250934075390616 usec\nrounds: 109686"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 363260.7367139859,
            "unit": "iter/sec",
            "range": "stddev: 4.556506812813191e-7",
            "extra": "mean: 2.752843615981961 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 355744.45575403183,
            "unit": "iter/sec",
            "range": "stddev: 4.364027879206585e-7",
            "extra": "mean: 2.8110065633501202 usec\nrounds: 183824"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 288075.2643013421,
            "unit": "iter/sec",
            "range": "stddev: 4.797151857756561e-7",
            "extra": "mean: 3.4713150482574813 usec\nrounds: 149858"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 131078.31040727685,
            "unit": "iter/sec",
            "range": "stddev: 9.67303919377845e-7",
            "extra": "mean: 7.629027234886336 usec\nrounds: 134717"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2729034.1957532624,
            "unit": "iter/sec",
            "range": "stddev: 5.933001385437043e-7",
            "extra": "mean: 366.4299998717979 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2824938.5566168916,
            "unit": "iter/sec",
            "range": "stddev: 1.5762405613186508e-7",
            "extra": "mean: 353.9900001214846 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1948406.2092143013,
            "unit": "iter/sec",
            "range": "stddev: 1.480566450773111e-7",
            "extra": "mean: 513.2399985541269 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 506731.93353001366,
            "unit": "iter/sec",
            "range": "stddev: 1.4825995719900926e-7",
            "extra": "mean: 1.9734300008167338 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 49874.71472302177,
            "unit": "iter/sec",
            "range": "stddev: 0.000004031815018327972",
            "extra": "mean: 20.050239997431163 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 814385.3016973262,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013684795703908281",
            "extra": "mean: 1.2279200004172708 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 889497.6998260245,
            "unit": "iter/sec",
            "range": "stddev: 4.870107090167521e-7",
            "extra": "mean: 1.1242300010394501 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 648277.2036382587,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017221465471430736",
            "extra": "mean: 1.542549999271614 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 392580.23333677475,
            "unit": "iter/sec",
            "range": "stddev: 5.372434024149678e-7",
            "extra": "mean: 2.5472500016121558 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 48236.451226424804,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028206016296317634",
            "extra": "mean: 20.731209999382827 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1615724.3475732554,
            "unit": "iter/sec",
            "range": "stddev: 7.305492706895208e-8",
            "extra": "mean: 618.9174542686967 nsec\nrounds: 167449"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 371198.3754328926,
            "unit": "iter/sec",
            "range": "stddev: 3.28861309597614e-7",
            "extra": "mean: 2.6939773075078715 usec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 44709.1395499845,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015743595885891805",
            "extra": "mean: 22.366791445002136 usec\nrounds: 45494"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4487.494697262987,
            "unit": "iter/sec",
            "range": "stddev: 0.000017999376668763642",
            "extra": "mean: 222.84148895149002 usec\nrounds: 4616"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 204929.2820828067,
            "unit": "iter/sec",
            "range": "stddev: 5.951035865693912e-7",
            "extra": "mean: 4.879732119472929 usec\nrounds: 106191"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 132092.58747560586,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010332145958250876",
            "extra": "mean: 7.570447510422753 usec\nrounds: 136370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31809.81923106765,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019996936301438716",
            "extra": "mean: 31.43683378820749 usec\nrounds: 32609"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3730.8822267080336,
            "unit": "iter/sec",
            "range": "stddev: 0.000006162610454885204",
            "extra": "mean: 268.03311904121836 usec\nrounds: 3839"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 139668.60766458447,
            "unit": "iter/sec",
            "range": "stddev: 8.479667286642581e-7",
            "extra": "mean: 7.159805032219622 usec\nrounds: 144865"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16807.224602477552,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032652646145312015",
            "extra": "mean: 59.4982231541423 usec\nrounds: 17051"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1602.0841955604774,
            "unit": "iter/sec",
            "range": "stddev: 0.00005228070881618233",
            "extra": "mean: 624.1869202449484 usec\nrounds: 1630"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1204940.685484795,
            "unit": "iter/sec",
            "range": "stddev: 8.883999435228031e-8",
            "extra": "mean: 829.9163701968293 nsec\nrounds: 130481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 74084.00989218047,
            "unit": "iter/sec",
            "range": "stddev: 0.00000172476385485923",
            "extra": "mean: 13.49818944000694 usec\nrounds: 107216"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 16960.794643492933,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025987996954882428",
            "extra": "mean: 58.959501663659 usec\nrounds: 17432"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2351.5780916064537,
            "unit": "iter/sec",
            "range": "stddev: 0.000006974647985138507",
            "extra": "mean: 425.24634991681756 usec\nrounds: 2412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 701484.8127999604,
            "unit": "iter/sec",
            "range": "stddev: 1.3218692214110245e-7",
            "extra": "mean: 1.4255476123688684 usec\nrounds: 73336"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 102945.81246009836,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010183967751554982",
            "extra": "mean: 9.713848247956648 usec\nrounds: 106417"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 22214.154949909713,
            "unit": "iter/sec",
            "range": "stddev: 0.000002907549493688495",
            "extra": "mean: 45.01634215908197 usec\nrounds: 23530"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4304.259078423021,
            "unit": "iter/sec",
            "range": "stddev: 0.00006013281180993583",
            "extra": "mean: 232.32802249588948 usec\nrounds: 4623"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2111835.403133799,
            "unit": "iter/sec",
            "range": "stddev: 5.949530719029517e-8",
            "extra": "mean: 473.5217519869636 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 989861.0738395958,
            "unit": "iter/sec",
            "range": "stddev: 9.49372077771882e-8",
            "extra": "mean: 1.0102427769192728 usec\nrounds: 101854"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 366420.20550776063,
            "unit": "iter/sec",
            "range": "stddev: 3.3410195878121723e-7",
            "extra": "mean: 2.729107142479402 usec\nrounds: 186602"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 117085.43758707796,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010937396236777077",
            "extra": "mean: 8.540771769813706 usec\nrounds: 122911"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 888168.9438085214,
            "unit": "iter/sec",
            "range": "stddev: 1.0671422115553712e-7",
            "extra": "mean: 1.1259119190904556 usec\nrounds: 91408"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 718601.0128705326,
            "unit": "iter/sec",
            "range": "stddev: 1.248810991255893e-7",
            "extra": "mean: 1.3915928061461762 usec\nrounds: 73660"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 494782.7537949324,
            "unit": "iter/sec",
            "range": "stddev: 1.8859920045079336e-7",
            "extra": "mean: 2.0210890382296594 usec\nrounds: 50667"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 160149.36057202873,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010491913449614082",
            "extra": "mean: 6.244171044006388 usec\nrounds: 183824"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5580887.1710436335,
            "unit": "iter/sec",
            "range": "stddev: 1.3000295980996686e-8",
            "extra": "mean: 179.18298101209407 nsec\nrounds: 57561"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 402859.32581925497,
            "unit": "iter/sec",
            "range": "stddev: 2.416075908377128e-7",
            "extra": "mean: 2.4822560529445505 usec\nrounds: 41641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 49190.98361041669,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013457641238396535",
            "extra": "mean: 20.32892873051312 usec\nrounds: 55578"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 3990.793379381568,
            "unit": "iter/sec",
            "range": "stddev: 0.000013333494486121286",
            "extra": "mean: 250.57674124811862 usec\nrounds: 4085"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4505710.5416536,
            "unit": "iter/sec",
            "range": "stddev: 1.280384001099666e-8",
            "extra": "mean: 221.94057757486536 nsec\nrounds: 46020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 380427.79920128937,
            "unit": "iter/sec",
            "range": "stddev: 3.9211300247542304e-7",
            "extra": "mean: 2.628619680526782 usec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2553045.4295400665,
            "unit": "iter/sec",
            "range": "stddev: 4.9635386837638794e-8",
            "extra": "mean: 391.6890739308733 nsec\nrounds: 199601"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 49120.47669513136,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020111772170100934",
            "extra": "mean: 20.358108619477555 usec\nrounds: 51372"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 113211.25431167685,
            "unit": "iter/sec",
            "range": "stddev: 0.000001276121855732042",
            "extra": "mean: 8.83304408276358 usec\nrounds: 120410"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13817.200778087654,
            "unit": "iter/sec",
            "range": "stddev: 0.0000036507506406568435",
            "extra": "mean: 72.3735593091963 usec\nrounds: 14416"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1325.8352110335952,
            "unit": "iter/sec",
            "range": "stddev: 0.000014288035945798261",
            "extra": "mean: 754.24154651951 usec\nrounds: 1365"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51373.370998797654,
            "unit": "iter/sec",
            "range": "stddev: 0.000002085846837879344",
            "extra": "mean: 19.465337402589448 usec\nrounds: 53491"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49091.234977713575,
            "unit": "iter/sec",
            "range": "stddev: 0.00000206528682967794",
            "extra": "mean: 20.370235143890344 usec\nrounds: 51241"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35402.8314052947,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024819380449155743",
            "extra": "mean: 28.246328338880947 usec\nrounds: 37175"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 67821.9588821537,
            "unit": "iter/sec",
            "range": "stddev: 0.000004884804327696287",
            "extra": "mean: 14.744487131927038 usec\nrounds: 124767"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 248007.75682195977,
            "unit": "iter/sec",
            "range": "stddev: 4.672030847163044e-7",
            "extra": "mean: 4.0321319494772165 usec\nrounds: 128966"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 31904.496980003387,
            "unit": "iter/sec",
            "range": "stddev: 0.000007398417375865475",
            "extra": "mean: 31.343543846711164 usec\nrounds: 51110"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32445.78004190333,
            "unit": "iter/sec",
            "range": "stddev: 0.000005093097354143736",
            "extra": "mean: 30.820649055393712 usec\nrounds: 32877"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8775.127472510934,
            "unit": "iter/sec",
            "range": "stddev: 0.000035645437265566334",
            "extra": "mean: 113.95845851044461 usec\nrounds: 12015"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3128.9904176383393,
            "unit": "iter/sec",
            "range": "stddev: 0.000007921480826634606",
            "extra": "mean: 319.5919023474567 usec\nrounds: 3195"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 18003.415099988204,
            "unit": "iter/sec",
            "range": "stddev: 0.0000078748498850111",
            "extra": "mean: 55.545017122926595 usec\nrounds: 23010"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28632.7394583362,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019378042502946305",
            "extra": "mean: 34.92505498662154 usec\nrounds: 29298"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 31388.10642965389,
            "unit": "iter/sec",
            "range": "stddev: 0.000008066626718550195",
            "extra": "mean: 31.859201262783113 usec\nrounds: 47987"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24941.088928748504,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019806768382969613",
            "extra": "mean: 40.094480351551276 usec\nrounds: 25600"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28006.297754598,
            "unit": "iter/sec",
            "range": "stddev: 0.000007813636575539456",
            "extra": "mean: 35.70625467037401 usec\nrounds: 40790"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22755.129141934034,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019714197803168603",
            "extra": "mean: 43.94613600136249 usec\nrounds: 23213"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 15895.1417064265,
            "unit": "iter/sec",
            "range": "stddev: 0.000012482132918454407",
            "extra": "mean: 62.91230480793349 usec\nrounds: 20987"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21216.388602013623,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025646175705127492",
            "extra": "mean: 47.13337499413501 usec\nrounds: 21619"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 30177.088062051454,
            "unit": "iter/sec",
            "range": "stddev: 0.000008485697359613002",
            "extra": "mean: 33.13772349219898 usec\nrounds: 56870"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47658.950020890356,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015245969712611607",
            "extra": "mean: 20.982417773821492 usec\nrounds: 49759"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4490.765107697836,
            "unit": "iter/sec",
            "range": "stddev: 0.00007778667770859132",
            "extra": "mean: 222.67920410396255 usec\nrounds: 6384"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 910.876182589735,
            "unit": "iter/sec",
            "range": "stddev: 0.00001708435628528213",
            "extra": "mean: 1.0978440529171318 msec\nrounds: 926"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1043.741344096528,
            "unit": "iter/sec",
            "range": "stddev: 0.00015296554763529724",
            "extra": "mean: 958.0917778682121 usec\nrounds: 1238"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 278.3232296226987,
            "unit": "iter/sec",
            "range": "stddev: 0.000024783932647483155",
            "extra": "mean: 3.5929447978726845 msec\nrounds: 282"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 53339.077886779494,
            "unit": "iter/sec",
            "range": "stddev: 0.000004016315557726275",
            "extra": "mean: 18.74798064793426 usec\nrounds: 85676"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 23341.626389489986,
            "unit": "iter/sec",
            "range": "stddev: 0.000008846948343721551",
            "extra": "mean: 42.84191612501643 usec\nrounds: 34301"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5558.768087675797,
            "unit": "iter/sec",
            "range": "stddev: 0.00006233802421154893",
            "extra": "mean: 179.89597411287485 usec\nrounds: 8344"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1703.4770271226373,
            "unit": "iter/sec",
            "range": "stddev: 0.00009473253878526829",
            "extra": "mean: 587.0346262838141 usec\nrounds: 1948"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 32916.104031643255,
            "unit": "iter/sec",
            "range": "stddev: 0.0000039563322590843255",
            "extra": "mean: 30.38026611650849 usec\nrounds: 47017"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1708.8740803553544,
            "unit": "iter/sec",
            "range": "stddev: 0.00009477018516235018",
            "extra": "mean: 585.1806236022104 usec\nrounds: 1966"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1145.985430358859,
            "unit": "iter/sec",
            "range": "stddev: 0.00014007113193986335",
            "extra": "mean: 872.6114429629837 usec\nrounds: 1350"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1763947597382,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5635369.505335002,
            "unit": "iter/sec",
            "range": "stddev: 1.5418108589039944e-8",
            "extra": "mean: 177.45065324523864 nsec\nrounds: 57972"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5235728.468602534,
            "unit": "iter/sec",
            "range": "stddev: 1.0028608475563582e-8",
            "extra": "mean: 190.99538984818852 nsec\nrounds: 53534"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3014755.4120771466,
            "unit": "iter/sec",
            "range": "stddev: 0.000001347217363859719",
            "extra": "mean: 331.7018674198205 nsec\nrounds: 195925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 583433.6502074276,
            "unit": "iter/sec",
            "range": "stddev: 7.691291745987758e-8",
            "extra": "mean: 1.7139909562029347 usec\nrounds: 59057"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 65458.31765926454,
            "unit": "iter/sec",
            "range": "stddev: 7.44304814120113e-7",
            "extra": "mean: 15.276897356350963 usec\nrounds: 66239"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 454539.74156667653,
            "unit": "iter/sec",
            "range": "stddev: 1.1542241024340397e-7",
            "extra": "mean: 2.200027651164814 usec\nrounds: 46649"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 451789.35284228774,
            "unit": "iter/sec",
            "range": "stddev: 1.2519465769546668e-7",
            "extra": "mean: 2.2134209089010866 usec\nrounds: 46406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 402564.4998324082,
            "unit": "iter/sec",
            "range": "stddev: 1.3699571274323727e-7",
            "extra": "mean: 2.484073981725436 usec\nrounds: 41321"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 210705.72364800665,
            "unit": "iter/sec",
            "range": "stddev: 3.220312686607252e-7",
            "extra": "mean: 4.745955556815081 usec\nrounds: 108296"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 36919.92046076696,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011974160318583364",
            "extra": "mean: 27.085648818302637 usec\nrounds: 37482"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3931082.8137290543,
            "unit": "iter/sec",
            "range": "stddev: 2.2594711656445613e-8",
            "extra": "mean: 254.38283734638196 nsec\nrounds: 193536"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3585019.5212363238,
            "unit": "iter/sec",
            "range": "stddev: 2.3342932906939188e-8",
            "extra": "mean: 278.93850900291255 nsec\nrounds: 194025"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2355740.83638345,
            "unit": "iter/sec",
            "range": "stddev: 3.78915205541299e-8",
            "extra": "mean: 424.4949124094705 nsec\nrounds: 187477"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 298794.01916696713,
            "unit": "iter/sec",
            "range": "stddev: 2.487330823860595e-7",
            "extra": "mean: 3.3467872040678177 usec\nrounds: 152463"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1601342.2709546017,
            "unit": "iter/sec",
            "range": "stddev: 5.559395658107559e-8",
            "extra": "mean: 624.4761149057063 nsec\nrounds: 163667"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1543682.4417617097,
            "unit": "iter/sec",
            "range": "stddev: 5.762843698323476e-8",
            "extra": "mean: 647.8016287202003 nsec\nrounds: 157978"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1303682.5935406021,
            "unit": "iter/sec",
            "range": "stddev: 6.236870242094889e-8",
            "extra": "mean: 767.057875095319 nsec\nrounds: 134481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 401022.5549531477,
            "unit": "iter/sec",
            "range": "stddev: 1.135507645690748e-7",
            "extra": "mean: 2.4936253276747222 usec\nrounds: 40817"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2767571.5472503603,
            "unit": "iter/sec",
            "range": "stddev: 3.078226824300941e-8",
            "extra": "mean: 361.3276054212695 nsec\nrounds: 191425"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 518752.30411097826,
            "unit": "iter/sec",
            "range": "stddev: 1.062290908465269e-7",
            "extra": "mean: 1.9277022811759434 usec\nrounds: 53744"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3098357.3073554337,
            "unit": "iter/sec",
            "range": "stddev: 9.210492557775724e-8",
            "extra": "mean: 322.7516715473832 nsec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2970341.9389669993,
            "unit": "iter/sec",
            "range": "stddev: 3.0202882304270273e-8",
            "extra": "mean: 336.661576528045 nsec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1871475.8018258582,
            "unit": "iter/sec",
            "range": "stddev: 8.440380124332554e-8",
            "extra": "mean: 534.3376596290382 nsec\nrounds: 197551"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 451052.52925125585,
            "unit": "iter/sec",
            "range": "stddev: 1.0025213112779628e-7",
            "extra": "mean: 2.2170366756616904 usec\nrounds: 46205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 22300.992292265833,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021551614713266297",
            "extra": "mean: 44.84105401654294 usec\nrounds: 22993"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1248972.5943458467,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026958311318628518",
            "extra": "mean: 800.6580805111704 nsec\nrounds: 131805"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1244435.3889332833,
            "unit": "iter/sec",
            "range": "stddev: 6.395558375819048e-8",
            "extra": "mean: 803.5772760024194 nsec\nrounds: 130668"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 897434.7353892059,
            "unit": "iter/sec",
            "range": "stddev: 1.3601246890342633e-7",
            "extra": "mean: 1.1142871571226967 usec\nrounds: 97743"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 354413.41140569665,
            "unit": "iter/sec",
            "range": "stddev: 2.536502697849692e-7",
            "extra": "mean: 2.8215636536826794 usec\nrounds: 184129"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 21584.433071032945,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021833384917462914",
            "extra": "mean: 46.329685691028615 usec\nrounds: 22042"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2944238.9532282525,
            "unit": "iter/sec",
            "range": "stddev: 2.968654009623659e-8",
            "extra": "mean: 339.64634524773743 nsec\nrounds: 191278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2386344.3345906646,
            "unit": "iter/sec",
            "range": "stddev: 3.720854961320943e-8",
            "extra": "mean: 419.05100848387605 nsec\nrounds: 190368"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 870482.011580009,
            "unit": "iter/sec",
            "range": "stddev: 8.01157841390206e-8",
            "extra": "mean: 1.148788816652176 usec\nrounds: 89937"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 112166.52273478427,
            "unit": "iter/sec",
            "range": "stddev: 5.350083624221714e-7",
            "extra": "mean: 8.915316046343722 usec\nrounds: 113727"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 380532.01519640884,
            "unit": "iter/sec",
            "range": "stddev: 3.127802521669239e-7",
            "extra": "mean: 2.6278997825816504 usec\nrounds: 196348"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 362283.6766444821,
            "unit": "iter/sec",
            "range": "stddev: 2.6795099795063993e-7",
            "extra": "mean: 2.760267890792454 usec\nrounds: 187653"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 275247.06927122734,
            "unit": "iter/sec",
            "range": "stddev: 3.2006958575281383e-7",
            "extra": "mean: 3.6330995372546693 usec\nrounds: 144760"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 89923.31100523529,
            "unit": "iter/sec",
            "range": "stddev: 7.298572285093052e-7",
            "extra": "mean: 11.120586962615073 usec\nrounds: 97381"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2718942.8436093135,
            "unit": "iter/sec",
            "range": "stddev: 5.397865317159684e-7",
            "extra": "mean: 367.79000424758124 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2815711.6141939685,
            "unit": "iter/sec",
            "range": "stddev: 1.9180631995267866e-7",
            "extra": "mean: 355.15000718078227 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2103890.1050528716,
            "unit": "iter/sec",
            "range": "stddev: 1.8763728817768654e-7",
            "extra": "mean: 475.30999722766865 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 544858.1736665104,
            "unit": "iter/sec",
            "range": "stddev: 2.0620540423765923e-7",
            "extra": "mean: 1.8353399991610786 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 58261.070183762175,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018041449297749134",
            "extra": "mean: 17.16412000064338 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 891464.2334004379,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013259562254748291",
            "extra": "mean: 1.1217499957183463 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 943868.1659887545,
            "unit": "iter/sec",
            "range": "stddev: 5.366196644529127e-7",
            "extra": "mean: 1.0594699938337726 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 821422.7083606152,
            "unit": "iter/sec",
            "range": "stddev: 5.399839834576743e-7",
            "extra": "mean: 1.2173999937203916 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 367351.2872319262,
            "unit": "iter/sec",
            "range": "stddev: 8.060416054670877e-7",
            "extra": "mean: 2.722189998394242 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 26252.134963099634,
            "unit": "iter/sec",
            "range": "stddev: 0.000001660769390093394",
            "extra": "mean: 38.092139988066265 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1665052.8974313356,
            "unit": "iter/sec",
            "range": "stddev: 5.0378042468424694e-8",
            "extra": "mean: 600.5815199881536 nsec\nrounds: 173793"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 432337.77615377464,
            "unit": "iter/sec",
            "range": "stddev: 1.2816435852149096e-7",
            "extra": "mean: 2.3130063000655383 usec\nrounds: 44317"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 51326.66457493963,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010224083515530152",
            "extra": "mean: 19.483050540717436 usec\nrounds: 52967"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 5221.0993203855105,
            "unit": "iter/sec",
            "range": "stddev: 0.000008317401945369993",
            "extra": "mean: 191.53054531936448 usec\nrounds: 5395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 235188.11877102236,
            "unit": "iter/sec",
            "range": "stddev: 3.128231272866169e-7",
            "extra": "mean: 4.251915467607416 usec\nrounds: 121551"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 150341.3064847909,
            "unit": "iter/sec",
            "range": "stddev: 8.247971435581343e-7",
            "extra": "mean: 6.6515319267972695 usec\nrounds: 156153"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 34576.84545317062,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011946587702563422",
            "extra": "mean: 28.921088285926963 usec\nrounds: 35589"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 4219.045438509889,
            "unit": "iter/sec",
            "range": "stddev: 0.000017195842814037474",
            "extra": "mean: 237.02043852677414 usec\nrounds: 4392"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 141242.89268331294,
            "unit": "iter/sec",
            "range": "stddev: 5.820172043308767e-7",
            "extra": "mean: 7.080002264199906 usec\nrounds: 147951"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17435.599639736916,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023131263233363557",
            "extra": "mean: 57.35392075193859 usec\nrounds: 17868"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1710.813868253421,
            "unit": "iter/sec",
            "range": "stddev: 0.00006007408893390152",
            "extra": "mean: 584.5171228480309 usec\nrounds: 1742"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1230570.3559564985,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026175920207759775",
            "extra": "mean: 812.6313096683724 nsec\nrounds: 130260"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 108464.47247232884,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018289595461899181",
            "extra": "mean: 9.219608754886234 usec\nrounds: 114956"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 18751.130959839367,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014251629228461845",
            "extra": "mean: 53.33011657492933 usec\nrounds: 18992"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2568.6660064860916,
            "unit": "iter/sec",
            "range": "stddev: 0.000007165710909445493",
            "extra": "mean: 389.30713353737633 usec\nrounds: 2606"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 809769.882952591,
            "unit": "iter/sec",
            "range": "stddev: 1.7893245956731374e-7",
            "extra": "mean: 1.2349187356212807 usec\nrounds: 90827"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 161863.44535871563,
            "unit": "iter/sec",
            "range": "stddev: 5.833274677611657e-7",
            "extra": "mean: 6.178047166757373 usec\nrounds: 181357"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 27909.49079940928,
            "unit": "iter/sec",
            "range": "stddev: 0.000001726012609238417",
            "extra": "mean: 35.83010550737693 usec\nrounds: 31941"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 6587.818709669389,
            "unit": "iter/sec",
            "range": "stddev: 0.000009544004156047663",
            "extra": "mean: 151.79531254134727 usec\nrounds: 7631"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2101809.848045402,
            "unit": "iter/sec",
            "range": "stddev: 4.437649600234287e-8",
            "extra": "mean: 475.78043319663425 nsec\nrounds: 186151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 972513.6865463698,
            "unit": "iter/sec",
            "range": "stddev: 6.712918073711619e-8",
            "extra": "mean: 1.02826316362832 usec\nrounds: 100960"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 328530.0130949515,
            "unit": "iter/sec",
            "range": "stddev: 2.2967784871412202e-7",
            "extra": "mean: 3.043861930845815 usec\nrounds: 171204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 98604.75035161468,
            "unit": "iter/sec",
            "range": "stddev: 6.009700896104239e-7",
            "extra": "mean: 10.141499232380792 usec\nrounds: 100929"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 977284.5843277131,
            "unit": "iter/sec",
            "range": "stddev: 7.399688490949116e-8",
            "extra": "mean: 1.023243399145515 usec\nrounds: 100442"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 800802.3459856198,
            "unit": "iter/sec",
            "range": "stddev: 8.215032260237793e-8",
            "extra": "mean: 1.248747590479643 usec\nrounds: 82672"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 428405.19393351313,
            "unit": "iter/sec",
            "range": "stddev: 1.1460234968820849e-7",
            "extra": "mean: 2.3342387397739808 usec\nrounds: 43676"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 207440.0147796789,
            "unit": "iter/sec",
            "range": "stddev: 9.000229050148258e-7",
            "extra": "mean: 4.820670693945406 usec\nrounds: 106316"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5810182.350205071,
            "unit": "iter/sec",
            "range": "stddev: 9.242749491549579e-9",
            "extra": "mean: 172.1116377637799 nsec\nrounds: 59624"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 454524.4289085151,
            "unit": "iter/sec",
            "range": "stddev: 1.7160652002695292e-7",
            "extra": "mean: 2.20010176879023 usec\nrounds: 46756"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 58162.70727805577,
            "unit": "iter/sec",
            "range": "stddev: 8.608158036981375e-7",
            "extra": "mean: 17.193147410063737 usec\nrounds: 60118"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4522.4446117158395,
            "unit": "iter/sec",
            "range": "stddev: 0.00014445672068251987",
            "extra": "mean: 221.11934713570645 usec\nrounds: 4661"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4510628.976223357,
            "unit": "iter/sec",
            "range": "stddev: 9.830207833807534e-9",
            "extra": "mean: 221.69857136803822 nsec\nrounds: 46086"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 441106.8309287369,
            "unit": "iter/sec",
            "range": "stddev: 1.1672394497282098e-7",
            "extra": "mean: 2.267024516248201 usec\nrounds: 45321"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2584750.8474237025,
            "unit": "iter/sec",
            "range": "stddev: 3.575757371310918e-8",
            "extra": "mean: 386.88448482247634 nsec\nrounds: 191059"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 53166.43211434034,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012447138541884532",
            "extra": "mean: 18.808860407435063 usec\nrounds: 55139"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 116897.49496246924,
            "unit": "iter/sec",
            "range": "stddev: 6.613976432091084e-7",
            "extra": "mean: 8.55450324509569 usec\nrounds: 122640"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14190.012722803467,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024771032089451805",
            "extra": "mean: 70.4721003098885 usec\nrounds: 14555"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1369.25737814449,
            "unit": "iter/sec",
            "range": "stddev: 0.000009442998717101792",
            "extra": "mean: 730.3228859391807 usec\nrounds: 1394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 54375.06243465934,
            "unit": "iter/sec",
            "range": "stddev: 0.00000115276387931507",
            "extra": "mean: 18.390783480969162 usec\nrounds: 57353"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 52713.96192190924,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010466813445618676",
            "extra": "mean: 18.970306225159202 usec\nrounds: 55384"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 37624.32259777485,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026027934027603055",
            "extra": "mean: 26.578551611162858 usec\nrounds: 38732"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 73512.98227973178,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021294754128289012",
            "extra": "mean: 13.603039476684508 usec\nrounds: 134409"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 254398.0030372113,
            "unit": "iter/sec",
            "range": "stddev: 3.127594818527917e-7",
            "extra": "mean: 3.9308484660303247 usec\nrounds: 130908"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 26308.212862019896,
            "unit": "iter/sec",
            "range": "stddev: 0.00001288287123841102",
            "extra": "mean: 38.010943778080026 usec\nrounds: 49109"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 33093.638260280066,
            "unit": "iter/sec",
            "range": "stddev: 0.000001507808637018071",
            "extra": "mean: 30.217288052012968 usec\nrounds: 33914"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8010.057339448023,
            "unit": "iter/sec",
            "range": "stddev: 0.00004295197231739925",
            "extra": "mean: 124.84305138181577 usec\nrounds: 11794"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3194.8289606182943,
            "unit": "iter/sec",
            "range": "stddev: 0.0000047280585088803615",
            "extra": "mean: 313.00580166472207 usec\nrounds: 3242"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 13979.594427922382,
            "unit": "iter/sec",
            "range": "stddev: 0.000017516762143027098",
            "extra": "mean: 71.53283345635785 usec\nrounds: 21736"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28716.126001313893,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017622637713888997",
            "extra": "mean: 34.82363881375383 usec\nrounds: 29392"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 26602.870634389197,
            "unit": "iter/sec",
            "range": "stddev: 0.000010736600232681182",
            "extra": "mean: 37.58992831049265 usec\nrounds: 47064"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 25190.317064374372,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013690582590870627",
            "extra": "mean: 39.697793300674995 usec\nrounds: 26391"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 22024.307542229777,
            "unit": "iter/sec",
            "range": "stddev: 0.000015831706630735904",
            "extra": "mean: 45.40437868852781 usec\nrounds: 38430"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22660.78311605548,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019605512856332737",
            "extra": "mean: 44.12910157952511 usec\nrounds: 23164"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 12497.356842045292,
            "unit": "iter/sec",
            "range": "stddev: 0.00002158507540196979",
            "extra": "mean: 80.01691978864405 usec\nrounds: 19461"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 20721.48564626273,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021376473159762176",
            "extra": "mean: 48.25908803408395 usec\nrounds: 21151"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 30021.43198876212,
            "unit": "iter/sec",
            "range": "stddev: 0.00003867736143571924",
            "extra": "mean: 33.30953701256917 usec\nrounds: 56332"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 49131.01621221321,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011698502696150483",
            "extra": "mean: 20.35374142640704 usec\nrounds: 50651"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4677.560968305049,
            "unit": "iter/sec",
            "range": "stddev: 0.00004681627494411156",
            "extra": "mean: 213.7866308479904 usec\nrounds: 6263"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 966.3557665510732,
            "unit": "iter/sec",
            "range": "stddev: 0.000008095285192010778",
            "extra": "mean: 1.034815576843923 msec\nrounds: 976"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1016.3298142526613,
            "unit": "iter/sec",
            "range": "stddev: 0.00011055477698871188",
            "extra": "mean: 983.9325639928519 usec\nrounds: 1172"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 276.29368320349863,
            "unit": "iter/sec",
            "range": "stddev: 0.000017783767173519748",
            "extra": "mean: 3.6193371792125624 msec\nrounds: 279"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 53576.15670482467,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025697704112500846",
            "extra": "mean: 18.66501932024451 usec\nrounds: 93476"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 17952.487075433837,
            "unit": "iter/sec",
            "range": "stddev: 0.000010416574569816604",
            "extra": "mean: 55.702588493624305 usec\nrounds: 31912"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5966.273249722306,
            "unit": "iter/sec",
            "range": "stddev: 0.00003519981615738598",
            "extra": "mean: 167.60881678467274 usec\nrounds: 7434"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1634.6825057083188,
            "unit": "iter/sec",
            "range": "stddev: 0.00005598475623960687",
            "extra": "mean: 611.7395864383423 usec\nrounds: 1799"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 26813.558574009796,
            "unit": "iter/sec",
            "range": "stddev: 0.000012247537469759106",
            "extra": "mean: 37.29456488365157 usec\nrounds: 47601"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1689.9770984582385,
            "unit": "iter/sec",
            "range": "stddev: 0.00007693477786629807",
            "extra": "mean: 591.7239949063791 usec\nrounds: 1964"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1162.2556534955268,
            "unit": "iter/sec",
            "range": "stddev: 0.00009607672348674362",
            "extra": "mean: 860.3959008437286 usec\nrounds: 1301"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1764552912173,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5467594.335378157,
            "unit": "iter/sec",
            "range": "stddev: 1.5536255354316393e-8",
            "extra": "mean: 182.89579267604742 nsec\nrounds: 55729"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5077322.300246044,
            "unit": "iter/sec",
            "range": "stddev: 1.501923881833972e-8",
            "extra": "mean: 196.9542094957487 nsec\nrounds: 51581"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3233396.1568021504,
            "unit": "iter/sec",
            "range": "stddev: 3.692217250461614e-8",
            "extra": "mean: 309.2723413727919 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 694765.2283235616,
            "unit": "iter/sec",
            "range": "stddev: 3.3848458635091777e-7",
            "extra": "mean: 1.439335129670994 usec\nrounds: 70792"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 71086.70222423293,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014805902004232409",
            "extra": "mean: 14.06732861014767 usec\nrounds: 78902"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 384437.8443343422,
            "unit": "iter/sec",
            "range": "stddev: 3.122658620633387e-7",
            "extra": "mean: 2.6012007265608963 usec\nrounds: 134880"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 381150.03603954613,
            "unit": "iter/sec",
            "range": "stddev: 3.433989904923806e-7",
            "extra": "mean: 2.6236387391977902 usec\nrounds: 134157"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 342901.8105149076,
            "unit": "iter/sec",
            "range": "stddev: 6.283433297610615e-7",
            "extra": "mean: 2.916286730881887 usec\nrounds: 181489"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 207032.97019490742,
            "unit": "iter/sec",
            "range": "stddev: 5.61964969325287e-7",
            "extra": "mean: 4.830148546188408 usec\nrounds: 108027"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39403.0161683464,
            "unit": "iter/sec",
            "range": "stddev: 0.000001991024892036455",
            "extra": "mean: 25.378767851871434 usec\nrounds: 41060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3712290.984702154,
            "unit": "iter/sec",
            "range": "stddev: 3.3283429546012144e-8",
            "extra": "mean: 269.37543530956304 nsec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3477897.989531972,
            "unit": "iter/sec",
            "range": "stddev: 3.56375077382906e-8",
            "extra": "mean: 287.5299974322575 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2219167.65600291,
            "unit": "iter/sec",
            "range": "stddev: 8.460628903171208e-8",
            "extra": "mean: 450.61940106012014 nsec\nrounds: 190877"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 515312.96259277995,
            "unit": "iter/sec",
            "range": "stddev: 1.4201431672804747e-7",
            "extra": "mean: 1.9405683004140886 usec\nrounds: 52395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1455070.9325927552,
            "unit": "iter/sec",
            "range": "stddev: 8.00779796632469e-8",
            "extra": "mean: 687.251719212125 nsec\nrounds: 150331"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1433728.5597874452,
            "unit": "iter/sec",
            "range": "stddev: 8.160606122553715e-8",
            "extra": "mean: 697.482095319534 nsec\nrounds: 148302"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1249881.7655614975,
            "unit": "iter/sec",
            "range": "stddev: 8.685474102279702e-8",
            "extra": "mean: 800.0756771987689 nsec\nrounds: 129467"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 439287.65668604674,
            "unit": "iter/sec",
            "range": "stddev: 1.5739392362435452e-7",
            "extra": "mean: 2.276412698558186 usec\nrounds: 44761"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2756118.469597525,
            "unit": "iter/sec",
            "range": "stddev: 4.57982337531862e-8",
            "extra": "mean: 362.8291058716625 nsec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 502568.02955798124,
            "unit": "iter/sec",
            "range": "stddev: 1.699651004057028e-7",
            "extra": "mean: 1.9897803703898798 usec\nrounds: 52701"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2929295.526764037,
            "unit": "iter/sec",
            "range": "stddev: 4.392299854732698e-8",
            "extra": "mean: 341.3790076362455 nsec\nrounds: 192679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2793190.303888129,
            "unit": "iter/sec",
            "range": "stddev: 4.588952941901849e-8",
            "extra": "mean: 358.01355840603736 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1822944.3082131206,
            "unit": "iter/sec",
            "range": "stddev: 6.65226156150699e-8",
            "extra": "mean: 548.5631105101235 nsec\nrounds: 188715"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 504229.57777762075,
            "unit": "iter/sec",
            "range": "stddev: 1.423685242204116e-7",
            "extra": "mean: 1.9832236030410249 usec\nrounds: 51290"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 44863.11134062817,
            "unit": "iter/sec",
            "range": "stddev: 0.000003291232215342071",
            "extra": "mean: 22.29002782280053 usec\nrounds: 48881"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1173914.6535392448,
            "unit": "iter/sec",
            "range": "stddev: 8.928536300371698e-8",
            "extra": "mean: 851.8506835101919 nsec\nrounds: 120846"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1156956.3414038508,
            "unit": "iter/sec",
            "range": "stddev: 8.8133356683473e-8",
            "extra": "mean: 864.3368502450471 nsec\nrounds: 119546"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 906968.2358949027,
            "unit": "iter/sec",
            "range": "stddev: 1.1776531457741966e-7",
            "extra": "mean: 1.102574445744811 usec\nrounds: 93730"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 373062.62677970016,
            "unit": "iter/sec",
            "range": "stddev: 3.862992197056678e-7",
            "extra": "mean: 2.680515088396987 usec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 37123.477620292884,
            "unit": "iter/sec",
            "range": "stddev: 0.000004301978692111206",
            "extra": "mean: 26.93713154323042 usec\nrounds: 44244"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2790564.4489911646,
            "unit": "iter/sec",
            "range": "stddev: 4.463503137895716e-8",
            "extra": "mean: 358.35044066511347 nsec\nrounds: 189790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2249425.25677228,
            "unit": "iter/sec",
            "range": "stddev: 5.5208802537361174e-8",
            "extra": "mean: 444.55800297847395 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 789250.0198229034,
            "unit": "iter/sec",
            "range": "stddev: 1.164946473012291e-7",
            "extra": "mean: 1.267025625446797 usec\nrounds: 80822"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107951.08598899226,
            "unit": "iter/sec",
            "range": "stddev: 8.88416131364736e-7",
            "extra": "mean: 9.26345474747674 usec\nrounds: 109806"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 368957.6487794293,
            "unit": "iter/sec",
            "range": "stddev: 4.1482078204873445e-7",
            "extra": "mean: 2.7103381737935486 usec\nrounds: 192716"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 362101.96814988786,
            "unit": "iter/sec",
            "range": "stddev: 4.204101176607601e-7",
            "extra": "mean: 2.761653036876236 usec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 298250.45854910585,
            "unit": "iter/sec",
            "range": "stddev: 4.782753798477283e-7",
            "extra": "mean: 3.3528867142893386 usec\nrounds: 154512"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 121897.4847125757,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010612722903951739",
            "extra": "mean: 8.203614720663994 usec\nrounds: 126503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2552257.4565754985,
            "unit": "iter/sec",
            "range": "stddev: 5.676514929919972e-7",
            "extra": "mean: 391.8100023270199 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2808515.4155997625,
            "unit": "iter/sec",
            "range": "stddev: 1.8097116337811852e-7",
            "extra": "mean: 356.0600003993386 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1959669.9908323125,
            "unit": "iter/sec",
            "range": "stddev: 1.555203267901257e-7",
            "extra": "mean: 510.29000019298104 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 482737.3135935696,
            "unit": "iter/sec",
            "range": "stddev: 8.598424626645164e-7",
            "extra": "mean: 2.0715200002996426 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 46584.99385734892,
            "unit": "iter/sec",
            "range": "stddev: 0.00000339988660767138",
            "extra": "mean: 21.46613999912006 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 794678.8290325935,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017562241574006263",
            "extra": "mean: 1.258370002403808 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 780652.3133906734,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015291413720189683",
            "extra": "mean: 1.2809799994784044 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 729873.7315250544,
            "unit": "iter/sec",
            "range": "stddev: 4.4885647076965433e-7",
            "extra": "mean: 1.3701000005994501 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 355864.287602277,
            "unit": "iter/sec",
            "range": "stddev: 6.353563350667831e-7",
            "extra": "mean: 2.8100599999447695 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 41474.05424615902,
            "unit": "iter/sec",
            "range": "stddev: 0.000003834670395285059",
            "extra": "mean: 24.111460000142415 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1639869.1505228335,
            "unit": "iter/sec",
            "range": "stddev: 8.38126214651507e-8",
            "extra": "mean: 609.804751605411 nsec\nrounds: 168322"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 372498.3023130428,
            "unit": "iter/sec",
            "range": "stddev: 3.6451203513387943e-7",
            "extra": "mean: 2.68457599347557 usec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 45425.337521642345,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014388863455724774",
            "extra": "mean: 22.014145729210316 usec\nrounds: 46209"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4610.128404173243,
            "unit": "iter/sec",
            "range": "stddev: 0.00000606653900466611",
            "extra": "mean: 216.9136979123545 usec\nrounds: 4694"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 204975.51542332085,
            "unit": "iter/sec",
            "range": "stddev: 5.86520293531954e-7",
            "extra": "mean: 4.878631469396594 usec\nrounds: 107101"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 134497.65527164654,
            "unit": "iter/sec",
            "range": "stddev: 9.949917678095779e-7",
            "extra": "mean: 7.435073852999801 usec\nrounds: 139412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31890.41753446019,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019194135772296618",
            "extra": "mean: 31.3573818505016 usec\nrounds: 32717"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3690.5038080304817,
            "unit": "iter/sec",
            "range": "stddev: 0.000006068615378759006",
            "extra": "mean: 270.9657141726869 usec\nrounds: 3796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 141016.78319624928,
            "unit": "iter/sec",
            "range": "stddev: 8.990217105683597e-7",
            "extra": "mean: 7.091354499331664 usec\nrounds: 147233"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17243.919622859634,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027054834414465483",
            "extra": "mean: 57.99145564760906 usec\nrounds: 17654"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1615.7405483195362,
            "unit": "iter/sec",
            "range": "stddev: 0.000010739768675804172",
            "extra": "mean: 618.9112484922521 usec\nrounds: 1658"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1233676.2514920689,
            "unit": "iter/sec",
            "range": "stddev: 8.081355670157108e-8",
            "extra": "mean: 810.5854342178305 nsec\nrounds: 127470"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104905.20140683831,
            "unit": "iter/sec",
            "range": "stddev: 8.856193927785988e-7",
            "extra": "mean: 9.532415805788771 usec\nrounds: 106872"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17063.772048741845,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026243215174450295",
            "extra": "mean: 58.60368956779005 usec\nrounds: 17350"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2352.0876900570415,
            "unit": "iter/sec",
            "range": "stddev: 0.0000067406404941214315",
            "extra": "mean: 425.15421692281745 usec\nrounds: 2411"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 704487.5560313442,
            "unit": "iter/sec",
            "range": "stddev: 1.2637965543625957e-7",
            "extra": "mean: 1.4194714887987476 usec\nrounds: 73287"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 104894.33500562477,
            "unit": "iter/sec",
            "range": "stddev: 9.35024212550225e-7",
            "extra": "mean: 9.533403304824581 usec\nrounds: 107782"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21764.44177754818,
            "unit": "iter/sec",
            "range": "stddev: 0.0000033889546097110423",
            "extra": "mean: 45.94650348586393 usec\nrounds: 23524"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4262.580920900454,
            "unit": "iter/sec",
            "range": "stddev: 0.00003139631031616933",
            "extra": "mean: 234.5996518439711 usec\nrounds: 4610"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2093326.6727482248,
            "unit": "iter/sec",
            "range": "stddev: 8.851613436016645e-8",
            "extra": "mean: 477.70852634655245 nsec\nrounds: 199641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 955548.5058284661,
            "unit": "iter/sec",
            "range": "stddev: 9.400773064090428e-8",
            "extra": "mean: 1.0465193487305169 usec\nrounds: 98146"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 365058.44665460597,
            "unit": "iter/sec",
            "range": "stddev: 3.537609665067646e-7",
            "extra": "mean: 2.7392873912766453 usec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 121342.910347966,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010684007776573075",
            "extra": "mean: 8.24110775926154 usec\nrounds: 124611"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 883542.4975185121,
            "unit": "iter/sec",
            "range": "stddev: 1.1484369312104583e-7",
            "extra": "mean: 1.1318074714103556 usec\nrounds: 90572"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 714291.0742602004,
            "unit": "iter/sec",
            "range": "stddev: 1.3322852806767493e-7",
            "extra": "mean: 1.3999894945288063 usec\nrounds: 73714"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 497410.35082389944,
            "unit": "iter/sec",
            "range": "stddev: 1.5441547940462765e-7",
            "extra": "mean: 2.0104125262846626 usec\nrounds: 50925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 160366.74633630688,
            "unit": "iter/sec",
            "range": "stddev: 0.000001018992939278303",
            "extra": "mean: 6.23570673375694 usec\nrounds: 170040"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5730422.717358925,
            "unit": "iter/sec",
            "range": "stddev: 1.2210820545338884e-8",
            "extra": "mean: 174.50719594747454 nsec\nrounds: 57827"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 383641.7002585672,
            "unit": "iter/sec",
            "range": "stddev: 3.126091041983051e-7",
            "extra": "mean: 2.606598811667296 usec\nrounds: 134699"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 55868.087529422635,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013800632148992812",
            "extra": "mean: 17.899306101597897 usec\nrounds: 58167"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4036.1596510155764,
            "unit": "iter/sec",
            "range": "stddev: 0.000006703047962049455",
            "extra": "mean: 247.76026878629057 usec\nrounds: 4152"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4501147.984442889,
            "unit": "iter/sec",
            "range": "stddev: 1.419866545084157e-8",
            "extra": "mean: 222.16554609091153 nsec\nrounds: 45661"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 365027.0510355885,
            "unit": "iter/sec",
            "range": "stddev: 3.9562685917006764e-7",
            "extra": "mean: 2.739522994701301 usec\nrounds: 194175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2409496.532910557,
            "unit": "iter/sec",
            "range": "stddev: 5.402507368509193e-8",
            "extra": "mean: 415.0244610612122 nsec\nrounds: 191205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 49437.10983011125,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022219993431717757",
            "extra": "mean: 20.227719691471894 usec\nrounds: 51372"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 116666.92401901279,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010688473606657623",
            "extra": "mean: 8.571409663950972 usec\nrounds: 122310"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14291.810638744064,
            "unit": "iter/sec",
            "range": "stddev: 0.0000037750694835434657",
            "extra": "mean: 69.97014061248981 usec\nrounds: 14636"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1325.8089932546611,
            "unit": "iter/sec",
            "range": "stddev: 0.000014553585319258254",
            "extra": "mean: 754.2564615926694 usec\nrounds: 1458"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 50695.65841688866,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018295773708727348",
            "extra": "mean: 19.72555503227988 usec\nrounds: 52869"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49728.36521011393,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019677366705929943",
            "extra": "mean: 20.109247423975575 usec\nrounds: 51531"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 36342.835542098255,
            "unit": "iter/sec",
            "range": "stddev: 0.000002274603668884686",
            "extra": "mean: 27.515739624709123 usec\nrounds: 40265"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 67040.3742896234,
            "unit": "iter/sec",
            "range": "stddev: 0.000004109898706704624",
            "extra": "mean: 14.916384501074917 usec\nrounds: 119105"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 247271.92115617584,
            "unit": "iter/sec",
            "range": "stddev: 6.402338744580827e-7",
            "extra": "mean: 4.04413083104735 usec\nrounds: 140391"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 33334.001990449935,
            "unit": "iter/sec",
            "range": "stddev: 0.0000061776722405710224",
            "extra": "mean: 29.99939822066658 usec\nrounds: 50133"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32850.472712576084,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021123808158190917",
            "extra": "mean: 30.44096225796994 usec\nrounds: 33517"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8842.012415852005,
            "unit": "iter/sec",
            "range": "stddev: 0.00003261035290030294",
            "extra": "mean: 113.09642567422715 usec\nrounds: 12573"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3177.273251985226,
            "unit": "iter/sec",
            "range": "stddev: 0.000005667468776306371",
            "extra": "mean: 314.73528421742736 usec\nrounds: 3244"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 16163.54280447873,
            "unit": "iter/sec",
            "range": "stddev: 0.00001445991600625084",
            "extra": "mean: 61.86762469691431 usec\nrounds: 22675"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28152.797027660446,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020650214591579773",
            "extra": "mean: 35.520449318676526 usec\nrounds: 31047"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 30573.453679923237,
            "unit": "iter/sec",
            "range": "stddev: 0.00000726809903498746",
            "extra": "mean: 32.70811372732394 usec\nrounds: 44721"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 25349.474782158606,
            "unit": "iter/sec",
            "range": "stddev: 0.000002008993103837318",
            "extra": "mean: 39.448549076204806 usec\nrounds: 25980"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 27594.762151889983,
            "unit": "iter/sec",
            "range": "stddev: 0.00000490778713292551",
            "extra": "mean: 36.23876134520367 usec\nrounds: 39312"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23212.024172544407,
            "unit": "iter/sec",
            "range": "stddev: 0.0000035853861427304176",
            "extra": "mean: 43.081120050823394 usec\nrounds: 23715"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16002.254478429653,
            "unit": "iter/sec",
            "range": "stddev: 0.000012149754432197671",
            "extra": "mean: 62.491194684346304 usec\nrounds: 21296"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21204.17162343288,
            "unit": "iter/sec",
            "range": "stddev: 0.000002449713215653007",
            "extra": "mean: 47.160531321812776 usec\nrounds: 21742"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 33148.01970368017,
            "unit": "iter/sec",
            "range": "stddev: 0.000006457523142250959",
            "extra": "mean: 30.16771466106549 usec\nrounds: 53291"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47903.709301235765,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015917412189708052",
            "extra": "mean: 20.87521017864483 usec\nrounds: 49339"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4552.268562433324,
            "unit": "iter/sec",
            "range": "stddev: 0.00006827681078780874",
            "extra": "mean: 219.6706952336463 usec\nrounds: 6441"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 922.3928875357019,
            "unit": "iter/sec",
            "range": "stddev: 0.00001649904958314469",
            "extra": "mean: 1.084136720385644 msec\nrounds: 937"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 997.097423246594,
            "unit": "iter/sec",
            "range": "stddev: 0.0001742982576095877",
            "extra": "mean: 1.0029110262304712 msec\nrounds: 1220"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 280.3716714665857,
            "unit": "iter/sec",
            "range": "stddev: 0.000033040438708000526",
            "extra": "mean: 3.5666941484107055 msec\nrounds: 283"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 54836.32000975739,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029789549145475428",
            "extra": "mean: 18.236088778788645 usec\nrounds: 122011"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 23044.535196627698,
            "unit": "iter/sec",
            "range": "stddev: 0.000009714866132706449",
            "extra": "mean: 43.39423605065111 usec\nrounds: 34912"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5592.585764630129,
            "unit": "iter/sec",
            "range": "stddev: 0.000058199124583233124",
            "extra": "mean: 178.80816532567488 usec\nrounds: 8081"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1720.7860443013951,
            "unit": "iter/sec",
            "range": "stddev: 0.00009772512582667313",
            "extra": "mean: 581.1297710785307 usec\nrounds: 1957"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 31953.578364249854,
            "unit": "iter/sec",
            "range": "stddev: 0.000007129539974450048",
            "extra": "mean: 31.295399488616123 usec\nrounds: 48502"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1686.5707715801211,
            "unit": "iter/sec",
            "range": "stddev: 0.00009834553517161618",
            "extra": "mean: 592.9190857867862 usec\nrounds: 1970"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1162.0838642339722,
            "unit": "iter/sec",
            "range": "stddev: 0.00012508802189923143",
            "extra": "mean: 860.5230919880164 usec\nrounds: 1348"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1765157049792,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5404248.075397656,
            "unit": "iter/sec",
            "range": "stddev: 1.231461828440699e-8",
            "extra": "mean: 185.0396181019833 nsec\nrounds: 54753"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5006388.462572623,
            "unit": "iter/sec",
            "range": "stddev: 1.8186855093100494e-8",
            "extra": "mean: 199.74478758008522 nsec\nrounds: 51055"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3232653.9633780746,
            "unit": "iter/sec",
            "range": "stddev: 3.5947261176793066e-8",
            "extra": "mean: 309.34334801332864 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 697735.3724545577,
            "unit": "iter/sec",
            "range": "stddev: 1.0145119307510926e-7",
            "extra": "mean: 1.4332081179747298 usec\nrounds: 70892"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 72223.0831926974,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016797222138895366",
            "extra": "mean: 13.845988786326304 usec\nrounds: 76782"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 411284.6928511525,
            "unit": "iter/sec",
            "range": "stddev: 5.187375958392248e-7",
            "extra": "mean: 2.4314058300290426 usec\nrounds: 42384"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 407572.7017689577,
            "unit": "iter/sec",
            "range": "stddev: 2.3326515319087429e-7",
            "extra": "mean: 2.4535499940495793 usec\nrounds: 42063"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 356923.4551856941,
            "unit": "iter/sec",
            "range": "stddev: 4.5989485014532697e-7",
            "extra": "mean: 2.801721168701947 usec\nrounds: 186568"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 211341.32462767215,
            "unit": "iter/sec",
            "range": "stddev: 6.59045643389941e-7",
            "extra": "mean: 4.731682276344852 usec\nrounds: 110412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39483.67164689511,
            "unit": "iter/sec",
            "range": "stddev: 0.000004036949783616635",
            "extra": "mean: 25.326925239958967 usec\nrounds: 40757"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3790991.501327504,
            "unit": "iter/sec",
            "range": "stddev: 3.218292111089156e-8",
            "extra": "mean: 263.7832344519438 nsec\nrounds: 194175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3470288.362137151,
            "unit": "iter/sec",
            "range": "stddev: 4.429269736260064e-8",
            "extra": "mean: 288.1604914769196 nsec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2259080.1592037017,
            "unit": "iter/sec",
            "range": "stddev: 5.439233621328056e-8",
            "extra": "mean: 442.65804200258304 nsec\nrounds: 193051"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 510386.46660998353,
            "unit": "iter/sec",
            "range": "stddev: 1.3072738617993087e-7",
            "extra": "mean: 1.9592996002461607 usec\nrounds: 52533"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1458469.5717227794,
            "unit": "iter/sec",
            "range": "stddev: 7.800042268636541e-8",
            "extra": "mean: 685.6502318514309 nsec\nrounds: 150309"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1438700.4382801058,
            "unit": "iter/sec",
            "range": "stddev: 7.772594417619112e-8",
            "extra": "mean: 695.0717282017715 nsec\nrounds: 146135"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1245505.2230576088,
            "unit": "iter/sec",
            "range": "stddev: 9.240668975693758e-8",
            "extra": "mean: 802.8870385184567 nsec\nrounds: 127470"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 440861.8086774143,
            "unit": "iter/sec",
            "range": "stddev: 1.4570765295684612e-7",
            "extra": "mean: 2.268284483521005 usec\nrounds: 44920"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2769942.9990676213,
            "unit": "iter/sec",
            "range": "stddev: 4.278249128894162e-8",
            "extra": "mean: 361.018259342081 nsec\nrounds: 188006"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 497336.6497637537,
            "unit": "iter/sec",
            "range": "stddev: 1.6151951092750526e-7",
            "extra": "mean: 2.0107104523164274 usec\nrounds: 51003"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3247264.8934098464,
            "unit": "iter/sec",
            "range": "stddev: 3.8496871696615995e-8",
            "extra": "mean: 307.951470799143 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 3123212.25453935,
            "unit": "iter/sec",
            "range": "stddev: 4.234286644577136e-8",
            "extra": "mean: 320.1831699227875 nsec\nrounds: 191608"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1810858.7176879987,
            "unit": "iter/sec",
            "range": "stddev: 6.700771716569156e-8",
            "extra": "mean: 552.2241963065893 nsec\nrounds: 199204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 498093.9406949603,
            "unit": "iter/sec",
            "range": "stddev: 2.6086438429680293e-7",
            "extra": "mean: 2.0076534129380805 usec\nrounds: 51422"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36007.21051164075,
            "unit": "iter/sec",
            "range": "stddev: 0.000002275083659045734",
            "extra": "mean: 27.772215225522974 usec\nrounds: 37384"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1177751.4952641428,
            "unit": "iter/sec",
            "range": "stddev: 8.822182170186719e-8",
            "extra": "mean: 849.0755511847066 nsec\nrounds: 122325"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1162070.235111916,
            "unit": "iter/sec",
            "range": "stddev: 9.786903545023526e-8",
            "extra": "mean: 860.5331844711817 nsec\nrounds: 120701"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 894637.4779963858,
            "unit": "iter/sec",
            "range": "stddev: 1.1270373367671901e-7",
            "extra": "mean: 1.117771191789963 usec\nrounds: 91828"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 361867.09395890625,
            "unit": "iter/sec",
            "range": "stddev: 3.9378006626862294e-7",
            "extra": "mean: 2.763445521005456 usec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34576.07013797042,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025547853907746784",
            "extra": "mean: 28.921736796855626 usec\nrounds: 36166"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2816948.959846545,
            "unit": "iter/sec",
            "range": "stddev: 6.092810958381742e-8",
            "extra": "mean: 354.9940074364981 nsec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2265485.0109753674,
            "unit": "iter/sec",
            "range": "stddev: 5.089177675953922e-8",
            "extra": "mean: 441.40658408921865 nsec\nrounds: 194553"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 787219.9987548011,
            "unit": "iter/sec",
            "range": "stddev: 9.84156187829617e-8",
            "extra": "mean: 1.2702929315588638 usec\nrounds: 80626"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107337.46930345774,
            "unit": "iter/sec",
            "range": "stddev: 8.804746826243765e-7",
            "extra": "mean: 9.316411188835307 usec\nrounds: 109806"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 371245.29562290176,
            "unit": "iter/sec",
            "range": "stddev: 4.230686976210133e-7",
            "extra": "mean: 2.6936368266219484 usec\nrounds: 191242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 363379.6720267079,
            "unit": "iter/sec",
            "range": "stddev: 4.0082507597094635e-7",
            "extra": "mean: 2.7519426015842225 usec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 299518.89557147765,
            "unit": "iter/sec",
            "range": "stddev: 5.07325940725391e-7",
            "extra": "mean: 3.33868752451165 usec\nrounds: 155473"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 118953.47034808411,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010838090555434418",
            "extra": "mean: 8.406648390112364 usec\nrounds: 122926"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2566405.7608069247,
            "unit": "iter/sec",
            "range": "stddev: 4.7335485210303866e-7",
            "extra": "mean: 389.6499981692614 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2732091.145096604,
            "unit": "iter/sec",
            "range": "stddev: 1.7972880546846284e-7",
            "extra": "mean: 366.0199996602387 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1931061.1251007344,
            "unit": "iter/sec",
            "range": "stddev: 1.669971501538939e-7",
            "extra": "mean: 517.8499981184359 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 502323.2448349132,
            "unit": "iter/sec",
            "range": "stddev: 1.7125923568156288e-7",
            "extra": "mean: 1.9907500006866032 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 54834.81286382873,
            "unit": "iter/sec",
            "range": "stddev: 0.000003223101542359254",
            "extra": "mean: 18.236590001379227 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 802098.2901174227,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012146309807599162",
            "extra": "mean: 1.2467299984564306 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 881818.662725694,
            "unit": "iter/sec",
            "range": "stddev: 4.344517724128799e-7",
            "extra": "mean: 1.1340200001086487 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 678094.8240855978,
            "unit": "iter/sec",
            "range": "stddev: 5.493818554747362e-7",
            "extra": "mean: 1.474720001510832 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 369942.8802776092,
            "unit": "iter/sec",
            "range": "stddev: 6.109387585172459e-7",
            "extra": "mean: 2.7031200039573378 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 53876.87194872127,
            "unit": "iter/sec",
            "range": "stddev: 0.000002342806906979382",
            "extra": "mean: 18.560840001100587 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1663960.4214876278,
            "unit": "iter/sec",
            "range": "stddev: 7.388196410781611e-8",
            "extra": "mean: 600.9758327700249 nsec\nrounds: 176367"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 373748.572971199,
            "unit": "iter/sec",
            "range": "stddev: 3.2524995980197004e-7",
            "extra": "mean: 2.6755955000718084 usec\nrounds: 190513"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 44340.44246241951,
            "unit": "iter/sec",
            "range": "stddev: 0.000001400742401541403",
            "extra": "mean: 22.552774498079135 usec\nrounds: 45432"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4536.137750775272,
            "unit": "iter/sec",
            "range": "stddev: 0.000004892160086573676",
            "extra": "mean: 220.45185903560574 usec\nrounds: 4604"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 205559.1809508754,
            "unit": "iter/sec",
            "range": "stddev: 7.679021686341815e-7",
            "extra": "mean: 4.864779064472827 usec\nrounds: 108027"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 132803.75458594266,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017485389078070567",
            "extra": "mean: 7.529907592731949 usec\nrounds: 139199"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31940.491008766032,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020311452688856384",
            "extra": "mean: 31.30822252311497 usec\nrounds: 32855"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3690.5740023576254,
            "unit": "iter/sec",
            "range": "stddev: 0.000005952965471325192",
            "extra": "mean: 270.9605604334655 usec\nrounds: 3781"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 143432.1978078613,
            "unit": "iter/sec",
            "range": "stddev: 8.18241441367587e-7",
            "extra": "mean: 6.971935278713213 usec\nrounds: 149410"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17452.14561289914,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028912086873672305",
            "extra": "mean: 57.299544834240045 usec\nrounds: 18033"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1635.7289552709437,
            "unit": "iter/sec",
            "range": "stddev: 0.00001103447798671139",
            "extra": "mean: 611.3482290434597 usec\nrounds: 1694"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1255343.1424719628,
            "unit": "iter/sec",
            "range": "stddev: 8.337601675916925e-8",
            "extra": "mean: 796.5949437783609 nsec\nrounds: 130651"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 105056.27320924608,
            "unit": "iter/sec",
            "range": "stddev: 8.4624774769808e-7",
            "extra": "mean: 9.518708111873032 usec\nrounds: 107435"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17147.677524552753,
            "unit": "iter/sec",
            "range": "stddev: 0.000002396535828102576",
            "extra": "mean: 58.3169352565768 usec\nrounds: 17438"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2367.3482941072293,
            "unit": "iter/sec",
            "range": "stddev: 0.000006233192608554145",
            "extra": "mean: 422.4135512671229 usec\nrounds: 2409"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 720469.9114923765,
            "unit": "iter/sec",
            "range": "stddev: 1.0800519244478821e-7",
            "extra": "mean: 1.3879830150416586 usec\nrounds: 73883"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 102512.845805407,
            "unit": "iter/sec",
            "range": "stddev: 9.761325217783174e-7",
            "extra": "mean: 9.754875031937267 usec\nrounds: 105731"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21264.92171893179,
            "unit": "iter/sec",
            "range": "stddev: 0.000003311109874267536",
            "extra": "mean: 47.02580207994452 usec\nrounds: 22691"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4278.063696601867,
            "unit": "iter/sec",
            "range": "stddev: 0.00001742139789144171",
            "extra": "mean: 233.750610303983 usec\nrounds: 4542"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2140351.6366732833,
            "unit": "iter/sec",
            "range": "stddev: 5.330402030653641e-8",
            "extra": "mean: 467.2129489686442 nsec\nrounds: 188715"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 992802.8725747515,
            "unit": "iter/sec",
            "range": "stddev: 9.069924116662991e-8",
            "extra": "mean: 1.0072493015724266 usec\nrounds: 101441"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 366779.8971703855,
            "unit": "iter/sec",
            "range": "stddev: 3.244462029939571e-7",
            "extra": "mean: 2.726430776917568 usec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 125143.62233260997,
            "unit": "iter/sec",
            "range": "stddev: 9.832609386211802e-7",
            "extra": "mean: 7.990818719807983 usec\nrounds: 128784"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 889488.8641559298,
            "unit": "iter/sec",
            "range": "stddev: 9.659169910207598e-8",
            "extra": "mean: 1.1242411684927929 usec\nrounds: 91400"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 717788.1082604716,
            "unit": "iter/sec",
            "range": "stddev: 1.0939930446660782e-7",
            "extra": "mean: 1.3931688035671734 usec\nrounds: 73611"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 456515.7311448311,
            "unit": "iter/sec",
            "range": "stddev: 1.389357552230415e-7",
            "extra": "mean: 2.1905050182876216 usec\nrounds: 46729"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 194438.31778280577,
            "unit": "iter/sec",
            "range": "stddev: 0.000001546279689567046",
            "extra": "mean: 5.14301919191172 usec\nrounds: 103221"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5718298.479023007,
            "unit": "iter/sec",
            "range": "stddev: 1.109388120202028e-8",
            "extra": "mean: 174.87719531753652 nsec\nrounds: 58167"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 410060.3325870243,
            "unit": "iter/sec",
            "range": "stddev: 3.0881115397796674e-7",
            "extra": "mean: 2.4386655341449845 usec\nrounds: 42384"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 57096.27966890055,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012905027390861018",
            "extra": "mean: 17.514275987839614 usec\nrounds: 58133"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4094.7992705080046,
            "unit": "iter/sec",
            "range": "stddev: 0.0000057870992571593415",
            "extra": "mean: 244.2122150412367 usec\nrounds: 4162"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4498167.479498841,
            "unit": "iter/sec",
            "range": "stddev: 1.2696735270743516e-8",
            "extra": "mean: 222.31275392865794 nsec\nrounds: 45702"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 390590.6883772988,
            "unit": "iter/sec",
            "range": "stddev: 2.94615218629074e-7",
            "extra": "mean: 2.560224884403888 usec\nrounds: 135981"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2638593.2027680525,
            "unit": "iter/sec",
            "range": "stddev: 4.661084502565045e-8",
            "extra": "mean: 378.9898340338828 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 49491.750492976615,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019810976593549403",
            "extra": "mean: 20.20538756538648 usec\nrounds: 51212"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 113971.5159751543,
            "unit": "iter/sec",
            "range": "stddev: 0.000003738769788753611",
            "extra": "mean: 8.774122125549328 usec\nrounds: 122325"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14098.390440104213,
            "unit": "iter/sec",
            "range": "stddev: 0.000003848499028002929",
            "extra": "mean: 70.93008271039258 usec\nrounds: 14847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1329.2835740273981,
            "unit": "iter/sec",
            "range": "stddev: 0.00004076991061385683",
            "extra": "mean: 752.2849296709874 usec\nrounds: 1365"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51131.76061605637,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017322526920118155",
            "extra": "mean: 19.557315999910642 usec\nrounds: 53519"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49589.440013147716,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018545528209870964",
            "extra": "mean: 20.165583635041425 usec\nrounds: 51611"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35169.27854736823,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023611066951506134",
            "extra": "mean: 28.433907128721337 usec\nrounds: 37482"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 70113.93204120926,
            "unit": "iter/sec",
            "range": "stddev: 0.000002690419116779554",
            "extra": "mean: 14.26250063129041 usec\nrounds: 114851"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 248932.9349427934,
            "unit": "iter/sec",
            "range": "stddev: 6.904932125628417e-7",
            "extra": "mean: 4.017146225467543 usec\nrounds: 130822"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 31497.68335673561,
            "unit": "iter/sec",
            "range": "stddev: 0.0000074337026567156895",
            "extra": "mean: 31.748366655230708 usec\nrounds: 47827"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32089.26163839503,
            "unit": "iter/sec",
            "range": "stddev: 0.0000046149550783877045",
            "extra": "mean: 31.163072907962857 usec\nrounds: 33316"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8436.622528717699,
            "unit": "iter/sec",
            "range": "stddev: 0.00004173065635293402",
            "extra": "mean: 118.53084532300299 usec\nrounds: 12303"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3142.6994414168903,
            "unit": "iter/sec",
            "range": "stddev: 0.000006735540499042559",
            "extra": "mean: 318.19778462465655 usec\nrounds: 3213"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 17967.506689431888,
            "unit": "iter/sec",
            "range": "stddev: 0.00000514745385741933",
            "extra": "mean: 55.65602491681158 usec\nrounds: 23519"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28119.939807925213,
            "unit": "iter/sec",
            "range": "stddev: 0.000001966016949732049",
            "extra": "mean: 35.561953789039194 usec\nrounds: 28716"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 31519.453727874938,
            "unit": "iter/sec",
            "range": "stddev: 0.000006325817447326667",
            "extra": "mean: 31.726438174771655 usec\nrounds: 45370"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 25217.8459257869,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020995576257491978",
            "extra": "mean: 39.65445751960259 usec\nrounds: 25953"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28130.5373190445,
            "unit": "iter/sec",
            "range": "stddev: 0.000004965055257359619",
            "extra": "mean: 35.54855666844997 usec\nrounds: 40940"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23010.2581852601,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020616719282991398",
            "extra": "mean: 43.45887785998766 usec\nrounds: 23514"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16488.003573720736,
            "unit": "iter/sec",
            "range": "stddev: 0.0000066615776779583976",
            "extra": "mean: 60.650156674749965 usec\nrounds: 21018"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21058.49707915255,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025475809767406473",
            "extra": "mean: 47.48676965128619 usec\nrounds: 21563"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 36087.44415993335,
            "unit": "iter/sec",
            "range": "stddev: 0.000003283992021774987",
            "extra": "mean: 27.710468925651032 usec\nrounds: 55544"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47465.03523630742,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015203978376471636",
            "extra": "mean: 21.06813984275883 usec\nrounds: 48905"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4572.578881455651,
            "unit": "iter/sec",
            "range": "stddev: 0.00006896276053259467",
            "extra": "mean: 218.69496971513294 usec\nrounds: 6604"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 925.6130181257796,
            "unit": "iter/sec",
            "range": "stddev: 0.000013225120864435245",
            "extra": "mean: 1.080365099039815 msec\nrounds: 939"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1001.7530307910316,
            "unit": "iter/sec",
            "range": "stddev: 0.00017457699256401174",
            "extra": "mean: 998.2500369480815 usec\nrounds: 1218"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 273.9632895107449,
            "unit": "iter/sec",
            "range": "stddev: 0.000021766609584172773",
            "extra": "mean: 3.650124079710978 msec\nrounds: 276"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 52906.34489979477,
            "unit": "iter/sec",
            "range": "stddev: 0.000003538847995395492",
            "extra": "mean: 18.901324631176312 usec\nrounds: 106068"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 21963.119645895687,
            "unit": "iter/sec",
            "range": "stddev: 0.000010667517618183819",
            "extra": "mean: 45.53087248636251 usec\nrounds: 35157"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5622.3315182247015,
            "unit": "iter/sec",
            "range": "stddev: 0.00005803578301703448",
            "extra": "mean: 177.86215500784954 usec\nrounds: 8077"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1722.2064921670124,
            "unit": "iter/sec",
            "range": "stddev: 0.00009251130826645144",
            "extra": "mean: 580.6504647080521 usec\nrounds: 1941"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 31342.72214407138,
            "unit": "iter/sec",
            "range": "stddev: 0.000007064928957643307",
            "extra": "mean: 31.90533341052365 usec\nrounds: 47374"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1691.671813067613,
            "unit": "iter/sec",
            "range": "stddev: 0.00010493777758146728",
            "extra": "mean: 591.1312065823442 usec\nrounds: 1975"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1155.5933989431276,
            "unit": "iter/sec",
            "range": "stddev: 0.00013082082398998066",
            "extra": "mean: 865.3562757580402 usec\nrounds: 1349"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1765762096663,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5446656.151461246,
            "unit": "iter/sec",
            "range": "stddev: 4.654312883874808e-8",
            "extra": "mean: 183.5988856634124 nsec\nrounds: 55118"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5062155.239358354,
            "unit": "iter/sec",
            "range": "stddev: 1.2638856310209988e-8",
            "extra": "mean: 197.54431713690315 nsec\nrounds: 51555"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3247779.173234494,
            "unit": "iter/sec",
            "range": "stddev: 3.787768383689855e-8",
            "extra": "mean: 307.90270725323495 nsec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 698456.6313038686,
            "unit": "iter/sec",
            "range": "stddev: 9.844933869463199e-8",
            "extra": "mean: 1.4317281205179961 usec\nrounds: 71094"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 72321.15127046777,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013795640519492112",
            "extra": "mean: 13.827213511302999 usec\nrounds: 75729"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 401360.6856959698,
            "unit": "iter/sec",
            "range": "stddev: 1.6937803002118758e-7",
            "extra": "mean: 2.4915245454744874 usec\nrounds: 41417"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 400742.55466179585,
            "unit": "iter/sec",
            "range": "stddev: 1.933024672549239e-7",
            "extra": "mean: 2.4953676328283523 usec\nrounds: 41539"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 351746.8166285396,
            "unit": "iter/sec",
            "range": "stddev: 3.874099176045916e-7",
            "extra": "mean: 2.842953945070226 usec\nrounds: 183824"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 211037.64439657683,
            "unit": "iter/sec",
            "range": "stddev: 5.159448766844769e-7",
            "extra": "mean: 4.738491101240801 usec\nrounds: 109566"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 40262.357933574014,
            "unit": "iter/sec",
            "range": "stddev: 0.000002416244972235887",
            "extra": "mean: 24.83709477844861 usec\nrounds: 41539"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3784724.4683790123,
            "unit": "iter/sec",
            "range": "stddev: 8.649379323035386e-8",
            "extra": "mean: 264.22002667692345 nsec\nrounds: 193799"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3502300.327993014,
            "unit": "iter/sec",
            "range": "stddev: 3.381796938988481e-8",
            "extra": "mean: 285.52662717332714 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2260162.3708549687,
            "unit": "iter/sec",
            "range": "stddev: 6.234111872914381e-8",
            "extra": "mean: 442.4460883408711 nsec\nrounds: 191939"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 508344.6478787809,
            "unit": "iter/sec",
            "range": "stddev: 1.2938577540270114e-7",
            "extra": "mean: 1.9671693292588075 usec\nrounds: 52480"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1409464.4593507356,
            "unit": "iter/sec",
            "range": "stddev: 7.982674354310833e-8",
            "extra": "mean: 709.4893336017001 nsec\nrounds: 144238"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1385138.2577652547,
            "unit": "iter/sec",
            "range": "stddev: 8.741528008798816e-8",
            "extra": "mean: 721.949591958683 nsec\nrounds: 143001"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1208212.7385681483,
            "unit": "iter/sec",
            "range": "stddev: 8.266451878315735e-8",
            "extra": "mean: 827.6688103662034 nsec\nrounds: 124147"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 431983.65562003484,
            "unit": "iter/sec",
            "range": "stddev: 1.4994252292964343e-7",
            "extra": "mean: 2.3149023973249534 usec\nrounds: 44049"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2769459.0135221193,
            "unit": "iter/sec",
            "range": "stddev: 4.070901515020649e-8",
            "extra": "mean: 361.08135022686133 nsec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 505848.6560690204,
            "unit": "iter/sec",
            "range": "stddev: 1.4339521950463298e-7",
            "extra": "mean: 1.9768758659379404 usec\nrounds: 51852"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3004497.8815063667,
            "unit": "iter/sec",
            "range": "stddev: 3.86945618401541e-8",
            "extra": "mean: 332.8343168937864 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2868885.830968853,
            "unit": "iter/sec",
            "range": "stddev: 3.9689193435737793e-8",
            "extra": "mean: 348.5673738582859 nsec\nrounds: 198060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1835173.8141862878,
            "unit": "iter/sec",
            "range": "stddev: 6.664792451139934e-8",
            "extra": "mean: 544.907513538972 nsec\nrounds: 186916"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 494317.196198361,
            "unit": "iter/sec",
            "range": "stddev: 1.9439378551169917e-7",
            "extra": "mean: 2.0229925393870203 usec\nrounds: 51162"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47871.85435090851,
            "unit": "iter/sec",
            "range": "stddev: 0.000001780533928363908",
            "extra": "mean: 20.88910098760405 usec\nrounds: 49412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1175350.3132725563,
            "unit": "iter/sec",
            "range": "stddev: 8.059397190536666e-8",
            "extra": "mean: 850.8101701319301 nsec\nrounds: 121139"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1158535.5207364634,
            "unit": "iter/sec",
            "range": "stddev: 8.272264188074151e-8",
            "extra": "mean: 863.1586879307409 nsec\nrounds: 119689"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 898830.9483281727,
            "unit": "iter/sec",
            "range": "stddev: 1.0771696205187949e-7",
            "extra": "mean: 1.112556261953351 usec\nrounds: 93102"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 345495.1174482036,
            "unit": "iter/sec",
            "range": "stddev: 5.198480636375722e-7",
            "extra": "mean: 2.894396908951743 usec\nrounds: 184468"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 43712.32038388136,
            "unit": "iter/sec",
            "range": "stddev: 0.000001979455232028027",
            "extra": "mean: 22.876845503007058 usec\nrounds: 45619"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2816081.423949415,
            "unit": "iter/sec",
            "range": "stddev: 4.242271312249619e-8",
            "extra": "mean: 355.10336863690964 nsec\nrounds: 193837"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2307180.4743959904,
            "unit": "iter/sec",
            "range": "stddev: 5.088666106134278e-8",
            "extra": "mean: 433.42946557391565 nsec\nrounds: 196117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 793245.2195422661,
            "unit": "iter/sec",
            "range": "stddev: 9.492803291255645e-8",
            "extra": "mean: 1.2606442186655145 usec\nrounds: 80561"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 108038.06820163298,
            "unit": "iter/sec",
            "range": "stddev: 8.573734427186504e-7",
            "extra": "mean: 9.255996674558137 usec\nrounds: 110060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 366736.13486727903,
            "unit": "iter/sec",
            "range": "stddev: 3.739995502727204e-7",
            "extra": "mean: 2.726756119524185 usec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 357117.05699750956,
            "unit": "iter/sec",
            "range": "stddev: 3.787086180495044e-7",
            "extra": "mean: 2.800202287752875 usec\nrounds: 184502"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 279950.03261332837,
            "unit": "iter/sec",
            "range": "stddev: 0.000001445188674902158",
            "extra": "mean: 3.5720660243009026 usec\nrounds: 150331"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 117911.10620289869,
            "unit": "iter/sec",
            "range": "stddev: 0.00000182084115370607",
            "extra": "mean: 8.480965298376756 usec\nrounds: 124922"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2653082.879853066,
            "unit": "iter/sec",
            "range": "stddev: 5.048903815547226e-7",
            "extra": "mean: 376.9200003489459 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2809778.0357068814,
            "unit": "iter/sec",
            "range": "stddev: 1.9257135500277345e-7",
            "extra": "mean: 355.8999989650147 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1913912.2251423676,
            "unit": "iter/sec",
            "range": "stddev: 1.6282279529581662e-7",
            "extra": "mean: 522.490000776088 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 487265.32048049587,
            "unit": "iter/sec",
            "range": "stddev: 3.6341483289936826e-7",
            "extra": "mean: 2.0522700015135342 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 56749.3732026072,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027397497080572553",
            "extra": "mean: 17.621340000175678 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 814956.0747231373,
            "unit": "iter/sec",
            "range": "stddev: 0.000001370298161087338",
            "extra": "mean: 1.2270599987118658 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 899078.4440383868,
            "unit": "iter/sec",
            "range": "stddev: 4.328873311997506e-7",
            "extra": "mean: 1.112250000687709 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 723767.0622167757,
            "unit": "iter/sec",
            "range": "stddev: 4.406437695464551e-7",
            "extra": "mean: 1.3816600011296032 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 372922.8199866544,
            "unit": "iter/sec",
            "range": "stddev: 4.308828962203009e-7",
            "extra": "mean: 2.681519999327975 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 54889.38142921977,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014005145185772548",
            "extra": "mean: 18.218460000127834 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1637787.977697573,
            "unit": "iter/sec",
            "range": "stddev: 6.952545377528182e-8",
            "extra": "mean: 610.5796437740443 nsec\nrounds: 168322"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 358905.75910944893,
            "unit": "iter/sec",
            "range": "stddev: 3.1727207825924947e-7",
            "extra": "mean: 2.7862467364170893 usec\nrounds: 184843"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43291.19130231785,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012986697646638442",
            "extra": "mean: 23.09938742541509 usec\nrounds: 44264"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4386.457558731218,
            "unit": "iter/sec",
            "range": "stddev: 0.0000046426413644487665",
            "extra": "mean: 227.97439314316534 usec\nrounds: 4492"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 203658.12805169635,
            "unit": "iter/sec",
            "range": "stddev: 5.396469216164043e-7",
            "extra": "mean: 4.910189490429575 usec\nrounds: 106417"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 133177.54395170073,
            "unit": "iter/sec",
            "range": "stddev: 9.476816918108946e-7",
            "extra": "mean: 7.508773403740411 usec\nrounds: 138065"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31892.517248377113,
            "unit": "iter/sec",
            "range": "stddev: 0.000001914906149802763",
            "extra": "mean: 31.35531736839889 usec\nrounds: 32801"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3739.524960470768,
            "unit": "iter/sec",
            "range": "stddev: 0.00000516277507019825",
            "extra": "mean: 267.41364493368974 usec\nrounds: 3819"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 142408.80535078899,
            "unit": "iter/sec",
            "range": "stddev: 8.588685603699967e-7",
            "extra": "mean: 7.022037700103912 usec\nrounds: 148302"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17434.396838992718,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027287517834583597",
            "extra": "mean: 57.3578776045444 usec\nrounds: 17901"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1646.2393991394697,
            "unit": "iter/sec",
            "range": "stddev: 0.000011493845706254621",
            "extra": "mean: 607.4450657193145 usec\nrounds: 1689"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1254462.636342072,
            "unit": "iter/sec",
            "range": "stddev: 7.702679583218228e-8",
            "extra": "mean: 797.1540730108348 nsec\nrounds: 130481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 105150.4396863687,
            "unit": "iter/sec",
            "range": "stddev: 8.248338295763577e-7",
            "extra": "mean: 9.51018372327012 usec\nrounds: 106987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17212.241659577572,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023013040785605885",
            "extra": "mean: 58.09818498821508 usec\nrounds: 17493"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2372.5432073087945,
            "unit": "iter/sec",
            "range": "stddev: 0.000005622460809843725",
            "extra": "mean: 421.48863587370136 usec\nrounds: 2414"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 700368.9763881112,
            "unit": "iter/sec",
            "range": "stddev: 1.1720465688587481e-7",
            "extra": "mean: 1.4278188122454036 usec\nrounds: 72591"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 103617.23508876615,
            "unit": "iter/sec",
            "range": "stddev: 9.517761292438596e-7",
            "extra": "mean: 9.650904110144673 usec\nrounds: 106758"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 22637.320612013318,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029399031676140105",
            "extra": "mean: 44.174839290358136 usec\nrounds: 23670"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4256.201782543689,
            "unit": "iter/sec",
            "range": "stddev: 0.00005894589487653398",
            "extra": "mean: 234.95126666723897 usec\nrounds: 4590"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2116908.5720351785,
            "unit": "iter/sec",
            "range": "stddev: 5.3072595414561266e-8",
            "extra": "mean: 472.3869576656341 nsec\nrounds: 184843"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 991525.4947555477,
            "unit": "iter/sec",
            "range": "stddev: 8.9088442577479e-8",
            "extra": "mean: 1.0085469363009636 usec\nrounds: 101544"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 367455.8406171238,
            "unit": "iter/sec",
            "range": "stddev: 3.201960251333717e-7",
            "extra": "mean: 2.721415444970339 usec\nrounds: 187653"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 120816.5182026825,
            "unit": "iter/sec",
            "range": "stddev: 9.395556733237736e-7",
            "extra": "mean: 8.277013895752185 usec\nrounds: 125866"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 886039.8276949252,
            "unit": "iter/sec",
            "range": "stddev: 9.620545156840036e-8",
            "extra": "mean: 1.1286174376625602 usec\nrounds: 90901"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 719647.4623829258,
            "unit": "iter/sec",
            "range": "stddev: 1.1360502102874926e-7",
            "extra": "mean: 1.389569271444023 usec\nrounds: 73557"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 460658.32347320515,
            "unit": "iter/sec",
            "range": "stddev: 1.3512459588990798e-7",
            "extra": "mean: 2.170806320094996 usec\nrounds: 46993"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 180456.7485469182,
            "unit": "iter/sec",
            "range": "stddev: 8.915230261606355e-7",
            "extra": "mean: 5.54149405911524 usec\nrounds: 179857"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5661513.663201325,
            "unit": "iter/sec",
            "range": "stddev: 1.1045423802432882e-8",
            "extra": "mean: 176.63120845220686 nsec\nrounds: 57495"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 396561.7410234019,
            "unit": "iter/sec",
            "range": "stddev: 2.817063278314371e-7",
            "extra": "mean: 2.521675432983804 usec\nrounds: 138447"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 56852.54790053425,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011555903762597296",
            "extra": "mean: 17.589361197136476 usec\nrounds: 57697"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 3962.3649874777134,
            "unit": "iter/sec",
            "range": "stddev: 0.000005693579054411636",
            "extra": "mean: 252.37452964588226 usec\nrounds: 4014"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4576283.826485347,
            "unit": "iter/sec",
            "range": "stddev: 1.2351128983400276e-8",
            "extra": "mean: 218.51791495372237 nsec\nrounds: 46445"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 378012.58583897783,
            "unit": "iter/sec",
            "range": "stddev: 3.665433929770486e-7",
            "extra": "mean: 2.6454145641223974 usec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2548158.8193578143,
            "unit": "iter/sec",
            "range": "stddev: 4.9108998182905144e-8",
            "extra": "mean: 392.4402169924438 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48724.27702789267,
            "unit": "iter/sec",
            "range": "stddev: 0.000001912185218257841",
            "extra": "mean: 20.523649831223572 usec\nrounds: 51241"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 115802.25507310097,
            "unit": "iter/sec",
            "range": "stddev: 9.470146333493464e-7",
            "extra": "mean: 8.635410418983145 usec\nrounds: 121125"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14265.29208408399,
            "unit": "iter/sec",
            "range": "stddev: 0.000003035892024137978",
            "extra": "mean: 70.10021204653185 usec\nrounds: 14709"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1332.2305498377307,
            "unit": "iter/sec",
            "range": "stddev: 0.000013845302235757372",
            "extra": "mean: 750.62082919642 usec\nrounds: 1370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51133.88044002806,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016642786701612437",
            "extra": "mean: 19.556505225001292 usec\nrounds: 53491"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49677.20114398639,
            "unit": "iter/sec",
            "range": "stddev: 0.000001863018016906952",
            "extra": "mean: 20.129958551842723 usec\nrounds: 51317"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 36148.98004039234,
            "unit": "iter/sec",
            "range": "stddev: 0.000002215036697807726",
            "extra": "mean: 27.663297799346335 usec\nrounds: 38170"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 69397.94012355614,
            "unit": "iter/sec",
            "range": "stddev: 0.000002752329711622216",
            "extra": "mean: 14.409649597950592 usec\nrounds: 155958"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 250526.90539911084,
            "unit": "iter/sec",
            "range": "stddev: 5.010074188862067e-7",
            "extra": "mean: 3.9915872445193634 usec\nrounds: 130822"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 31987.12993413242,
            "unit": "iter/sec",
            "range": "stddev: 0.000007150004737628066",
            "extra": "mean: 31.26257348062143 usec\nrounds: 48455"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32441.688917974567,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017865447925313855",
            "extra": "mean: 30.824535754855297 usec\nrounds: 33450"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8902.327053816074,
            "unit": "iter/sec",
            "range": "stddev: 0.000025093014665835395",
            "extra": "mean: 112.33017995798522 usec\nrounds: 12853"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3170.5623065623904,
            "unit": "iter/sec",
            "range": "stddev: 0.000007138255963349434",
            "extra": "mean: 315.40146614693947 usec\nrounds: 3205"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 15872.738866252312,
            "unit": "iter/sec",
            "range": "stddev: 0.00001630240349160377",
            "extra": "mean: 63.001099459031714 usec\nrounds: 23105"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 27135.445313041742,
            "unit": "iter/sec",
            "range": "stddev: 0.000002192022271402815",
            "extra": "mean: 36.85216838948958 usec\nrounds: 27573"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 28177.598188885768,
            "unit": "iter/sec",
            "range": "stddev: 0.000010817350977423447",
            "extra": "mean: 35.48918517811909 usec\nrounds: 46512"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 25030.822190215913,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020132696108312078",
            "extra": "mean: 39.95074522126092 usec\nrounds: 26470"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28060.77947885184,
            "unit": "iter/sec",
            "range": "stddev: 0.000004255153118084076",
            "extra": "mean: 35.63692878715844 usec\nrounds: 38406"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22983.14245825792,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023230383778367036",
            "extra": "mean: 43.510151051632924 usec\nrounds: 23535"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16506.130520416405,
            "unit": "iter/sec",
            "range": "stddev: 0.000006584453201590488",
            "extra": "mean: 60.58355098810722 usec\nrounds: 20799"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21128.72415804046,
            "unit": "iter/sec",
            "range": "stddev: 0.00000233779759988364",
            "extra": "mean: 47.32893441743636 usec\nrounds: 21530"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 30336.30590356985,
            "unit": "iter/sec",
            "range": "stddev: 0.000008421239917611268",
            "extra": "mean: 32.96380261916874 usec\nrounds: 60021"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47579.50628740444,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014675544143136776",
            "extra": "mean: 21.017452219018224 usec\nrounds: 48335"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4694.47553006706,
            "unit": "iter/sec",
            "range": "stddev: 0.00006487629166806604",
            "extra": "mean: 213.01634092993453 usec\nrounds: 6450"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 934.2147768968929,
            "unit": "iter/sec",
            "range": "stddev: 0.000011733104152964007",
            "extra": "mean: 1.07041766489888 msec\nrounds: 943"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1038.7008172345288,
            "unit": "iter/sec",
            "range": "stddev: 0.0001505672604055301",
            "extra": "mean: 962.741131428425 usec\nrounds: 1225"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 271.7746876411284,
            "unit": "iter/sec",
            "range": "stddev: 0.0000245784332981046",
            "extra": "mean: 3.6795185330889777 msec\nrounds: 272"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 54167.909052406016,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029742001713462",
            "extra": "mean: 18.461115030903752 usec\nrounds: 87559"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 23342.978718046554,
            "unit": "iter/sec",
            "range": "stddev: 0.000006887983673751277",
            "extra": "mean: 42.839434164710774 usec\nrounds: 33485"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5647.629674712655,
            "unit": "iter/sec",
            "range": "stddev: 0.00006010516214139702",
            "extra": "mean: 177.06543410194095 usec\nrounds: 8111"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1741.8469942405077,
            "unit": "iter/sec",
            "range": "stddev: 0.00008977996848291583",
            "extra": "mean: 574.1032382904716 usec\nrounds: 1943"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 29748.67356136897,
            "unit": "iter/sec",
            "range": "stddev: 0.000010079334978294102",
            "extra": "mean: 33.614944139848305 usec\nrounds: 48335"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1741.3760310887326,
            "unit": "iter/sec",
            "range": "stddev: 0.00008221968925781939",
            "extra": "mean: 574.2585071501105 usec\nrounds: 1958"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1159.5485064615239,
            "unit": "iter/sec",
            "range": "stddev: 0.00013439025400656697",
            "extra": "mean: 862.4046294118373 usec\nrounds: 1360"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1766366946161,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5245001.237905669,
            "unit": "iter/sec",
            "range": "stddev: 1.811380259514802e-8",
            "extra": "mean: 190.6577243058087 nsec\nrounds: 53320"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4888464.427654448,
            "unit": "iter/sec",
            "range": "stddev: 1.2303313963224738e-8",
            "extra": "mean: 204.5632150543957 nsec\nrounds: 49685"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3150755.1164493845,
            "unit": "iter/sec",
            "range": "stddev: 3.684434104148319e-8",
            "extra": "mean: 317.38423426804377 nsec\nrounds: 189072"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 713601.1770497592,
            "unit": "iter/sec",
            "range": "stddev: 3.7315163227294315e-7",
            "extra": "mean: 1.4013429800302224 usec\nrounds: 72543"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 71521.90521979325,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015173676654459626",
            "extra": "mean: 13.98173044925062 usec\nrounds: 75444"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 409897.30638823955,
            "unit": "iter/sec",
            "range": "stddev: 1.7003610467214747e-7",
            "extra": "mean: 2.4396354511606475 usec\nrounds: 42402"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 401148.2405870509,
            "unit": "iter/sec",
            "range": "stddev: 1.773340965073821e-7",
            "extra": "mean: 2.4928440382452264 usec\nrounds: 41850"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 352725.01896248956,
            "unit": "iter/sec",
            "range": "stddev: 3.7512089151151885e-7",
            "extra": "mean: 2.8350696611808663 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 210678.68973478785,
            "unit": "iter/sec",
            "range": "stddev: 4.819198495345808e-7",
            "extra": "mean: 4.746564549356399 usec\nrounds: 109087"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39189.66069382031,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021768365892877275",
            "extra": "mean: 25.51693437237865 usec\nrounds: 41263"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3621554.5733506167,
            "unit": "iter/sec",
            "range": "stddev: 3.2411428113039414e-8",
            "extra": "mean: 276.1245149689891 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3425050.0074521266,
            "unit": "iter/sec",
            "range": "stddev: 3.291658928872974e-8",
            "extra": "mean: 291.9665399992267 nsec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2214831.540960496,
            "unit": "iter/sec",
            "range": "stddev: 5.4735690739323816e-8",
            "extra": "mean: 451.5016070099532 nsec\nrounds: 189430"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 515382.138046409,
            "unit": "iter/sec",
            "range": "stddev: 2.1089310984859663e-7",
            "extra": "mean: 1.9403078340870823 usec\nrounds: 52450"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1505161.1517712602,
            "unit": "iter/sec",
            "range": "stddev: 7.912205459320325e-8",
            "extra": "mean: 664.3806869604548 nsec\nrounds: 154274"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1403599.8347405416,
            "unit": "iter/sec",
            "range": "stddev: 4.154837344402081e-7",
            "extra": "mean: 712.4537743942051 nsec\nrounds: 152859"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1287959.8738510725,
            "unit": "iter/sec",
            "range": "stddev: 8.370029479620697e-8",
            "extra": "mean: 776.4217040473054 nsec\nrounds: 131510"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 440790.3612832008,
            "unit": "iter/sec",
            "range": "stddev: 1.7225626353213787e-7",
            "extra": "mean: 2.268652148129708 usec\nrounds: 45062"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2679795.3883162457,
            "unit": "iter/sec",
            "range": "stddev: 4.2440350117139304e-8",
            "extra": "mean: 373.1628184599369 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 659208.4474752091,
            "unit": "iter/sec",
            "range": "stddev: 1.2768263084887082e-7",
            "extra": "mean: 1.5169708516783127 usec\nrounds: 67582"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2935479.09041331,
            "unit": "iter/sec",
            "range": "stddev: 4.0590748358190416e-8",
            "extra": "mean: 340.6598954377842 nsec\nrounds: 189072"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2825746.7365620276,
            "unit": "iter/sec",
            "range": "stddev: 4.1236080229654526e-8",
            "extra": "mean: 353.88875692973204 nsec\nrounds: 192308"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1774972.1328395554,
            "unit": "iter/sec",
            "range": "stddev: 6.393623745016804e-8",
            "extra": "mean: 563.389126791662 nsec\nrounds: 183184"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 497484.3047412969,
            "unit": "iter/sec",
            "range": "stddev: 1.40799700786023e-7",
            "extra": "mean: 2.0101136668422384 usec\nrounds: 50641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36135.04133209691,
            "unit": "iter/sec",
            "range": "stddev: 0.000002177718196756904",
            "extra": "mean: 27.67396862257775 usec\nrounds: 37511"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1215572.4024670462,
            "unit": "iter/sec",
            "range": "stddev: 7.84941235759419e-8",
            "extra": "mean: 822.6577026349461 nsec\nrounds: 125866"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1193882.5563228496,
            "unit": "iter/sec",
            "range": "stddev: 8.095719984075589e-8",
            "extra": "mean: 837.6033259753851 nsec\nrounds: 123993"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 904661.0453066903,
            "unit": "iter/sec",
            "range": "stddev: 1.0271163711134138e-7",
            "extra": "mean: 1.1053863822123335 usec\nrounds: 92166"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 369787.31261820224,
            "unit": "iter/sec",
            "range": "stddev: 3.61278524823859e-7",
            "extra": "mean: 2.704257192924516 usec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 38324.21430576688,
            "unit": "iter/sec",
            "range": "stddev: 0.0000035744458033957802",
            "extra": "mean: 26.09316376381717 usec\nrounds: 36113"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2804789.859166381,
            "unit": "iter/sec",
            "range": "stddev: 4.509246747619317e-8",
            "extra": "mean: 356.5329490663935 nsec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2246132.7379800854,
            "unit": "iter/sec",
            "range": "stddev: 5.195891093279235e-8",
            "extra": "mean: 445.2096632985694 nsec\nrounds: 193837"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 776653.4724542031,
            "unit": "iter/sec",
            "range": "stddev: 1.544370868577607e-7",
            "extra": "mean: 1.2875755217317593 usec\nrounds: 79593"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107977.7597707855,
            "unit": "iter/sec",
            "range": "stddev: 8.512860285922467e-7",
            "extra": "mean: 9.261166393179424 usec\nrounds: 109686"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 446868.3023256167,
            "unit": "iter/sec",
            "range": "stddev: 1.9758184598654141e-7",
            "extra": "mean: 2.2377957774041044 usec\nrounds: 45849"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 440445.1897336715,
            "unit": "iter/sec",
            "range": "stddev: 1.7038784651937895e-7",
            "extra": "mean: 2.270430063283666 usec\nrounds: 45188"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 331816.50621024444,
            "unit": "iter/sec",
            "range": "stddev: 4.412502127144178e-7",
            "extra": "mean: 3.013713848720905 usec\nrounds: 171792"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 127161.21460187291,
            "unit": "iter/sec",
            "range": "stddev: 9.439453697843771e-7",
            "extra": "mean: 7.864033094767808 usec\nrounds: 130141"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2794232.7221515807,
            "unit": "iter/sec",
            "range": "stddev: 5.510178723642417e-7",
            "extra": "mean: 357.87999763670086 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2769776.1364418357,
            "unit": "iter/sec",
            "range": "stddev: 1.6542682808704484e-7",
            "extra": "mean: 361.0400085563015 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1983654.705234565,
            "unit": "iter/sec",
            "range": "stddev: 1.6611899855590838e-7",
            "extra": "mean: 504.1199949573638 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 458374.96927728143,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013349026566347784",
            "extra": "mean: 2.181619998964379 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 54347.02858785226,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010982455686988888",
            "extra": "mean: 18.40027000525879 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 880041.5443057427,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013145471393764774",
            "extra": "mean: 1.1363099918071384 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 952471.6612881047,
            "unit": "iter/sec",
            "range": "stddev: 4.354283304548894e-7",
            "extra": "mean: 1.0499000029540184 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 795570.2658591251,
            "unit": "iter/sec",
            "range": "stddev: 4.558381582143451e-7",
            "extra": "mean: 1.2569599982725776 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 373965.9833402986,
            "unit": "iter/sec",
            "range": "stddev: 8.283278661258433e-7",
            "extra": "mean: 2.6740400051039614 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 55109.08016304897,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020648853408314235",
            "extra": "mean: 18.145829998275076 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1594430.2773544786,
            "unit": "iter/sec",
            "range": "stddev: 7.231698712144809e-8",
            "extra": "mean: 627.1832730492551 nsec\nrounds: 163640"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 360173.7585536548,
            "unit": "iter/sec",
            "range": "stddev: 3.319390558601467e-7",
            "extra": "mean: 2.7764376950050087 usec\nrounds: 184536"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43441.81418633444,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014265945584431367",
            "extra": "mean: 23.019296471153627 usec\nrounds: 44011"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4426.110532963674,
            "unit": "iter/sec",
            "range": "stddev: 0.000005766769109182844",
            "extra": "mean: 225.93199888535347 usec\nrounds: 4485"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 208523.40513860885,
            "unit": "iter/sec",
            "range": "stddev: 5.677849494384793e-7",
            "extra": "mean: 4.795624737353987 usec\nrounds: 108027"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 135100.4886653143,
            "unit": "iter/sec",
            "range": "stddev: 9.385626666608263e-7",
            "extra": "mean: 7.401897727234053 usec\nrounds: 139998"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 32014.14166627192,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018699977015360843",
            "extra": "mean: 31.236195879445894 usec\nrounds: 32714"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3722.1449331060307,
            "unit": "iter/sec",
            "range": "stddev: 0.000005561422036880177",
            "extra": "mean: 268.6622949863284 usec\nrounds: 3790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 143849.29220330625,
            "unit": "iter/sec",
            "range": "stddev: 8.349201072852323e-7",
            "extra": "mean: 6.951719988908057 usec\nrounds: 148987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17273.789043843874,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027056060252486064",
            "extra": "mean: 57.89117821584057 usec\nrounds: 17664"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1669.0403945183157,
            "unit": "iter/sec",
            "range": "stddev: 0.000009427129786507677",
            "extra": "mean: 599.1466733126009 usec\nrounds: 1705"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1268434.08076577,
            "unit": "iter/sec",
            "range": "stddev: 7.771905830533396e-8",
            "extra": "mean: 788.3736452400441 nsec\nrounds: 129635"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104875.13661035188,
            "unit": "iter/sec",
            "range": "stddev: 8.957351068937106e-7",
            "extra": "mean: 9.535148485339786 usec\nrounds: 106987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17181.893933850315,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023925117780973767",
            "extra": "mean: 58.20080160254537 usec\nrounds: 17475"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2367.9447854017085,
            "unit": "iter/sec",
            "range": "stddev: 0.000006078825726672862",
            "extra": "mean: 422.3071442226875 usec\nrounds: 2406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 695534.7773694011,
            "unit": "iter/sec",
            "range": "stddev: 1.1474815483999239e-7",
            "extra": "mean: 1.437742629897138 usec\nrounds: 71912"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 105022.27655604878,
            "unit": "iter/sec",
            "range": "stddev: 9.724748052560093e-7",
            "extra": "mean: 9.521789403092166 usec\nrounds: 107447"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 22234.018098634595,
            "unit": "iter/sec",
            "range": "stddev: 0.000002835998275004996",
            "extra": "mean: 44.97612602291669 usec\nrounds: 23464"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4290.509453215265,
            "unit": "iter/sec",
            "range": "stddev: 0.000025704549048704362",
            "extra": "mean: 233.07255488054219 usec\nrounds: 4610"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2120530.6042172695,
            "unit": "iter/sec",
            "range": "stddev: 6.094966296551194e-8",
            "extra": "mean: 471.5800837824387 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 983581.6149592802,
            "unit": "iter/sec",
            "range": "stddev: 8.767627518655358e-8",
            "extra": "mean: 1.016692448080587 usec\nrounds: 100716"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 364176.8623121258,
            "unit": "iter/sec",
            "range": "stddev: 3.1683465114547123e-7",
            "extra": "mean: 2.7459185453218824 usec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 124653.9878825599,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010186931353218368",
            "extra": "mean: 8.022206244553754 usec\nrounds: 128304"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 898645.6058177268,
            "unit": "iter/sec",
            "range": "stddev: 9.812553465925255e-8",
            "extra": "mean: 1.1127857227878342 usec\nrounds: 92679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 721126.6640111085,
            "unit": "iter/sec",
            "range": "stddev: 1.1064260261369592e-7",
            "extra": "mean: 1.386718935668961 usec\nrounds: 73998"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 462907.0157589092,
            "unit": "iter/sec",
            "range": "stddev: 1.49435267853734e-7",
            "extra": "mean: 2.160261058823125 usec\nrounds: 47374"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 206308.77261376064,
            "unit": "iter/sec",
            "range": "stddev: 5.987636702574095e-7",
            "extra": "mean: 4.847103626912377 usec\nrounds: 106758"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5451637.352064721,
            "unit": "iter/sec",
            "range": "stddev: 1.2065056002269327e-8",
            "extra": "mean: 183.4311300294516 nsec\nrounds: 55795"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 404376.2592272103,
            "unit": "iter/sec",
            "range": "stddev: 1.821477077505496e-7",
            "extra": "mean: 2.4729443857833253 usec\nrounds: 42241"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 57135.75236825278,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013692383131494851",
            "extra": "mean: 17.502176107785804 usec\nrounds: 58890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4032.588520483263,
            "unit": "iter/sec",
            "range": "stddev: 0.000008236601968877103",
            "extra": "mean: 247.97967730170512 usec\nrounds: 4137"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4350381.65168496,
            "unit": "iter/sec",
            "range": "stddev: 1.333384095377227e-8",
            "extra": "mean: 229.8648900407739 nsec\nrounds: 44108"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 383622.2629062232,
            "unit": "iter/sec",
            "range": "stddev: 3.042675530880758e-7",
            "extra": "mean: 2.606730882676775 usec\nrounds: 134899"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2443142.646679263,
            "unit": "iter/sec",
            "range": "stddev: 5.042307782209968e-8",
            "extra": "mean: 409.30888802552687 nsec\nrounds: 193837"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48120.98126099196,
            "unit": "iter/sec",
            "range": "stddev: 0.000001957299538849129",
            "extra": "mean: 20.78095611925155 usec\nrounds: 49931"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 117871.60215963646,
            "unit": "iter/sec",
            "range": "stddev: 9.59560703612171e-7",
            "extra": "mean: 8.483807648984657 usec\nrounds: 122625"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14368.450222767113,
            "unit": "iter/sec",
            "range": "stddev: 0.000003319294474680347",
            "extra": "mean: 69.5969283044513 usec\nrounds: 14729"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1364.1002855615784,
            "unit": "iter/sec",
            "range": "stddev: 0.000020636694700303587",
            "extra": "mean: 733.0839312802548 usec\nrounds: 1397"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51256.76239934469,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018112842893961944",
            "extra": "mean: 19.50962084200591 usec\nrounds: 53181"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 48849.55714261169,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019202468818687974",
            "extra": "mean: 20.471014651792935 usec\nrounds: 50847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35614.708608549,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023543895255997886",
            "extra": "mean: 28.078286726736227 usec\nrounds: 37384"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 67950.9716387822,
            "unit": "iter/sec",
            "range": "stddev: 0.0000036851234389596343",
            "extra": "mean: 14.71649302258486 usec\nrounds: 115514"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 253148.22241104022,
            "unit": "iter/sec",
            "range": "stddev: 4.204717091223009e-7",
            "extra": "mean: 3.950254876276739 usec\nrounds: 132223"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 32438.44773934,
            "unit": "iter/sec",
            "range": "stddev: 0.0000067663979213466284",
            "extra": "mean: 30.827615674940006 usec\nrounds: 49734"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32508.50951557401,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018015775259918878",
            "extra": "mean: 30.76117653197619 usec\nrounds: 33552"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8896.074284027825,
            "unit": "iter/sec",
            "range": "stddev: 0.000039273550399834137",
            "extra": "mean: 112.40913329550523 usec\nrounds: 12191"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3166.930349226793,
            "unit": "iter/sec",
            "range": "stddev: 0.000007518326979729438",
            "extra": "mean: 315.76318065983054 usec\nrounds: 3216"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 16822.82726116301,
            "unit": "iter/sec",
            "range": "stddev: 0.000012213150764705704",
            "extra": "mean: 59.44304036864177 usec\nrounds: 22691"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28301.004083190768,
            "unit": "iter/sec",
            "range": "stddev: 0.000002093803651076661",
            "extra": "mean: 35.33443538117945 usec\nrounds: 28823"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 31421.46992601361,
            "unit": "iter/sec",
            "range": "stddev: 0.000005485503012961996",
            "extra": "mean: 31.825372980788114 usec\nrounds: 46190"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24777.286997284227,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027084995686038914",
            "extra": "mean: 40.35954380758504 usec\nrounds: 25726"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 27779.08980559027,
            "unit": "iter/sec",
            "range": "stddev: 0.000004531787165742837",
            "extra": "mean: 35.99829969226565 usec\nrounds: 39991"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23268.217786491834,
            "unit": "iter/sec",
            "range": "stddev: 0.000002051877696271587",
            "extra": "mean: 42.97707753881096 usec\nrounds: 23614"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16571.875059682738,
            "unit": "iter/sec",
            "range": "stddev: 0.00000527397172955312",
            "extra": "mean: 60.34320174383119 usec\nrounds: 21785"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21142.955991736497,
            "unit": "iter/sec",
            "range": "stddev: 0.000002199718475111761",
            "extra": "mean: 47.29707616999437 usec\nrounds: 21452"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 36207.79683909779,
            "unit": "iter/sec",
            "range": "stddev: 0.000002980883157292447",
            "extra": "mean: 27.618360886299023 usec\nrounds: 53291"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 46851.058599617485,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014203212655030155",
            "extra": "mean: 21.34423489863609 usec\nrounds: 49366"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4505.719473338968,
            "unit": "iter/sec",
            "range": "stddev: 0.00007919590444225022",
            "extra": "mean: 221.94013762222733 usec\nrounds: 6358"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 919.1058661405276,
            "unit": "iter/sec",
            "range": "stddev: 0.000009306157087214969",
            "extra": "mean: 1.088013945770099 msec\nrounds: 922"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1053.6356737606823,
            "unit": "iter/sec",
            "range": "stddev: 0.00013851033791700582",
            "extra": "mean: 949.0946680181743 usec\nrounds: 1232"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 277.16288120232963,
            "unit": "iter/sec",
            "range": "stddev: 0.00002941338514129101",
            "extra": "mean: 3.6079867392848946 msec\nrounds: 280"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 60119.849729936286,
            "unit": "iter/sec",
            "range": "stddev: 0.000002873872013376352",
            "extra": "mean: 16.633441442253915 usec\nrounds: 95521"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25556.902909135257,
            "unit": "iter/sec",
            "range": "stddev: 0.0000062428493336796625",
            "extra": "mean: 39.128371835796756 usec\nrounds: 37175"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5924.420960709555,
            "unit": "iter/sec",
            "range": "stddev: 0.00005495897693411343",
            "extra": "mean: 168.79286712270223 usec\nrounds: 8474"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1653.9723691817833,
            "unit": "iter/sec",
            "range": "stddev: 0.00011033634964126751",
            "extra": "mean: 604.6050216030501 usec\nrounds: 1944"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 28940.752304181788,
            "unit": "iter/sec",
            "range": "stddev: 0.000010435565750307871",
            "extra": "mean: 34.55335194778282 usec\nrounds: 49908"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1694.0489052739952,
            "unit": "iter/sec",
            "range": "stddev: 0.0001020643776254065",
            "extra": "mean: 590.3017303023256 usec\nrounds: 1980"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1142.7670750903674,
            "unit": "iter/sec",
            "range": "stddev: 0.00013462618149299836",
            "extra": "mean: 875.0689635689079 usec\nrounds: 1345"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1766971934730,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5121337.609281773,
            "unit": "iter/sec",
            "range": "stddev: 1.2914146769079207e-8",
            "extra": "mean: 195.26148758239174 nsec\nrounds: 52508"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4735589.04188487,
            "unit": "iter/sec",
            "range": "stddev: 1.2897409850253298e-8",
            "extra": "mean: 211.16697229334423 nsec\nrounds: 48905"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3099746.6662966334,
            "unit": "iter/sec",
            "range": "stddev: 3.726473053825192e-8",
            "extra": "mean: 322.6070087833119 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 712460.6730900217,
            "unit": "iter/sec",
            "range": "stddev: 1.1244178339721724e-7",
            "extra": "mean: 1.4035862438032596 usec\nrounds: 72491"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 69858.73118806574,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015861513627786326",
            "extra": "mean: 14.314602956471017 usec\nrounds: 77256"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 402610.99140157324,
            "unit": "iter/sec",
            "range": "stddev: 1.808710110620909e-7",
            "extra": "mean: 2.4837871328817536 usec\nrounds: 41641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 402108.7798257529,
            "unit": "iter/sec",
            "range": "stddev: 1.8180686160473015e-7",
            "extra": "mean: 2.4868892453264406 usec\nrounds: 41833"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 356207.8117407972,
            "unit": "iter/sec",
            "range": "stddev: 4.074538566030254e-7",
            "extra": "mean: 2.8073499991843893 usec\nrounds: 185529"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 210048.0236812402,
            "unit": "iter/sec",
            "range": "stddev: 6.293474102142295e-7",
            "extra": "mean: 4.7608160385148715 usec\nrounds: 108732"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39196.5978090261,
            "unit": "iter/sec",
            "range": "stddev: 0.000002102635360354035",
            "extra": "mean: 25.512418319370624 usec\nrounds: 41246"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3751354.342506939,
            "unit": "iter/sec",
            "range": "stddev: 3.1838028108708386e-8",
            "extra": "mean: 266.5703926362907 nsec\nrounds: 193462"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3530609.3777775886,
            "unit": "iter/sec",
            "range": "stddev: 3.8089247763089885e-8",
            "extra": "mean: 283.23722422946065 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2249838.2495607357,
            "unit": "iter/sec",
            "range": "stddev: 5.3524782840722996e-8",
            "extra": "mean: 444.4763974455781 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 512330.9872287716,
            "unit": "iter/sec",
            "range": "stddev: 1.2829823807617186e-7",
            "extra": "mean: 1.9518631996261813 usec\nrounds: 52206"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1531259.7675129548,
            "unit": "iter/sec",
            "range": "stddev: 6.977370393830613e-8",
            "extra": "mean: 653.057058779868 nsec\nrounds: 155473"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1495381.5607377652,
            "unit": "iter/sec",
            "range": "stddev: 8.267199789484227e-8",
            "extra": "mean: 668.7256458522692 nsec\nrounds: 153093"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1336910.9096645638,
            "unit": "iter/sec",
            "range": "stddev: 7.89874301451282e-8",
            "extra": "mean: 747.9929984645453 nsec\nrounds: 136370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 443298.60015007254,
            "unit": "iter/sec",
            "range": "stddev: 1.4919291905633008e-7",
            "extra": "mean: 2.2558158308225007 usec\nrounds: 45064"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2724647.3096049307,
            "unit": "iter/sec",
            "range": "stddev: 4.3303782156495893e-8",
            "extra": "mean: 367.01997960443913 nsec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 681690.9719635675,
            "unit": "iter/sec",
            "range": "stddev: 1.483211175297679e-7",
            "extra": "mean: 1.4669403602625843 usec\nrounds: 70339"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2866716.062743869,
            "unit": "iter/sec",
            "range": "stddev: 4.074862000464218e-8",
            "extra": "mean: 348.83119852581257 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2745860.7461028905,
            "unit": "iter/sec",
            "range": "stddev: 4.548204038185346e-8",
            "extra": "mean: 364.1845280827819 nsec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1766827.6379578828,
            "unit": "iter/sec",
            "range": "stddev: 6.577510575808782e-8",
            "extra": "mean: 565.9861655525364 nsec\nrounds: 181489"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 496209.69983286277,
            "unit": "iter/sec",
            "range": "stddev: 1.558510861648829e-7",
            "extra": "mean: 2.0152770095724177 usec\nrounds: 50695"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 35385.16886618859,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023209933897104454",
            "extra": "mean: 28.260427519268525 usec\nrounds: 37065"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1229589.711550017,
            "unit": "iter/sec",
            "range": "stddev: 8.334249473385441e-8",
            "extra": "mean: 813.2794139431361 nsec\nrounds: 132206"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1218353.9540393879,
            "unit": "iter/sec",
            "range": "stddev: 8.673205128687075e-8",
            "extra": "mean: 820.7795416796079 nsec\nrounds: 130651"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 924395.290393736,
            "unit": "iter/sec",
            "range": "stddev: 1.7190469597363352e-7",
            "extra": "mean: 1.0817882894817303 usec\nrounds: 95521"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 359402.9598861683,
            "unit": "iter/sec",
            "range": "stddev: 4.1717412652338627e-7",
            "extra": "mean: 2.782392221579712 usec\nrounds: 189036"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34339.47268777817,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025343440187405086",
            "extra": "mean: 29.121006286037463 usec\nrounds: 44225"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2783416.5193879674,
            "unit": "iter/sec",
            "range": "stddev: 4.3414843853956874e-8",
            "extra": "mean: 359.2706995286974 nsec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2225165.0962483115,
            "unit": "iter/sec",
            "range": "stddev: 5.437199472791262e-8",
            "extra": "mean: 449.4048561547241 nsec\nrounds: 191242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 767522.8677253587,
            "unit": "iter/sec",
            "range": "stddev: 1.0111246794671456e-7",
            "extra": "mean: 1.3028927762942388 usec\nrounds: 79599"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 108042.42309534858,
            "unit": "iter/sec",
            "range": "stddev: 8.526464451247971e-7",
            "extra": "mean: 9.2556235907213 usec\nrounds: 109806"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 440708.7121617175,
            "unit": "iter/sec",
            "range": "stddev: 4.6638143073009573e-7",
            "extra": "mean: 2.2690724562601177 usec\nrounds: 45956"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 441925.8200180365,
            "unit": "iter/sec",
            "range": "stddev: 1.6731147518182823e-7",
            "extra": "mean: 2.2628232040372294 usec\nrounds: 45268"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 339604.6837459951,
            "unit": "iter/sec",
            "range": "stddev: 4.217025563697838e-7",
            "extra": "mean: 2.9446001420520536 usec\nrounds: 176679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 127603.76808865005,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010349839851189263",
            "extra": "mean: 7.836759172388003 usec\nrounds: 132381"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2510481.268956202,
            "unit": "iter/sec",
            "range": "stddev: 6.170382679164273e-7",
            "extra": "mean: 398.32999846112216 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2633658.1594772795,
            "unit": "iter/sec",
            "range": "stddev: 1.725206149289288e-7",
            "extra": "mean: 379.6999988026073 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1867657.7592393213,
            "unit": "iter/sec",
            "range": "stddev: 1.6184892484161355e-7",
            "extra": "mean: 535.4300031967796 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 431896.41345150775,
            "unit": "iter/sec",
            "range": "stddev: 9.930969419016728e-7",
            "extra": "mean: 2.3153700027478408 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 47884.37264567401,
            "unit": "iter/sec",
            "range": "stddev: 0.000003220717027983404",
            "extra": "mean: 20.883640000874948 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 733218.4598945635,
            "unit": "iter/sec",
            "range": "stddev: 0.000002170696007775675",
            "extra": "mean: 1.3638500047363777 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 756973.619273141,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021570762013142497",
            "extra": "mean: 1.3210500003424386 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 742644.111225946,
            "unit": "iter/sec",
            "range": "stddev: 5.959112963296121e-7",
            "extra": "mean: 1.3465399979395443 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 403455.1904142627,
            "unit": "iter/sec",
            "range": "stddev: 5.943126249371041e-7",
            "extra": "mean: 2.4785899989865356 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 45953.474868540165,
            "unit": "iter/sec",
            "range": "stddev: 0.0000035505010743505484",
            "extra": "mean: 21.761139997806822 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1613200.3033041519,
            "unit": "iter/sec",
            "range": "stddev: 7.415074606935211e-8",
            "extra": "mean: 619.8858244396703 nsec\nrounds: 166639"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 358008.12458505965,
            "unit": "iter/sec",
            "range": "stddev: 3.308251951336173e-7",
            "extra": "mean: 2.793232698724435 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 42697.8765711864,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015354013650590415",
            "extra": "mean: 23.4203684188554 usec\nrounds: 43665"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4325.205200775274,
            "unit": "iter/sec",
            "range": "stddev: 0.000005750452541588428",
            "extra": "mean: 231.20290334913 usec\nrounds: 4418"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 207008.89954181062,
            "unit": "iter/sec",
            "range": "stddev: 5.80546052849576e-7",
            "extra": "mean: 4.830710187887478 usec\nrounds: 107794"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 134161.05541119684,
            "unit": "iter/sec",
            "range": "stddev: 9.828692527441189e-7",
            "extra": "mean: 7.45372788649471 usec\nrounds: 139802"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31934.09370216712,
            "unit": "iter/sec",
            "range": "stddev: 0.000002054510573558438",
            "extra": "mean: 31.31449444992822 usec\nrounds: 32792"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3696.0997057614495,
            "unit": "iter/sec",
            "range": "stddev: 0.000005586516203647803",
            "extra": "mean: 270.55547187788477 usec\nrounds: 3787"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 140597.44331735693,
            "unit": "iter/sec",
            "range": "stddev: 8.768623160369536e-7",
            "extra": "mean: 7.11250486783602 usec\nrounds: 146371"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16844.98863670415,
            "unit": "iter/sec",
            "range": "stddev: 0.0000030318462503657338",
            "extra": "mean: 59.36483672188796 usec\nrounds: 17314"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1578.5133768741844,
            "unit": "iter/sec",
            "range": "stddev: 0.000010096731384842785",
            "extra": "mean: 633.5074600256017 usec\nrounds: 1626"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1265551.9630445943,
            "unit": "iter/sec",
            "range": "stddev: 1.0237805336481125e-7",
            "extra": "mean: 790.1690560332721 nsec\nrounds: 130481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104188.47275375236,
            "unit": "iter/sec",
            "range": "stddev: 8.73475912700718e-7",
            "extra": "mean: 9.597990771622909 usec\nrounds: 106304"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17122.841325844016,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025761316689822375",
            "extra": "mean: 58.40152232741129 usec\nrounds: 17445"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2350.9640250320717,
            "unit": "iter/sec",
            "range": "stddev: 0.0000066535774121962846",
            "extra": "mean: 425.3574233175933 usec\nrounds: 2393"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 717155.8389392658,
            "unit": "iter/sec",
            "range": "stddev: 1.201067687456623e-7",
            "extra": "mean: 1.3943970692326477 usec\nrounds: 74383"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 105123.78417751423,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010145423544328238",
            "extra": "mean: 9.512595154597735 usec\nrounds: 107562"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21147.36369289977,
            "unit": "iter/sec",
            "range": "stddev: 0.0000038842233384849464",
            "extra": "mean: 47.287218138483624 usec\nrounds: 23398"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4160.694938709015,
            "unit": "iter/sec",
            "range": "stddev: 0.0002443197955196762",
            "extra": "mean: 240.34446522298532 usec\nrounds: 4615"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2099300.6510569667,
            "unit": "iter/sec",
            "range": "stddev: 6.154885978007684e-8",
            "extra": "mean: 476.34911154647796 nsec\nrounds: 195734"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 982153.2392326042,
            "unit": "iter/sec",
            "range": "stddev: 1.0099113381666328e-7",
            "extra": "mean: 1.0181710552432164 usec\nrounds: 100412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 363932.546150135,
            "unit": "iter/sec",
            "range": "stddev: 3.6538451140726245e-7",
            "extra": "mean: 2.7477619426416036 usec\nrounds: 185909"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 119342.95841618207,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010841585641120423",
            "extra": "mean: 8.379212425024038 usec\nrounds: 123686"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 898044.7223015004,
            "unit": "iter/sec",
            "range": "stddev: 1.026080235368332e-7",
            "extra": "mean: 1.1135302899361326 usec\nrounds: 93985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 723228.8567787516,
            "unit": "iter/sec",
            "range": "stddev: 1.2446844947783065e-7",
            "extra": "mean: 1.3826881914723024 usec\nrounds: 75166"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 496704.7380724902,
            "unit": "iter/sec",
            "range": "stddev: 1.4473335976695917e-7",
            "extra": "mean: 2.0132684940365313 usec\nrounds: 51057"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 170641.8176567564,
            "unit": "iter/sec",
            "range": "stddev: 9.751666284564617e-7",
            "extra": "mean: 5.860228247283944 usec\nrounds: 180506"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5237373.736790639,
            "unit": "iter/sec",
            "range": "stddev: 1.2913167229991193e-8",
            "extra": "mean: 190.9353905709732 nsec\nrounds: 53985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 407091.3849912721,
            "unit": "iter/sec",
            "range": "stddev: 1.6999409003173612e-7",
            "extra": "mean: 2.4564509023482257 usec\nrounds: 41835"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 53752.52929662848,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013649030472822508",
            "extra": "mean: 18.603775730842177 usec\nrounds: 55206"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4043.8448842646176,
            "unit": "iter/sec",
            "range": "stddev: 0.0000072242807360022396",
            "extra": "mean: 247.2894061518515 usec\nrounds: 4129"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4249986.053596026,
            "unit": "iter/sec",
            "range": "stddev: 1.4016604745220269e-8",
            "extra": "mean: 235.2948897688926 nsec\nrounds: 43953"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 379967.9553009274,
            "unit": "iter/sec",
            "range": "stddev: 3.942967615374229e-7",
            "extra": "mean: 2.631800882282347 usec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2510914.5248157056,
            "unit": "iter/sec",
            "range": "stddev: 4.914872196955866e-8",
            "extra": "mean: 398.2612670072499 nsec\nrounds: 187301"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 46264.25108134502,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020755140272822814",
            "extra": "mean: 21.614961371399495 usec\nrounds: 48384"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 116117.94858319713,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010042020116487201",
            "extra": "mean: 8.611933057734928 usec\nrounds: 121433"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14052.049801802747,
            "unit": "iter/sec",
            "range": "stddev: 0.000003104913853882864",
            "extra": "mean: 71.16399486939687 usec\nrounds: 14423"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1327.1361018734501,
            "unit": "iter/sec",
            "range": "stddev: 0.000009132334910412838",
            "extra": "mean: 753.5022207506458 usec\nrounds: 1359"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 48176.429480708444,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019238815528515895",
            "extra": "mean: 20.75703846837457 usec\nrounds: 50925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 45348.03529244296,
            "unit": "iter/sec",
            "range": "stddev: 0.000002040047390004897",
            "extra": "mean: 22.051671997499863 usec\nrounds: 48335"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 33599.30111932962,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026172383096867596",
            "extra": "mean: 29.762523822994098 usec\nrounds: 35197"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 67247.13076065878,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032229146141133702",
            "extra": "mean: 14.870522930697653 usec\nrounds: 150083"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 247528.65745060839,
            "unit": "iter/sec",
            "range": "stddev: 4.4161445618900763e-7",
            "extra": "mean: 4.039936265559631 usec\nrounds: 128800"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 33639.302818097334,
            "unit": "iter/sec",
            "range": "stddev: 0.0000034197507025688656",
            "extra": "mean: 29.727132140860483 usec\nrounds: 48244"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32653.657530365515,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022859435505489254",
            "extra": "mean: 30.624440740522655 usec\nrounds: 33117"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8765.573639854394,
            "unit": "iter/sec",
            "range": "stddev: 0.00003821921216667742",
            "extra": "mean: 114.08266487583931 usec\nrounds: 12282"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3133.438781325176,
            "unit": "iter/sec",
            "range": "stddev: 0.000009007616413344384",
            "extra": "mean: 319.138196016418 usec\nrounds: 3163"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 17086.206351374054,
            "unit": "iter/sec",
            "range": "stddev: 0.000011209840208926277",
            "extra": "mean: 58.52674253343435 usec\nrounds: 23840"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 27811.749234163144,
            "unit": "iter/sec",
            "range": "stddev: 0.000001927960930356828",
            "extra": "mean: 35.95602677057181 usec\nrounds: 28576"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 30850.2705114062,
            "unit": "iter/sec",
            "range": "stddev: 0.000007339640243714474",
            "extra": "mean: 32.414626627999006 usec\nrounds: 45681"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24568.685508587343,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019869176429925446",
            "extra": "mean: 40.702218262775034 usec\nrounds: 25987"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 27574.982947247427,
            "unit": "iter/sec",
            "range": "stddev: 0.00000610769876399915",
            "extra": "mean: 36.26475497421192 usec\nrounds: 41212"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23018.350710280494,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020925008289413627",
            "extra": "mean: 43.443599091284085 usec\nrounds: 23327"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16014.952067375021,
            "unit": "iter/sec",
            "range": "stddev: 0.000010041647475772026",
            "extra": "mean: 62.44164801699016 usec\nrounds: 20930"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21002.81703367873,
            "unit": "iter/sec",
            "range": "stddev: 0.000002315378868975403",
            "extra": "mean: 47.61266064435385 usec\nrounds: 21420"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 35474.6087288635,
            "unit": "iter/sec",
            "range": "stddev: 0.00000431536326136772",
            "extra": "mean: 28.18917631038906 usec\nrounds: 56168"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47506.94759966659,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015929655044365388",
            "extra": "mean: 21.049552760721213 usec\nrounds: 48502"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4501.845453696572,
            "unit": "iter/sec",
            "range": "stddev: 0.00006356204460989802",
            "extra": "mean: 222.13112606495105 usec\nrounds: 6457"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 903.9604583262047,
            "unit": "iter/sec",
            "range": "stddev: 0.00007979484370657589",
            "extra": "mean: 1.1062430782112134 msec\nrounds: 895"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1033.9738578045801,
            "unit": "iter/sec",
            "range": "stddev: 0.00016291084392602007",
            "extra": "mean: 967.1424402579033 usec\nrounds: 1247"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 275.2088933218372,
            "unit": "iter/sec",
            "range": "stddev: 0.000020360606922413363",
            "extra": "mean: 3.6336035072477517 msec\nrounds: 276"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 59748.03468218637,
            "unit": "iter/sec",
            "range": "stddev: 0.0000030845766693167904",
            "extra": "mean: 16.73695219130188 usec\nrounds: 95694"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25579.3229865266,
            "unit": "iter/sec",
            "range": "stddev: 0.000005087102406541649",
            "extra": "mean: 39.09407612260614 usec\nrounds: 36323"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 6110.714114210199,
            "unit": "iter/sec",
            "range": "stddev: 0.00004373734509834914",
            "extra": "mean: 163.64699465722734 usec\nrounds: 8423"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1723.1058175325459,
            "unit": "iter/sec",
            "range": "stddev: 0.00009334203745193672",
            "extra": "mean: 580.347410951221 usec\nrounds: 1954"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 30455.84775297584,
            "unit": "iter/sec",
            "range": "stddev: 0.000008765778952420466",
            "extra": "mean: 32.83441682894183 usec\nrounds: 46729"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1708.0905972941623,
            "unit": "iter/sec",
            "range": "stddev: 0.00009965537591046775",
            "extra": "mean: 585.4490397547589 usec\nrounds: 1962"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1150.9265878071017,
            "unit": "iter/sec",
            "range": "stddev: 0.00014521303258371765",
            "extra": "mean: 868.8651479546866 usec\nrounds: 1345"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "committer": {
            "name": "Cache McClure",
            "username": "cachemcclure",
            "email": "cache.mcclure@gmail.com"
          },
          "id": "1eaa10ed575a91cea766992342cdde9452ea105f",
          "message": "Adding xfail markers to troublesome tests (for now)",
          "timestamp": "2025-10-24T14:08:32Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/1eaa10ed575a91cea766992342cdde9452ea105f"
        },
        "date": 1767576845521,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5826966.322028341,
            "unit": "iter/sec",
            "range": "stddev: 1.3003629731432681e-8",
            "extra": "mean: 171.61588805131584 nsec\nrounds: 59517"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5383675.55700376,
            "unit": "iter/sec",
            "range": "stddev: 9.088060706190166e-9",
            "extra": "mean: 185.74670583539847 nsec\nrounds: 54991"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3113059.2824132037,
            "unit": "iter/sec",
            "range": "stddev: 2.980927572576634e-8",
            "extra": "mean: 321.22741948711393 nsec\nrounds: 196735"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 584780.5148782835,
            "unit": "iter/sec",
            "range": "stddev: 7.706700918543979e-8",
            "extra": "mean: 1.7100432975406918 usec\nrounds: 58994"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 65599.2612547841,
            "unit": "iter/sec",
            "range": "stddev: 6.867462766901569e-7",
            "extra": "mean: 15.244074108030766 usec\nrounds: 66335"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 450758.5319407178,
            "unit": "iter/sec",
            "range": "stddev: 1.0378083285344125e-7",
            "extra": "mean: 2.2184826889344746 usec\nrounds: 46560"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 441862.73170933116,
            "unit": "iter/sec",
            "range": "stddev: 1.1032993280633664e-7",
            "extra": "mean: 2.2631462855704783 usec\nrounds: 45997"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 394760.7235296676,
            "unit": "iter/sec",
            "range": "stddev: 1.647880518024715e-7",
            "extra": "mean: 2.533180076930438 usec\nrounds: 41038"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 208075.7444108498,
            "unit": "iter/sec",
            "range": "stddev: 2.9919292834638295e-7",
            "extra": "mean: 4.805942195864404 usec\nrounds: 108366"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 36820.95247386436,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013968382802215513",
            "extra": "mean: 27.158450089247516 usec\nrounds: 37486"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3786074.4482712187,
            "unit": "iter/sec",
            "range": "stddev: 2.1154856810179755e-8",
            "extra": "mean: 264.1258151847996 nsec\nrounds: 193574"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3507402.383486044,
            "unit": "iter/sec",
            "range": "stddev: 2.304812059662428e-8",
            "extra": "mean: 285.11128483811734 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2421017.285851719,
            "unit": "iter/sec",
            "range": "stddev: 3.438461061409192e-8",
            "extra": "mean: 413.0495085036929 nsec\nrounds: 189251"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 301099.59488969465,
            "unit": "iter/sec",
            "range": "stddev: 2.442922978679802e-7",
            "extra": "mean: 3.3211602306085526 usec\nrounds: 152649"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1492231.0827365213,
            "unit": "iter/sec",
            "range": "stddev: 4.822441199364262e-8",
            "extra": "mean: 670.1374951700756 nsec\nrounds: 157159"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1445778.4196632097,
            "unit": "iter/sec",
            "range": "stddev: 5.52089635412068e-8",
            "extra": "mean: 691.6689213226504 nsec\nrounds: 151654"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1209496.8124295685,
            "unit": "iter/sec",
            "range": "stddev: 6.39554295729747e-8",
            "extra": "mean: 826.7901078558986 nsec\nrounds: 129618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 300728.1493237956,
            "unit": "iter/sec",
            "range": "stddev: 2.6826127782841947e-7",
            "extra": "mean: 3.325262374834405 usec\nrounds: 153658"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2853732.100464468,
            "unit": "iter/sec",
            "range": "stddev: 2.8281007511319624e-8",
            "extra": "mean: 350.41831706531883 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 685200.169179083,
            "unit": "iter/sec",
            "range": "stddev: 2.8677279324610276e-7",
            "extra": "mean: 1.459427543922046 usec\nrounds: 71809"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2758778.3996213134,
            "unit": "iter/sec",
            "range": "stddev: 3.0776046996986325e-8",
            "extra": "mean: 362.4792771094917 nsec\nrounds: 199045"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2637388.867679803,
            "unit": "iter/sec",
            "range": "stddev: 4.676712844321124e-8",
            "extra": "mean: 379.1628956407027 nsec\nrounds: 194515"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1729575.2587760983,
            "unit": "iter/sec",
            "range": "stddev: 4.628723289086769e-8",
            "extra": "mean: 578.1766332085643 nsec\nrounds: 179148"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 382383.1707957021,
            "unit": "iter/sec",
            "range": "stddev: 2.2387999992084698e-7",
            "extra": "mean: 2.6151778539811192 usec\nrounds: 197629"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 23058.951137046326,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025751524775065454",
            "extra": "mean: 43.367106945007926 usec\nrounds: 23788"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1367506.609411298,
            "unit": "iter/sec",
            "range": "stddev: 6.358869364379288e-8",
            "extra": "mean: 731.2578916386276 nsec\nrounds: 139412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1351717.4311261897,
            "unit": "iter/sec",
            "range": "stddev: 6.438290142708868e-8",
            "extra": "mean: 739.799589006451 nsec\nrounds: 137742"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 994804.5484042974,
            "unit": "iter/sec",
            "range": "stddev: 8.779252574681227e-8",
            "extra": "mean: 1.005222585284761 usec\nrounds: 103864"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 346150.8106641922,
            "unit": "iter/sec",
            "range": "stddev: 3.1896345808229305e-7",
            "extra": "mean: 2.8889142223333404 usec\nrounds: 178444"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 21788.168287281056,
            "unit": "iter/sec",
            "range": "stddev: 0.000002179004921596576",
            "extra": "mean: 45.89646944225939 usec\nrounds: 22220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2643718.2521555317,
            "unit": "iter/sec",
            "range": "stddev: 3.224661451778011e-8",
            "extra": "mean: 378.25513334662054 nsec\nrounds: 192753"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2236833.841006958,
            "unit": "iter/sec",
            "range": "stddev: 4.150566874299744e-8",
            "extra": "mean: 447.0604752428783 nsec\nrounds: 191205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 875803.7279881196,
            "unit": "iter/sec",
            "range": "stddev: 7.111886293246234e-8",
            "extra": "mean: 1.141808339063783 usec\nrounds: 89135"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 111952.60776960042,
            "unit": "iter/sec",
            "range": "stddev: 5.422891630396128e-7",
            "extra": "mean: 8.932351107515155 usec\nrounds: 113418"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 475284.89206447866,
            "unit": "iter/sec",
            "range": "stddev: 1.4481736723467003e-7",
            "extra": "mean: 2.10400123525141 usec\nrounds: 49383"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 458009.909782788,
            "unit": "iter/sec",
            "range": "stddev: 1.2966902145258767e-7",
            "extra": "mean: 2.1833588720258152 usec\nrounds: 47255"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 318653.07485774386,
            "unit": "iter/sec",
            "range": "stddev: 2.6827315072684347e-7",
            "extra": "mean: 3.1382091650815847 usec\nrounds: 165838"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 93985.44301528214,
            "unit": "iter/sec",
            "range": "stddev: 6.356662320333912e-7",
            "extra": "mean: 10.639945590695348 usec\nrounds: 101103"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2636782.933837365,
            "unit": "iter/sec",
            "range": "stddev: 5.936143200312259e-7",
            "extra": "mean: 379.2500274357735 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2819522.0766397747,
            "unit": "iter/sec",
            "range": "stddev: 2.0988635527989404e-7",
            "extra": "mean: 354.6700372680789 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2028438.644777115,
            "unit": "iter/sec",
            "range": "stddev: 1.7250768585196243e-7",
            "extra": "mean: 492.99001602776116 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 540371.1277211155,
            "unit": "iter/sec",
            "range": "stddev: 1.7974230625882208e-7",
            "extra": "mean: 1.8505799971535453 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 58447.0957666438,
            "unit": "iter/sec",
            "range": "stddev: 0.000001981472128219166",
            "extra": "mean: 17.109489990616567 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 852783.9131364966,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018258804211667662",
            "extra": "mean: 1.1726299999281764 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 921124.1387138467,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014050711196293415",
            "extra": "mean: 1.0856300013983855 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 823079.1280711577,
            "unit": "iter/sec",
            "range": "stddev: 5.273515815202397e-7",
            "extra": "mean: 1.2149500162195181 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 366637.45967270265,
            "unit": "iter/sec",
            "range": "stddev: 7.12264038405064e-7",
            "extra": "mean: 2.7274899866824853 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 26189.034549017182,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017568626613122905",
            "extra": "mean: 38.183919996299664 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1490257.1442906887,
            "unit": "iter/sec",
            "range": "stddev: 5.420646345471054e-8",
            "extra": "mean: 671.025134038841 nsec\nrounds: 153823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 418092.4860177261,
            "unit": "iter/sec",
            "range": "stddev: 1.0930308696253524e-7",
            "extra": "mean: 2.3918152883465384 usec\nrounds: 42752"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 51681.45244484558,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010121565848175523",
            "extra": "mean: 19.349301397192725 usec\nrounds: 53076"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 5255.988903712374,
            "unit": "iter/sec",
            "range": "stddev: 0.000004239942256825288",
            "extra": "mean: 190.25915357121224 usec\nrounds: 5398"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 231146.96566251235,
            "unit": "iter/sec",
            "range": "stddev: 3.2491238648910805e-7",
            "extra": "mean: 4.326251902696645 usec\nrounds: 120352"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 147840.00394398085,
            "unit": "iter/sec",
            "range": "stddev: 5.692565433520014e-7",
            "extra": "mean: 6.764069083621761 usec\nrounds: 153351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 34439.02408528975,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010758587946757211",
            "extra": "mean: 29.036827452585655 usec\nrounds: 35341"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 4075.7285663031457,
            "unit": "iter/sec",
            "range": "stddev: 0.0000037859618736592266",
            "extra": "mean: 245.35490617988853 usec\nrounds: 4189"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 139406.94294914772,
            "unit": "iter/sec",
            "range": "stddev: 5.52922563457283e-7",
            "extra": "mean: 7.173243877564805 usec\nrounds: 145688"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17141.9065532268,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019419521501465444",
            "extra": "mean: 58.33656815797771 usec\nrounds: 17657"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1695.2539828094375,
            "unit": "iter/sec",
            "range": "stddev: 0.000006924549063472617",
            "extra": "mean: 589.882112143906 usec\nrounds: 1721"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1158209.5892776924,
            "unit": "iter/sec",
            "range": "stddev: 5.4090684318832614e-8",
            "extra": "mean: 863.4015891921896 nsec\nrounds: 118554"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 110964.48659221658,
            "unit": "iter/sec",
            "range": "stddev: 5.764132571411542e-7",
            "extra": "mean: 9.011892279328071 usec\nrounds: 115142"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 18762.373117924933,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014137163941890479",
            "extra": "mean: 53.29816189640926 usec\nrounds: 18975"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2570.00280792575,
            "unit": "iter/sec",
            "range": "stddev: 0.000017149477624378956",
            "extra": "mean: 389.1046332385529 usec\nrounds: 2612"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 825833.5446419421,
            "unit": "iter/sec",
            "range": "stddev: 7.510598267979203e-8",
            "extra": "mean: 1.2108977729084072 usec\nrounds: 84682"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 169464.40186151967,
            "unit": "iter/sec",
            "range": "stddev: 6.198317807176547e-7",
            "extra": "mean: 5.900944322319473 usec\nrounds: 190042"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 29248.740066879276,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025488779348923328",
            "extra": "mean: 34.18950688861915 usec\nrounds: 32366"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 6228.295815233123,
            "unit": "iter/sec",
            "range": "stddev: 0.00005571803828957593",
            "extra": "mean: 160.55756336335327 usec\nrounds: 7189"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 1872184.5598737507,
            "unit": "iter/sec",
            "range": "stddev: 4.283481169229711e-8",
            "extra": "mean: 534.1353739544935 nsec\nrounds: 192865"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 943805.6296767711,
            "unit": "iter/sec",
            "range": "stddev: 6.437720713654939e-8",
            "extra": "mean: 1.0595401940360236 usec\nrounds: 96442"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 327961.3991787428,
            "unit": "iter/sec",
            "range": "stddev: 2.386277938590338e-7",
            "extra": "mean: 3.0491393270797342 usec\nrounds: 170591"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 100166.93826568774,
            "unit": "iter/sec",
            "range": "stddev: 6.52415605412252e-7",
            "extra": "mean: 9.983333995370314 usec\nrounds: 101792"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 945355.6517422564,
            "unit": "iter/sec",
            "range": "stddev: 6.181892144242589e-8",
            "extra": "mean: 1.0578029529490107 usec\nrounds: 96890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 779793.9633341039,
            "unit": "iter/sec",
            "range": "stddev: 7.710854957074031e-8",
            "extra": "mean: 1.2823900248270252 usec\nrounds: 79898"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 420924.9435727805,
            "unit": "iter/sec",
            "range": "stddev: 1.0557623766274173e-7",
            "extra": "mean: 2.375720458645363 usec\nrounds: 43214"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 202570.69099775178,
            "unit": "iter/sec",
            "range": "stddev: 3.008989606982443e-7",
            "extra": "mean: 4.936548298643551 usec\nrounds: 103285"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 6066031.423527453,
            "unit": "iter/sec",
            "range": "stddev: 8.89153919594375e-9",
            "extra": "mean: 164.85242659994054 nsec\nrounds: 62532"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 455529.2956013615,
            "unit": "iter/sec",
            "range": "stddev: 1.0333347058994357e-7",
            "extra": "mean: 2.1952484936887946 usec\nrounds: 47070"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 59700.217147198084,
            "unit": "iter/sec",
            "range": "stddev: 7.917524219444842e-7",
            "extra": "mean: 16.750357834283573 usec\nrounds: 61668"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4514.720214607987,
            "unit": "iter/sec",
            "range": "stddev: 0.000003821347985850301",
            "extra": "mean: 221.49766817539765 usec\nrounds: 4641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4653469.772285936,
            "unit": "iter/sec",
            "range": "stddev: 1.246032870609962e-8",
            "extra": "mean: 214.89341264352393 nsec\nrounds: 47333"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 437760.95760447363,
            "unit": "iter/sec",
            "range": "stddev: 1.095571744950892e-7",
            "extra": "mean: 2.284351728103426 usec\nrounds: 45161"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2288551.9095422784,
            "unit": "iter/sec",
            "range": "stddev: 3.469753630924616e-8",
            "extra": "mean: 436.9575345134948 nsec\nrounds: 194138"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 53254.59164061153,
            "unit": "iter/sec",
            "range": "stddev: 0.000001372708238949349",
            "extra": "mean: 18.77772355759476 usec\nrounds: 55006"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 116564.65787254342,
            "unit": "iter/sec",
            "range": "stddev: 7.036184527097741e-7",
            "extra": "mean: 8.57892965373296 usec\nrounds: 122011"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14232.659340816159,
            "unit": "iter/sec",
            "range": "stddev: 0.0000036147234941011486",
            "extra": "mean: 70.2609383147546 usec\nrounds: 14525"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1372.0425019414135,
            "unit": "iter/sec",
            "range": "stddev: 0.000006837146000454131",
            "extra": "mean: 728.8403956765329 usec\nrounds: 1390"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 55877.017341620354,
            "unit": "iter/sec",
            "range": "stddev: 0.000001533370962952105",
            "extra": "mean: 17.89644557951635 usec\nrounds: 58167"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 53745.012111685865,
            "unit": "iter/sec",
            "range": "stddev: 9.989170808930285e-7",
            "extra": "mean: 18.60637779598841 usec\nrounds: 55522"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 38364.52809452382,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016011330698693857",
            "extra": "mean: 26.06574483429501 usec\nrounds: 39578"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 74752.12088508789,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029171663611063603",
            "extra": "mean: 13.377546859670272 usec\nrounds: 146456"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 255409.55781224582,
            "unit": "iter/sec",
            "range": "stddev: 3.5041135031846385e-7",
            "extra": "mean: 3.915280260322561 usec\nrounds: 132468"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 26149.30303968855,
            "unit": "iter/sec",
            "range": "stddev: 0.000013139215250270372",
            "extra": "mean: 38.241937021504285 usec\nrounds: 49128"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 33602.972065928574,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011439019975875888",
            "extra": "mean: 29.759272425010906 usec\nrounds: 34391"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 7890.69370268002,
            "unit": "iter/sec",
            "range": "stddev: 0.00004307913538225238",
            "extra": "mean: 126.73156983147844 usec\nrounds: 11528"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3213.4195610071065,
            "unit": "iter/sec",
            "range": "stddev: 0.00000557929447340685",
            "extra": "mean: 311.19496879100143 usec\nrounds: 3268"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 13838.215188342256,
            "unit": "iter/sec",
            "range": "stddev: 0.000018801478887795726",
            "extra": "mean: 72.26365440844071 usec\nrounds: 22460"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 29140.881285618732,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014196949219017674",
            "extra": "mean: 34.31605208499676 usec\nrounds: 29759"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 26046.812218779036,
            "unit": "iter/sec",
            "range": "stddev: 0.00001227677114263337",
            "extra": "mean: 38.39241407357432 usec\nrounds: 47692"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 25216.31818520892,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014273392969061845",
            "extra": "mean: 39.65686000054392 usec\nrounds: 26536"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 21914.449184822883,
            "unit": "iter/sec",
            "range": "stddev: 0.000015682223244827392",
            "extra": "mean: 45.631993374150696 usec\nrounds: 38483"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22780.51630742224,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018646655300649317",
            "extra": "mean: 43.89716135073658 usec\nrounds: 23198"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 12883.521618012,
            "unit": "iter/sec",
            "range": "stddev: 0.000020736439049585705",
            "extra": "mean: 77.61852928487619 usec\nrounds: 20165"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 20715.352816407125,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017859778755475274",
            "extra": "mean: 48.27337525277257 usec\nrounds: 21095"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 30258.799007924405,
            "unit": "iter/sec",
            "range": "stddev: 0.000010201758271299953",
            "extra": "mean: 33.04823829055847 usec\nrounds: 56020"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 50642.73293101996,
            "unit": "iter/sec",
            "range": "stddev: 0.000001796253995494266",
            "extra": "mean: 19.746169729072317 usec\nrounds: 52242"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4654.884440954963,
            "unit": "iter/sec",
            "range": "stddev: 0.000050227970911253934",
            "extra": "mean: 214.82810426005915 usec\nrounds: 6177"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 990.8153234320049,
            "unit": "iter/sec",
            "range": "stddev: 0.000010236264391669645",
            "extra": "mean: 1.0092698168374918 msec\nrounds: 999"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1033.775800084222,
            "unit": "iter/sec",
            "range": "stddev: 0.00010439607990368838",
            "extra": "mean: 967.3277319110484 usec\nrounds: 1175"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 277.3973564738899,
            "unit": "iter/sec",
            "range": "stddev: 0.000020771600290926787",
            "extra": "mean: 3.6049370214316565 msec\nrounds: 280"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 62469.83277629557,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018493432912113623",
            "extra": "mean: 16.007726538679865 usec\nrounds: 105463"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 19977.466383015057,
            "unit": "iter/sec",
            "range": "stddev: 0.00001753438646015914",
            "extra": "mean: 50.056397584540804 usec\nrounds: 33809"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 6099.416549784369,
            "unit": "iter/sec",
            "range": "stddev: 0.000035772788046517936",
            "extra": "mean: 163.95010765994542 usec\nrounds: 7691"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1638.5266938582256,
            "unit": "iter/sec",
            "range": "stddev: 0.00005275745287055053",
            "extra": "mean: 610.3043690092763 usec\nrounds: 1794"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 26930.630842317023,
            "unit": "iter/sec",
            "range": "stddev: 0.000010272933216185818",
            "extra": "mean: 37.13243874067242 usec\nrounds: 48286"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1713.684431437509,
            "unit": "iter/sec",
            "range": "stddev: 0.0000701158270292467",
            "extra": "mean: 583.5380083141439 usec\nrounds: 1925"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1141.3884358111168,
            "unit": "iter/sec",
            "range": "stddev: 0.0001100539945729438",
            "extra": "mean: 876.1259257803497 usec\nrounds: 1307"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Joe Howarth",
            "username": "JoeHowarth",
            "email": "josephehowarth@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f9dfb0553351970c1eee00f13e9120c571a99926",
          "message": "Add Rust flags for macOS target configurations (#2)\n\nAdded target-specific Rust flags to prevent link errors on macOS.",
          "timestamp": "2026-01-05T04:54:01Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/f9dfb0553351970c1eee00f13e9120c571a99926"
        },
        "date": 1768181508371,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5450675.404200831,
            "unit": "iter/sec",
            "range": "stddev: 1.220275735397263e-8",
            "extra": "mean: 183.46350238165476 nsec\nrounds: 54994"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5108107.025414442,
            "unit": "iter/sec",
            "range": "stddev: 1.20274204371188e-8",
            "extra": "mean: 195.76723726122177 nsec\nrounds: 51905"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3249457.088348978,
            "unit": "iter/sec",
            "range": "stddev: 3.798209735543613e-8",
            "extra": "mean: 307.7437161996229 nsec\nrounds: 193462"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 695265.5596259526,
            "unit": "iter/sec",
            "range": "stddev: 1.06192468477678e-7",
            "extra": "mean: 1.4382993464224034 usec\nrounds: 70687"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 75492.45031363619,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012471554668860019",
            "extra": "mean: 13.246357693325132 usec\nrounds: 77737"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 406291.4625327594,
            "unit": "iter/sec",
            "range": "stddev: 1.584045804457648e-7",
            "extra": "mean: 2.4612872585757812 usec\nrounds: 41887"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 400560.27614112245,
            "unit": "iter/sec",
            "range": "stddev: 1.687766110453127e-7",
            "extra": "mean: 2.496503172091098 usec\nrounds: 41676"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 351914.29271804093,
            "unit": "iter/sec",
            "range": "stddev: 3.7049973335976954e-7",
            "extra": "mean: 2.8416009826608977 usec\nrounds: 184502"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 205518.65915496534,
            "unit": "iter/sec",
            "range": "stddev: 5.410715386791967e-7",
            "extra": "mean: 4.865738245430938 usec\nrounds: 107320"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39097.70262851156,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018269264715213058",
            "extra": "mean: 25.576950377405584 usec\nrounds: 40808"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3731131.9635775527,
            "unit": "iter/sec",
            "range": "stddev: 3.0729729229197806e-8",
            "extra": "mean: 268.01517870761256 nsec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3414761.5698077106,
            "unit": "iter/sec",
            "range": "stddev: 3.9430867797275696e-8",
            "extra": "mean: 292.8462147521591 nsec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2243635.561177175,
            "unit": "iter/sec",
            "range": "stddev: 5.195514502225833e-8",
            "extra": "mean: 445.7051837221357 nsec\nrounds: 190513"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 494309.03968150844,
            "unit": "iter/sec",
            "range": "stddev: 1.274064015673102e-7",
            "extra": "mean: 2.023025920473378 usec\nrounds: 50925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1499141.6973058034,
            "unit": "iter/sec",
            "range": "stddev: 8.650031077701798e-8",
            "extra": "mean: 667.0483529322707 nsec\nrounds: 154274"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1477685.3043581522,
            "unit": "iter/sec",
            "range": "stddev: 7.534226109279061e-8",
            "extra": "mean: 676.7340766336739 nsec\nrounds: 151218"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1273248.596349442,
            "unit": "iter/sec",
            "range": "stddev: 8.124861081088631e-8",
            "extra": "mean: 785.3925799463954 nsec\nrounds: 129972"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 438069.75581558666,
            "unit": "iter/sec",
            "range": "stddev: 1.550293847374161e-7",
            "extra": "mean: 2.282741473759639 usec\nrounds: 44580"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2800323.4767261185,
            "unit": "iter/sec",
            "range": "stddev: 4.271619585190105e-8",
            "extra": "mean: 357.10160212257335 nsec\nrounds: 189790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 672738.0714152282,
            "unit": "iter/sec",
            "range": "stddev: 1.1944992812982241e-7",
            "extra": "mean: 1.486462625634228 usec\nrounds: 68555"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3067094.848213442,
            "unit": "iter/sec",
            "range": "stddev: 4.0186699426712376e-8",
            "extra": "mean: 326.04143317657486 nsec\nrounds: 196117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2898170.626244974,
            "unit": "iter/sec",
            "range": "stddev: 4.181968243374072e-8",
            "extra": "mean: 345.0452471446514 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1769594.150454452,
            "unit": "iter/sec",
            "range": "stddev: 6.96064818098754e-8",
            "extra": "mean: 565.101325489348 nsec\nrounds: 181819"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 496229.8612610861,
            "unit": "iter/sec",
            "range": "stddev: 1.483554986504635e-7",
            "extra": "mean: 2.0151951304556204 usec\nrounds: 50744"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47528.855673694954,
            "unit": "iter/sec",
            "range": "stddev: 0.00000193954064773",
            "extra": "mean: 21.039850125267254 usec\nrounds: 49121"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1208179.2796125638,
            "unit": "iter/sec",
            "range": "stddev: 8.326398810721197e-8",
            "extra": "mean: 827.6917315786721 nsec\nrounds: 125550"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1194002.4925867182,
            "unit": "iter/sec",
            "range": "stddev: 8.384126664889977e-8",
            "extra": "mean: 837.5191896237803 nsec\nrounds: 123671"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 893311.5414416977,
            "unit": "iter/sec",
            "range": "stddev: 1.0599842389564456e-7",
            "extra": "mean: 1.1194302923547936 usec\nrounds: 92251"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 360171.3436012693,
            "unit": "iter/sec",
            "range": "stddev: 7.232047880705686e-7",
            "extra": "mean: 2.7764563110469402 usec\nrounds: 189430"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 44513.81567881932,
            "unit": "iter/sec",
            "range": "stddev: 0.000001820338608475358",
            "extra": "mean: 22.464935543052594 usec\nrounds: 45767"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2860474.0545483083,
            "unit": "iter/sec",
            "range": "stddev: 4.6048019722890124e-8",
            "extra": "mean: 349.5924035423281 nsec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2319915.8239045246,
            "unit": "iter/sec",
            "range": "stddev: 5.280283842663788e-8",
            "extra": "mean: 431.05012246391203 nsec\nrounds: 198020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 785562.8923450665,
            "unit": "iter/sec",
            "range": "stddev: 9.621995539519372e-8",
            "extra": "mean: 1.2729725522227715 usec\nrounds: 79981"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107301.36114891333,
            "unit": "iter/sec",
            "range": "stddev: 8.103119986109287e-7",
            "extra": "mean: 9.319546269428914 usec\nrounds: 109446"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 442559.5958883832,
            "unit": "iter/sec",
            "range": "stddev: 1.5336874772458257e-7",
            "extra": "mean: 2.2595826851130094 usec\nrounds: 45227"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 438646.81942223746,
            "unit": "iter/sec",
            "range": "stddev: 1.617045853761388e-7",
            "extra": "mean: 2.2797384039331554 usec\nrounds: 44860"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 330147.2894115783,
            "unit": "iter/sec",
            "range": "stddev: 4.1237665206724433e-7",
            "extra": "mean: 3.0289511138567895 usec\nrounds: 171204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 124763.98631256842,
            "unit": "iter/sec",
            "range": "stddev: 9.671120458541866e-7",
            "extra": "mean: 8.01513344960558 usec\nrounds: 131338"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2451761.5503612086,
            "unit": "iter/sec",
            "range": "stddev: 5.616812356903e-7",
            "extra": "mean: 407.870006711164 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2551085.4859685465,
            "unit": "iter/sec",
            "range": "stddev: 2.017961992190934e-7",
            "extra": "mean: 391.9900001392307 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1935958.4843965652,
            "unit": "iter/sec",
            "range": "stddev: 1.6159860890586235e-7",
            "extra": "mean: 516.5400023088296 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 447828.03434479167,
            "unit": "iter/sec",
            "range": "stddev: 3.451425164808182e-7",
            "extra": "mean: 2.232999998454943 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 50294.80299034858,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018366139977255254",
            "extra": "mean: 19.882769998957883 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 881391.1869859674,
            "unit": "iter/sec",
            "range": "stddev: 0.00000126734883668342",
            "extra": "mean: 1.1345700011133886 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 945751.6837391775,
            "unit": "iter/sec",
            "range": "stddev: 4.057527631633559e-7",
            "extra": "mean: 1.0573599996632765 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 796736.5651475659,
            "unit": "iter/sec",
            "range": "stddev: 4.489421488329252e-7",
            "extra": "mean: 1.255120002952026 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 387452.7313914501,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010697092737864398",
            "extra": "mean: 2.5809599958392937 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 54092.06469130164,
            "unit": "iter/sec",
            "range": "stddev: 0.000002193034633916231",
            "extra": "mean: 18.487000000959597 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1638578.302362129,
            "unit": "iter/sec",
            "range": "stddev: 7.275377702974473e-8",
            "extra": "mean: 610.2851469218713 nsec\nrounds: 168322"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 364595.8331266804,
            "unit": "iter/sec",
            "range": "stddev: 3.252617811967957e-7",
            "extra": "mean: 2.742763106819561 usec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 44149.30607874323,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013602011926015363",
            "extra": "mean: 22.65041262973496 usec\nrounds: 45227"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4481.130734892821,
            "unit": "iter/sec",
            "range": "stddev: 0.000005245302473932915",
            "extra": "mean: 223.15796149694745 usec\nrounds: 4597"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 208627.73241853257,
            "unit": "iter/sec",
            "range": "stddev: 5.453376860407886e-7",
            "extra": "mean: 4.793226616650746 usec\nrounds: 108496"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 134418.4700072326,
            "unit": "iter/sec",
            "range": "stddev: 9.546566580044593e-7",
            "extra": "mean: 7.439453818706562 usec\nrounds: 140371"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31738.555949277223,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018249768780843178",
            "extra": "mean: 31.507419606554997 usec\nrounds: 32683"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3712.439941827084,
            "unit": "iter/sec",
            "range": "stddev: 0.00000567277518907886",
            "extra": "mean: 269.36462694877923 usec\nrounds: 3785"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 136687.47420415835,
            "unit": "iter/sec",
            "range": "stddev: 8.493255434476037e-7",
            "extra": "mean: 7.315959313919181 usec\nrounds: 144865"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16908.39765842871,
            "unit": "iter/sec",
            "range": "stddev: 0.000002850989490659956",
            "extra": "mean: 59.14220969965819 usec\nrounds: 17444"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1610.8321846996,
            "unit": "iter/sec",
            "range": "stddev: 0.000010850555184034312",
            "extra": "mean: 620.7971317549055 usec\nrounds: 1647"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1279706.9204446487,
            "unit": "iter/sec",
            "range": "stddev: 8.22710179060869e-8",
            "extra": "mean: 781.428922532162 nsec\nrounds: 131510"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 105282.57497848464,
            "unit": "iter/sec",
            "range": "stddev: 8.677484844901733e-7",
            "extra": "mean: 9.49824793138236 usec\nrounds: 107562"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17143.78642042329,
            "unit": "iter/sec",
            "range": "stddev: 0.000002430276195962988",
            "extra": "mean: 58.33017137968459 usec\nrounds: 17435"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2367.2959563140034,
            "unit": "iter/sec",
            "range": "stddev: 0.000005608696702399647",
            "extra": "mean: 422.4228902739518 usec\nrounds: 2406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 714601.3849003721,
            "unit": "iter/sec",
            "range": "stddev: 1.1395399438850238e-7",
            "extra": "mean: 1.3993815589084053 usec\nrounds: 73282"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 104431.6508443489,
            "unit": "iter/sec",
            "range": "stddev: 9.298415725052093e-7",
            "extra": "mean: 9.575641023720472 usec\nrounds: 107216"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21423.680171858858,
            "unit": "iter/sec",
            "range": "stddev: 0.000003038724684035476",
            "extra": "mean: 46.67732116882296 usec\nrounds: 22552"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4305.954222297555,
            "unit": "iter/sec",
            "range": "stddev: 0.000017835015441709053",
            "extra": "mean: 232.23656090482626 usec\nrounds: 4507"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2196284.0767946015,
            "unit": "iter/sec",
            "range": "stddev: 5.441175287065629e-8",
            "extra": "mean: 455.31450624523677 nsec\nrounds: 186916"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 994901.0418101411,
            "unit": "iter/sec",
            "range": "stddev: 9.09854252201573e-8",
            "extra": "mean: 1.0051250908136553 usec\nrounds: 101854"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 366150.7535967942,
            "unit": "iter/sec",
            "range": "stddev: 3.92779284730573e-7",
            "extra": "mean: 2.7311155041379527 usec\nrounds: 187266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 119845.0384887461,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013888794358273032",
            "extra": "mean: 8.344108463813493 usec\nrounds: 127001"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 877303.050904343,
            "unit": "iter/sec",
            "range": "stddev: 2.1957531583019397e-7",
            "extra": "mean: 1.1398569729914607 usec\nrounds: 92765"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 701089.4229179606,
            "unit": "iter/sec",
            "range": "stddev: 4.106485143886124e-7",
            "extra": "mean: 1.426351571298788 usec\nrounds: 74047"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 447457.973621283,
            "unit": "iter/sec",
            "range": "stddev: 2.978547107129456e-7",
            "extra": "mean: 2.2348467542258534 usec\nrounds: 46599"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 169350.29121711064,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012187993250705323",
            "extra": "mean: 5.904920462864625 usec\nrounds: 179212"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5512128.849726666,
            "unit": "iter/sec",
            "range": "stddev: 1.3788289264479793e-8",
            "extra": "mean: 181.4181103639499 nsec\nrounds: 56974"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 411948.57255452295,
            "unit": "iter/sec",
            "range": "stddev: 1.59760621422946e-7",
            "extra": "mean: 2.427487474465392 usec\nrounds: 42601"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 55358.23598445014,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012694900589733669",
            "extra": "mean: 18.06415941940229 usec\nrounds: 57101"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4026.5787295518508,
            "unit": "iter/sec",
            "range": "stddev: 0.000011545897580716069",
            "extra": "mean: 248.34979449446845 usec\nrounds: 4141"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4544104.996500602,
            "unit": "iter/sec",
            "range": "stddev: 1.2892638919304965e-8",
            "extra": "mean: 220.06533756810853 nsec\nrounds: 46124"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 378014.21310101385,
            "unit": "iter/sec",
            "range": "stddev: 3.81034470543687e-7",
            "extra": "mean: 2.6454031762365973 usec\nrounds: 199641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2515705.955481215,
            "unit": "iter/sec",
            "range": "stddev: 4.9380007418671606e-8",
            "extra": "mean: 397.50273589055007 nsec\nrounds: 199204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 47739.00884028312,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019693766514139825",
            "extra": "mean: 20.94723003876403 usec\nrounds: 49809"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 113660.32931507975,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010124164682483707",
            "extra": "mean: 8.798144489163697 usec\nrounds: 117995"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14033.315970250029,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032402140347105723",
            "extra": "mean: 71.25899553034743 usec\nrounds: 14319"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1329.6639311356162,
            "unit": "iter/sec",
            "range": "stddev: 0.000014652838798475654",
            "extra": "mean: 752.0697347531549 usec\nrounds: 1361"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 50934.14636741015,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017287977568390522",
            "extra": "mean: 19.63319445439539 usec\nrounds: 53519"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 48813.43991662236,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018997077539458578",
            "extra": "mean: 20.486161223386176 usec\nrounds: 51531"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35820.31916148707,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023521825254192374",
            "extra": "mean: 27.91711585515882 usec\nrounds: 37426"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 69628.67975891966,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027428639812502408",
            "extra": "mean: 14.361898049228724 usec\nrounds: 118822"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 249576.45385731853,
            "unit": "iter/sec",
            "range": "stddev: 4.963699616400746e-7",
            "extra": "mean: 4.006788238812361 usec\nrounds: 128966"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 33506.39174952104,
            "unit": "iter/sec",
            "range": "stddev: 0.0000056069864212944325",
            "extra": "mean: 29.845051877730004 usec\nrounds: 49366"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32193.522915908103,
            "unit": "iter/sec",
            "range": "stddev: 0.000001884441915394048",
            "extra": "mean: 31.062148824534518 usec\nrounds: 32965"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 9003.180866190114,
            "unit": "iter/sec",
            "range": "stddev: 0.000041249221525758746",
            "extra": "mean: 111.07185503240602 usec\nrounds: 12320"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3107.5122424097317,
            "unit": "iter/sec",
            "range": "stddev: 0.000005897369348435062",
            "extra": "mean: 321.80082393643164 usec\nrounds: 3175"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 17034.5453406694,
            "unit": "iter/sec",
            "range": "stddev: 0.000011867924205370596",
            "extra": "mean: 58.704237771027195 usec\nrounds: 22753"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28230.03371124315,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022473254395153134",
            "extra": "mean: 35.423266235836294 usec\nrounds: 28625"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 32200.295825660673,
            "unit": "iter/sec",
            "range": "stddev: 0.0000038017618176363022",
            "extra": "mean: 31.05561530907092 usec\nrounds: 46861"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 25092.089060978768,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027350208751693482",
            "extra": "mean: 39.853198255824815 usec\nrounds: 26027"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 27749.29013587234,
            "unit": "iter/sec",
            "range": "stddev: 0.0000064072252223352985",
            "extra": "mean: 36.036957886258506 usec\nrounds: 40248"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22986.86855093876,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020338108885902308",
            "extra": "mean: 43.5030982051342 usec\nrounds: 23675"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16510.529181125148,
            "unit": "iter/sec",
            "range": "stddev: 0.000005372458597018973",
            "extra": "mean: 60.56741059173324 usec\nrounds: 20865"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21162.047706194808,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022863915828774067",
            "extra": "mean: 47.25440627880581 usec\nrounds: 21596"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 33649.48595500801,
            "unit": "iter/sec",
            "range": "stddev: 0.000006062876163650071",
            "extra": "mean: 29.71813600175284 usec\nrounds: 57036"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 48610.613947815655,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014247707189419724",
            "extra": "mean: 20.571638964970024 usec\nrounds: 49660"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4634.229059839093,
            "unit": "iter/sec",
            "range": "stddev: 0.00007162934549841196",
            "extra": "mean: 215.78562196377953 usec\nrounds: 6465"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 900.6270137469073,
            "unit": "iter/sec",
            "range": "stddev: 0.00001239370367109123",
            "extra": "mean: 1.1103375589853428 msec\nrounds: 907"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1044.4981373633311,
            "unit": "iter/sec",
            "range": "stddev: 0.00015092070452373715",
            "extra": "mean: 957.3975905063271 usec\nrounds: 1243"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 276.6498521784564,
            "unit": "iter/sec",
            "range": "stddev: 0.00018873754216818672",
            "extra": "mean: 3.614677514285956 msec\nrounds: 280"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 58978.60991875634,
            "unit": "iter/sec",
            "range": "stddev: 0.000004008261059215077",
            "extra": "mean: 16.95529958026326 usec\nrounds: 96712"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25760.151100436786,
            "unit": "iter/sec",
            "range": "stddev: 0.000006038348557889231",
            "extra": "mean: 38.8196480719806 usec\nrounds: 36928"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5931.77286029691,
            "unit": "iter/sec",
            "range": "stddev: 0.00005800757023068012",
            "extra": "mean: 168.583663527188 usec\nrounds: 8494"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1747.8577852135797,
            "unit": "iter/sec",
            "range": "stddev: 0.00009085966825652654",
            "extra": "mean: 572.1289274560772 usec\nrounds: 1985"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 32005.187395151635,
            "unit": "iter/sec",
            "range": "stddev: 0.000005514986419638388",
            "extra": "mean: 31.24493500548873 usec\nrounds: 47081"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1734.3338762267156,
            "unit": "iter/sec",
            "range": "stddev: 0.00008397075498448491",
            "extra": "mean: 576.5902481104958 usec\nrounds: 1983"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1149.8159064797171,
            "unit": "iter/sec",
            "range": "stddev: 0.0001422048328457279",
            "extra": "mean: 869.7044408279285 usec\nrounds: 1352"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Joe Howarth",
            "username": "JoeHowarth",
            "email": "josephehowarth@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f9dfb0553351970c1eee00f13e9120c571a99926",
          "message": "Add Rust flags for macOS target configurations (#2)\n\nAdded target-specific Rust flags to prevent link errors on macOS.",
          "timestamp": "2026-01-05T04:54:01Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/f9dfb0553351970c1eee00f13e9120c571a99926"
        },
        "date": 1768786363180,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5198036.445840855,
            "unit": "iter/sec",
            "range": "stddev: 1.3055844925839296e-8",
            "extra": "mean: 192.38033638644004 nsec\nrounds: 52618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4883604.906433098,
            "unit": "iter/sec",
            "range": "stddev: 2.2222736468526135e-8",
            "extra": "mean: 204.7667694580957 nsec\nrounds: 49809"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3139201.0028175996,
            "unit": "iter/sec",
            "range": "stddev: 3.8799119089178346e-8",
            "extra": "mean: 318.5523956900213 nsec\nrounds: 188715"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 690413.2176424986,
            "unit": "iter/sec",
            "range": "stddev: 4.3876043071020946e-7",
            "extra": "mean: 1.4484079598224215 usec\nrounds: 70592"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 75581.71528806954,
            "unit": "iter/sec",
            "range": "stddev: 0.000001217449122709005",
            "extra": "mean: 13.230713224602468 usec\nrounds: 79027"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 404001.78273996594,
            "unit": "iter/sec",
            "range": "stddev: 1.78240993802949e-7",
            "extra": "mean: 2.475236602219744 usec\nrounds: 41798"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 400104.13509003614,
            "unit": "iter/sec",
            "range": "stddev: 1.7736022276999377e-7",
            "extra": "mean: 2.499349325082506 usec\nrounds: 41486"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 352256.6439826541,
            "unit": "iter/sec",
            "range": "stddev: 3.8859942199944307e-7",
            "extra": "mean: 2.838839286873017 usec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 208225.46203879043,
            "unit": "iter/sec",
            "range": "stddev: 5.556098514967393e-7",
            "extra": "mean: 4.802486642165353 usec\nrounds: 108850"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39368.139622472314,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017068484786065266",
            "extra": "mean: 25.401251102787064 usec\nrounds: 40808"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3643780.113491877,
            "unit": "iter/sec",
            "range": "stddev: 3.3513449097129696e-8",
            "extra": "mean: 274.44027050313633 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3407991.275483354,
            "unit": "iter/sec",
            "range": "stddev: 3.692660953656728e-8",
            "extra": "mean: 293.42798122548834 nsec\nrounds: 193799"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2201588.0817087134,
            "unit": "iter/sec",
            "range": "stddev: 5.757762246573649e-8",
            "extra": "mean: 454.2175751713999 nsec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 482509.38313288183,
            "unit": "iter/sec",
            "range": "stddev: 3.784610047565033e-7",
            "extra": "mean: 2.0724985564158165 usec\nrounds: 49391"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1529496.8660503451,
            "unit": "iter/sec",
            "range": "stddev: 7.689261284319457e-8",
            "extra": "mean: 653.8097737867827 nsec\nrounds: 155958"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1507050.4080276978,
            "unit": "iter/sec",
            "range": "stddev: 8.272421143926441e-8",
            "extra": "mean: 663.5478114555133 nsec\nrounds: 153093"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1298205.257891835,
            "unit": "iter/sec",
            "range": "stddev: 9.034592901219438e-8",
            "extra": "mean: 770.2942149718123 nsec\nrounds: 133085"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 445751.79678662436,
            "unit": "iter/sec",
            "range": "stddev: 1.4740042628289113e-7",
            "extra": "mean: 2.243400940184382 usec\nrounds: 45204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2721624.9872640343,
            "unit": "iter/sec",
            "range": "stddev: 4.700297191989076e-8",
            "extra": "mean: 367.4275495997944 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 656492.6782343651,
            "unit": "iter/sec",
            "range": "stddev: 1.312142119514768e-7",
            "extra": "mean: 1.5232462343517776 usec\nrounds: 67173"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2929829.590421728,
            "unit": "iter/sec",
            "range": "stddev: 4.2909252346817275e-8",
            "extra": "mean: 341.3167794021963 nsec\nrounds: 189790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2835647.440692141,
            "unit": "iter/sec",
            "range": "stddev: 4.39159909309209e-8",
            "extra": "mean: 352.6531492067507 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1816221.4664238954,
            "unit": "iter/sec",
            "range": "stddev: 6.78745432356891e-8",
            "extra": "mean: 550.5936464725852 nsec\nrounds: 187266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 499130.9801675267,
            "unit": "iter/sec",
            "range": "stddev: 1.4210180190968837e-7",
            "extra": "mean: 2.0034821314124334 usec\nrounds: 50821"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47794.72513729085,
            "unit": "iter/sec",
            "range": "stddev: 0.000001661880738320734",
            "extra": "mean: 20.92281098233099 usec\nrounds: 49170"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1206225.6185290706,
            "unit": "iter/sec",
            "range": "stddev: 8.666920408343475e-8",
            "extra": "mean: 829.0323009549711 nsec\nrounds: 126343"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1190686.0579691438,
            "unit": "iter/sec",
            "range": "stddev: 9.282321562031363e-8",
            "extra": "mean: 839.8519435976547 nsec\nrounds: 123534"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 898069.2447046164,
            "unit": "iter/sec",
            "range": "stddev: 1.1102669703305022e-7",
            "extra": "mean: 1.1134998842198378 usec\nrounds: 93284"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 349371.34228362574,
            "unit": "iter/sec",
            "range": "stddev: 3.7425428731635086e-7",
            "extra": "mean: 2.8622839911929083 usec\nrounds: 182150"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 45397.478042096016,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017354192884945447",
            "extra": "mean: 22.027655348447407 usec\nrounds: 46621"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2813451.7546723415,
            "unit": "iter/sec",
            "range": "stddev: 4.337801757947406e-8",
            "extra": "mean: 355.43527566790186 nsec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2242561.9646358355,
            "unit": "iter/sec",
            "range": "stddev: 1.1682930579299767e-7",
            "extra": "mean: 445.91855911656035 nsec\nrounds: 195734"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 781549.9065578723,
            "unit": "iter/sec",
            "range": "stddev: 1.0284743615179672e-7",
            "extra": "mean: 1.2795088216493384 usec\nrounds: 79917"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107838.82410704061,
            "unit": "iter/sec",
            "range": "stddev: 8.574586601557498e-7",
            "extra": "mean: 9.273098146984632 usec\nrounds: 109927"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 447761.14896888984,
            "unit": "iter/sec",
            "range": "stddev: 1.6297764170058396e-7",
            "extra": "mean: 2.233333558087493 usec\nrounds: 45828"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 441764.1156572147,
            "unit": "iter/sec",
            "range": "stddev: 1.643960363840498e-7",
            "extra": "mean: 2.2636514930876364 usec\nrounds: 45041"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 332806.90301377885,
            "unit": "iter/sec",
            "range": "stddev: 4.1870023759523665e-7",
            "extra": "mean: 3.004745367191491 usec\nrounds: 172682"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 140752.48905867245,
            "unit": "iter/sec",
            "range": "stddev: 9.361459526832689e-7",
            "extra": "mean: 7.104670096335928 usec\nrounds: 145497"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2700731.9011082267,
            "unit": "iter/sec",
            "range": "stddev: 5.118697128622893e-7",
            "extra": "mean: 370.26999962108675 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2868205.9345608554,
            "unit": "iter/sec",
            "range": "stddev: 2.305347927222458e-7",
            "extra": "mean: 348.6500003191395 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2069279.482441599,
            "unit": "iter/sec",
            "range": "stddev: 1.4780622197367687e-7",
            "extra": "mean: 483.25999870257874 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 461392.9453425865,
            "unit": "iter/sec",
            "range": "stddev: 9.095107823969975e-7",
            "extra": "mean: 2.167349999808721 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 53302.1480254769,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012083658179915746",
            "extra": "mean: 18.760969999220833 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 887689.5211644282,
            "unit": "iter/sec",
            "range": "stddev: 8.141086778867863e-7",
            "extra": "mean: 1.1265200006960185 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 913125.2632193674,
            "unit": "iter/sec",
            "range": "stddev: 4.773332294036145e-7",
            "extra": "mean: 1.0951399991654398 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 767436.1490886017,
            "unit": "iter/sec",
            "range": "stddev: 4.1838460398428536e-7",
            "extra": "mean: 1.3030400003799514 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 377513.7700570912,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010533102804820528",
            "extra": "mean: 2.6489099982995867 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 55620.66263375773,
            "unit": "iter/sec",
            "range": "stddev: 0.000002000742305454147",
            "extra": "mean: 17.978929999173943 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1602569.474978026,
            "unit": "iter/sec",
            "range": "stddev: 7.403188130223705e-8",
            "extra": "mean: 623.9979081179821 nsec\nrounds: 168891"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 373052.36238490633,
            "unit": "iter/sec",
            "range": "stddev: 3.297752052619071e-7",
            "extra": "mean: 2.6805888417568156 usec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 45371.48293086683,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014667131127540514",
            "extra": "mean: 22.04027586058217 usec\nrounds: 46190"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4625.837030640968,
            "unit": "iter/sec",
            "range": "stddev: 0.000005673771600261995",
            "extra": "mean: 216.17709257289536 usec\nrounds: 4699"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 210764.43548830604,
            "unit": "iter/sec",
            "range": "stddev: 5.49161321197495e-7",
            "extra": "mean: 4.744633494181154 usec\nrounds: 109207"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 134518.05254048426,
            "unit": "iter/sec",
            "range": "stddev: 9.633412955196299e-7",
            "extra": "mean: 7.433946456361626 usec\nrounds: 139998"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31815.013246031955,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019145222543090147",
            "extra": "mean: 31.43170151358407 usec\nrounds: 32705"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3723.7467837895306,
            "unit": "iter/sec",
            "range": "stddev: 0.0000053575644634483164",
            "extra": "mean: 268.54672405580004 usec\nrounds: 3787"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 135935.9147678119,
            "unit": "iter/sec",
            "range": "stddev: 9.104778208918049e-7",
            "extra": "mean: 7.356407625667362 usec\nrounds: 143411"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16807.765359763187,
            "unit": "iter/sec",
            "range": "stddev: 0.000003181635870552719",
            "extra": "mean: 59.49630891408932 usec\nrounds: 17377"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1612.0539280850796,
            "unit": "iter/sec",
            "range": "stddev: 0.00002686112500900134",
            "extra": "mean: 620.3266420422276 usec\nrounds: 1665"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1187244.7112528416,
            "unit": "iter/sec",
            "range": "stddev: 1.0745069091874579e-7",
            "extra": "mean: 842.286337872807 nsec\nrounds: 126503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 103939.9186667748,
            "unit": "iter/sec",
            "range": "stddev: 8.743728538926515e-7",
            "extra": "mean: 9.620942683300923 usec\nrounds: 106531"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17155.063898526638,
            "unit": "iter/sec",
            "range": "stddev: 0.000002342029939879689",
            "extra": "mean: 58.29182601213657 usec\nrounds: 17438"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2377.5553396836235,
            "unit": "iter/sec",
            "range": "stddev: 0.000005319103950850566",
            "extra": "mean: 420.60009426870715 usec\nrounds: 2408"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 720937.016253021,
            "unit": "iter/sec",
            "range": "stddev: 1.1655811697271338e-7",
            "extra": "mean: 1.3870837222332628 usec\nrounds: 76841"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 104575.3620448425,
            "unit": "iter/sec",
            "range": "stddev: 0.000001044173428023597",
            "extra": "mean: 9.562481835551232 usec\nrounds: 107435"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21385.47246724138,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029825796187985153",
            "extra": "mean: 46.7607157864675 usec\nrounds: 22557"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4410.745319813689,
            "unit": "iter/sec",
            "range": "stddev: 0.000015914562071782516",
            "extra": "mean: 226.71905256190135 usec\nrounds: 4566"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2104295.649123088,
            "unit": "iter/sec",
            "range": "stddev: 5.777795754158155e-8",
            "extra": "mean: 475.21839453344813 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 973825.5235820051,
            "unit": "iter/sec",
            "range": "stddev: 9.248988678474064e-8",
            "extra": "mean: 1.0268779938337265 usec\nrounds: 99513"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 357643.69930213713,
            "unit": "iter/sec",
            "range": "stddev: 3.558184412920143e-7",
            "extra": "mean: 2.79607889626262 usec\nrounds: 182816"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 121368.51639351438,
            "unit": "iter/sec",
            "range": "stddev: 9.753353466552044e-7",
            "extra": "mean: 8.239369069633263 usec\nrounds: 125708"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 906913.753521458,
            "unit": "iter/sec",
            "range": "stddev: 9.818294937047098e-8",
            "extra": "mean: 1.1026406823328891 usec\nrounds: 92507"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 721551.6637650568,
            "unit": "iter/sec",
            "range": "stddev: 1.1386762259011918e-7",
            "extra": "mean: 1.3859021470229782 usec\nrounds: 74102"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 495970.99645709904,
            "unit": "iter/sec",
            "range": "stddev: 1.388384763022717e-7",
            "extra": "mean: 2.0162469320653256 usec\nrounds: 50718"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 173304.25301932442,
            "unit": "iter/sec",
            "range": "stddev: 8.275781812734305e-7",
            "extra": "mean: 5.770198841505028 usec\nrounds: 183151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5588316.545041559,
            "unit": "iter/sec",
            "range": "stddev: 1.3450918731110928e-8",
            "extra": "mean: 178.9447666286853 nsec\nrounds: 56841"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 397159.091909942,
            "unit": "iter/sec",
            "range": "stddev: 1.870752515943137e-7",
            "extra": "mean: 2.5178826832114094 usec\nrounds: 41555"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 55875.95217344646,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012488550262056708",
            "extra": "mean: 17.896786741027086 usec\nrounds: 57531"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4044.1894556108205,
            "unit": "iter/sec",
            "range": "stddev: 0.0000062243471403884935",
            "extra": "mean: 247.26833670282727 usec\nrounds: 4155"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4444411.662660298,
            "unit": "iter/sec",
            "range": "stddev: 2.1164670609072883e-8",
            "extra": "mean: 225.00165959003272 nsec\nrounds: 45451"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 372909.42373909574,
            "unit": "iter/sec",
            "range": "stddev: 3.805437212432241e-7",
            "extra": "mean: 2.6816163291696404 usec\nrounds: 198020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2435763.509338356,
            "unit": "iter/sec",
            "range": "stddev: 5.088984406621679e-8",
            "extra": "mean: 410.54888792212597 nsec\nrounds: 192308"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 49017.99767440956,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021317774380253",
            "extra": "mean: 20.40067010982911 usec\nrounds: 51214"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 110754.07660276338,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010087641976635118",
            "extra": "mean: 9.029013023029885 usec\nrounds: 115795"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13658.009317947939,
            "unit": "iter/sec",
            "range": "stddev: 0.0000033845328583140437",
            "extra": "mean: 73.21711215161523 usec\nrounds: 14097"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1302.384945551825,
            "unit": "iter/sec",
            "range": "stddev: 0.000014389067016991134",
            "extra": "mean: 767.8221430732958 usec\nrounds: 1328"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51535.26334416887,
            "unit": "iter/sec",
            "range": "stddev: 0.000001973735458151892",
            "extra": "mean: 19.404189192198015 usec\nrounds: 52925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49523.417325075236,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018356961862109175",
            "extra": "mean: 20.192467604485547 usec\nrounds: 51396"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 34829.488527044494,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024753966152248446",
            "extra": "mean: 28.71130304493324 usec\nrounds: 37077"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 69155.83002884084,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027797322043546105",
            "extra": "mean: 14.46009685059031 usec\nrounds: 174795"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 250306.09746157052,
            "unit": "iter/sec",
            "range": "stddev: 4.701449013081698e-7",
            "extra": "mean: 3.9951084298037522 usec\nrounds: 130822"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 32244.914146022486,
            "unit": "iter/sec",
            "range": "stddev: 0.000008006163681489311",
            "extra": "mean: 31.01264265959763 usec\nrounds: 49194"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32149.698837508375,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018059854760736832",
            "extra": "mean: 31.10449043564045 usec\nrounds: 33405"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8705.986072252656,
            "unit": "iter/sec",
            "range": "stddev: 0.00003736911010957494",
            "extra": "mean: 114.86349641508812 usec\nrounds: 12693"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3114.186446107449,
            "unit": "iter/sec",
            "range": "stddev: 0.0000064251828216581244",
            "extra": "mean: 321.11115288230144 usec\nrounds: 3192"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 18018.751774451393,
            "unit": "iter/sec",
            "range": "stddev: 0.000004577833580869642",
            "extra": "mean: 55.49773993878366 usec\nrounds: 22810"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28249.43655562567,
            "unit": "iter/sec",
            "range": "stddev: 0.000002006101282487884",
            "extra": "mean: 35.39893611792612 usec\nrounds: 28991"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 32424.950520630628,
            "unit": "iter/sec",
            "range": "stddev: 0.0000034455738279533785",
            "extra": "mean: 30.84044798661272 usec\nrounds: 47084"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24752.796850206236,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020854110648331622",
            "extra": "mean: 40.39947509978729 usec\nrounds: 25321"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 27949.17609682008,
            "unit": "iter/sec",
            "range": "stddev: 0.0000051286740778788894",
            "extra": "mean: 35.7792300043426 usec\nrounds: 40134"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23188.55968891027,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021666095747108245",
            "extra": "mean: 43.124713799203384 usec\nrounds: 23777"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16260.35611719574,
            "unit": "iter/sec",
            "range": "stddev: 0.00000777378456629386",
            "extra": "mean: 61.49926808444709 usec\nrounds: 21206"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21259.179404782666,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023703489036044275",
            "extra": "mean: 47.038504213150894 usec\nrounds: 21600"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 35438.02066022841,
            "unit": "iter/sec",
            "range": "stddev: 0.000004791250963390152",
            "extra": "mean: 28.218280292451148 usec\nrounds: 54964"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47138.37381055558,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014242717944519016",
            "extra": "mean: 21.21413869767549 usec\nrounds: 48977"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4543.220557609681,
            "unit": "iter/sec",
            "range": "stddev: 0.00007819022944379115",
            "extra": "mean: 220.10817817881346 usec\nrounds: 6370"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 908.6884675610651,
            "unit": "iter/sec",
            "range": "stddev: 0.000012277377720869098",
            "extra": "mean: 1.1004871699142573 msec\nrounds: 924"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1050.4469533620918,
            "unit": "iter/sec",
            "range": "stddev: 0.00014585773595575282",
            "extra": "mean: 951.9757249991255 usec\nrounds: 1240"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 278.49655823336803,
            "unit": "iter/sec",
            "range": "stddev: 0.00006758562552679818",
            "extra": "mean: 3.590708647688362 msec\nrounds: 281"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 59480.86118070195,
            "unit": "iter/sec",
            "range": "stddev: 0.0000031986100067999204",
            "extra": "mean: 16.812130492899477 usec\nrounds: 95239"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25571.715238907465,
            "unit": "iter/sec",
            "range": "stddev: 0.000008028614844207487",
            "extra": "mean: 39.10570685843146 usec\nrounds: 37064"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5980.689681128073,
            "unit": "iter/sec",
            "range": "stddev: 0.00005712531013159941",
            "extra": "mean: 167.2047963223166 usec\nrounds: 8430"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1741.0453796157765,
            "unit": "iter/sec",
            "range": "stddev: 0.00009337079614501421",
            "extra": "mean: 574.3675677314543 usec\nrounds: 1971"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 32973.62790034385,
            "unit": "iter/sec",
            "range": "stddev: 0.000003499082861504488",
            "extra": "mean: 30.32726647556947 usec\nrounds: 47783"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1737.8863375492035,
            "unit": "iter/sec",
            "range": "stddev: 0.00008639216779436607",
            "extra": "mean: 575.4116241055309 usec\nrounds: 1958"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1149.2242244872086,
            "unit": "iter/sec",
            "range": "stddev: 0.0001399891242394763",
            "extra": "mean: 870.1522111111141 usec\nrounds: 1350"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Joe Howarth",
            "username": "JoeHowarth",
            "email": "josephehowarth@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f9dfb0553351970c1eee00f13e9120c571a99926",
          "message": "Add Rust flags for macOS target configurations (#2)\n\nAdded target-specific Rust flags to prevent link errors on macOS.",
          "timestamp": "2026-01-05T04:54:01Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/f9dfb0553351970c1eee00f13e9120c571a99926"
        },
        "date": 1769391327921,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5278381.709075106,
            "unit": "iter/sec",
            "range": "stddev: 1.2357431907795168e-8",
            "extra": "mean: 189.45200539041744 nsec\nrounds: 53810"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4953383.1687859455,
            "unit": "iter/sec",
            "range": "stddev: 1.211833962757687e-8",
            "extra": "mean: 201.88222189258573 nsec\nrounds: 50259"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3182741.1274174987,
            "unit": "iter/sec",
            "range": "stddev: 3.4560263238193964e-8",
            "extra": "mean: 314.19457629947425 nsec\nrounds: 190840"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 692525.5944868769,
            "unit": "iter/sec",
            "range": "stddev: 9.71854373947869e-8",
            "extra": "mean: 1.44398995208392 usec\nrounds: 70542"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 74441.33393142333,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012117024658037493",
            "extra": "mean: 13.433397108671072 usec\nrounds: 77616"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 417367.33334053354,
            "unit": "iter/sec",
            "range": "stddev: 1.8881234279737594e-7",
            "extra": "mean: 2.395970935233912 usec\nrounds: 42801"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 415286.8378229721,
            "unit": "iter/sec",
            "range": "stddev: 2.460794057355293e-7",
            "extra": "mean: 2.4079742214856252 usec\nrounds: 42710"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 359490.664380872,
            "unit": "iter/sec",
            "range": "stddev: 3.765352965681419e-7",
            "extra": "mean: 2.7817134047757173 usec\nrounds: 186568"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 216127.0358867307,
            "unit": "iter/sec",
            "range": "stddev: 4.976024635332591e-7",
            "extra": "mean: 4.626908410126378 usec\nrounds: 111770"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39847.87410284289,
            "unit": "iter/sec",
            "range": "stddev: 0.00000169900626864863",
            "extra": "mean: 25.095441664443936 usec\nrounds: 40807"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3614232.7159284265,
            "unit": "iter/sec",
            "range": "stddev: 3.1659266470998717e-8",
            "extra": "mean: 276.68389907300724 nsec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3367352.113797001,
            "unit": "iter/sec",
            "range": "stddev: 3.292448731445763e-8",
            "extra": "mean: 296.96924058011183 nsec\nrounds: 192716"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2187898.9246893763,
            "unit": "iter/sec",
            "range": "stddev: 5.6685621149030686e-8",
            "extra": "mean: 457.0595052246168 nsec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 506439.9266429397,
            "unit": "iter/sec",
            "range": "stddev: 1.3820239159782995e-7",
            "extra": "mean: 1.974567855715363 usec\nrounds: 52395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1509041.4160763065,
            "unit": "iter/sec",
            "range": "stddev: 8.084647575053069e-8",
            "extra": "mean: 662.6723357931954 nsec\nrounds: 154727"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1485260.2761776347,
            "unit": "iter/sec",
            "range": "stddev: 7.410727198919613e-8",
            "extra": "mean: 673.2826670443991 nsec\nrounds: 152161"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1314304.21066916,
            "unit": "iter/sec",
            "range": "stddev: 1.2412596787914063e-7",
            "extra": "mean: 760.8588573956067 nsec\nrounds: 135796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 442210.5960669682,
            "unit": "iter/sec",
            "range": "stddev: 1.702490083945243e-7",
            "extra": "mean: 2.2613659846553276 usec\nrounds: 45062"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2719265.657966411,
            "unit": "iter/sec",
            "range": "stddev: 6.801068351962513e-8",
            "extra": "mean: 367.7463424989787 nsec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 686922.7573592435,
            "unit": "iter/sec",
            "range": "stddev: 1.1851943743600813e-7",
            "extra": "mean: 1.4557677545061027 usec\nrounds: 71089"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3060858.0501142964,
            "unit": "iter/sec",
            "range": "stddev: 4.380878923392494e-8",
            "extra": "mean: 326.70577453360136 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2936873.3679900425,
            "unit": "iter/sec",
            "range": "stddev: 4.431806852610563e-8",
            "extra": "mean: 340.49816750677473 nsec\nrounds: 199601"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1841356.195900628,
            "unit": "iter/sec",
            "range": "stddev: 6.260425090805285e-8",
            "extra": "mean: 543.0779781914775 nsec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 491819.18985059136,
            "unit": "iter/sec",
            "range": "stddev: 1.4858055473224754e-7",
            "extra": "mean: 2.0332675516459577 usec\nrounds: 50058"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36458.168577510485,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017261292616366128",
            "extra": "mean: 27.428695379308166 usec\nrounds: 37440"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1200048.365334761,
            "unit": "iter/sec",
            "range": "stddev: 8.216938236175729e-8",
            "extra": "mean: 833.2997476489667 nsec\nrounds: 125220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1185646.8945338638,
            "unit": "iter/sec",
            "range": "stddev: 8.188130568286365e-8",
            "extra": "mean: 843.4214306217652 nsec\nrounds: 122325"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 912983.1090369218,
            "unit": "iter/sec",
            "range": "stddev: 9.93906749331251e-8",
            "extra": "mean: 1.095310515716819 usec\nrounds: 94877"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 353509.31087246555,
            "unit": "iter/sec",
            "range": "stddev: 3.6026747762281707e-7",
            "extra": "mean: 2.828779806483702 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34776.951921305736,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019406472892597503",
            "extra": "mean: 28.754676438085436 usec\nrounds: 36126"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2908069.1667161584,
            "unit": "iter/sec",
            "range": "stddev: 4.202445602598666e-8",
            "extra": "mean: 343.87077564913494 nsec\nrounds: 196464"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2264292.308212935,
            "unit": "iter/sec",
            "range": "stddev: 5.535338758128254e-8",
            "extra": "mean: 441.63909243203585 nsec\nrounds: 198020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 790854.4542819123,
            "unit": "iter/sec",
            "range": "stddev: 9.431835928194769e-8",
            "extra": "mean: 1.2644551656575245 usec\nrounds: 80561"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107268.41797007826,
            "unit": "iter/sec",
            "range": "stddev: 8.272322550677151e-7",
            "extra": "mean: 9.322408393111035 usec\nrounds: 109566"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 449906.08336898213,
            "unit": "iter/sec",
            "range": "stddev: 1.6420630733125784e-7",
            "extra": "mean: 2.2226861048684117 usec\nrounds: 46340"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 444864.9587542509,
            "unit": "iter/sec",
            "range": "stddev: 1.6075869798188634e-7",
            "extra": "mean: 2.247873158632878 usec\nrounds: 45143"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 339279.17978906934,
            "unit": "iter/sec",
            "range": "stddev: 3.993533734865901e-7",
            "extra": "mean: 2.9474251871915698 usec\nrounds: 176026"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 128880.65281454947,
            "unit": "iter/sec",
            "range": "stddev: 9.216220767047328e-7",
            "extra": "mean: 7.759116501674866 usec\nrounds: 132539"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2645572.6114908857,
            "unit": "iter/sec",
            "range": "stddev: 5.442492468494109e-7",
            "extra": "mean: 377.9900032441219 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2808909.8771844287,
            "unit": "iter/sec",
            "range": "stddev: 2.020648687438178e-7",
            "extra": "mean: 356.0099980859377 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1978983.207360569,
            "unit": "iter/sec",
            "range": "stddev: 1.8188273184106279e-7",
            "extra": "mean: 505.3099977203601 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 503722.509619472,
            "unit": "iter/sec",
            "range": "stddev: 2.5441168439786856e-7",
            "extra": "mean: 1.985219998914545 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 58064.59104612946,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016980913955605062",
            "extra": "mean: 17.2222000014699 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 846582.3458176536,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013232866579731645",
            "extra": "mean: 1.1812200017402574 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 940866.5399588698,
            "unit": "iter/sec",
            "range": "stddev: 4.2571844088306036e-7",
            "extra": "mean: 1.0628499978793116 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 631476.5818740436,
            "unit": "iter/sec",
            "range": "stddev: 0.000002385085213558943",
            "extra": "mean: 1.5835899995408909 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 358978.7771029709,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011215279724042245",
            "extra": "mean: 2.7856800005565674 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 56024.6374107292,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018645451535459107",
            "extra": "mean: 17.849289994842366 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1556836.1139813815,
            "unit": "iter/sec",
            "range": "stddev: 7.158401286607349e-8",
            "extra": "mean: 642.3283677834954 nsec\nrounds: 159949"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 336658.63701128226,
            "unit": "iter/sec",
            "range": "stddev: 3.307725789154804e-7",
            "extra": "mean: 2.9703678743477107 usec\nrounds: 173914"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 39567.64426723087,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014111609282148405",
            "extra": "mean: 25.273175052985906 usec\nrounds: 40542"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4000.9204910297276,
            "unit": "iter/sec",
            "range": "stddev: 0.000005995201958204119",
            "extra": "mean: 249.942482546717 usec\nrounds: 4068"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 211478.17756837732,
            "unit": "iter/sec",
            "range": "stddev: 5.612750152773612e-7",
            "extra": "mean: 4.728620283653947 usec\nrounds: 109566"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 134834.37078236288,
            "unit": "iter/sec",
            "range": "stddev: 9.292013707402546e-7",
            "extra": "mean: 7.416506593961173 usec\nrounds: 140965"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 32119.47456408109,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018320281941548315",
            "extra": "mean: 31.133759613810454 usec\nrounds: 33051"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3726.1858504876095,
            "unit": "iter/sec",
            "range": "stddev: 0.000006573353325770937",
            "extra": "mean: 268.37094018516007 usec\nrounds: 3795"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 137236.69671061914,
            "unit": "iter/sec",
            "range": "stddev: 8.345243694704337e-7",
            "extra": "mean: 7.2866807783097975 usec\nrounds: 144655"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17026.332912589012,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029458128378196747",
            "extra": "mean: 58.73255298917685 usec\nrounds: 17447"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1632.6360039822275,
            "unit": "iter/sec",
            "range": "stddev: 0.00001683282681921269",
            "extra": "mean: 612.5063991978984 usec\nrounds: 1746"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1149593.9283638338,
            "unit": "iter/sec",
            "range": "stddev: 2.0356012052269065e-7",
            "extra": "mean: 869.8723743463811 nsec\nrounds: 119675"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 103833.79566260226,
            "unit": "iter/sec",
            "range": "stddev: 8.598679534911925e-7",
            "extra": "mean: 9.630775737500747 usec\nrounds: 106304"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17037.093579577082,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025317982596252206",
            "extra": "mean: 58.69545737535495 usec\nrounds: 17314"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2369.0692745340493,
            "unit": "iter/sec",
            "range": "stddev: 0.000005268527172248564",
            "extra": "mean: 422.10669428258103 usec\nrounds: 2414"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 713161.2829755922,
            "unit": "iter/sec",
            "range": "stddev: 1.1689618029836691e-7",
            "extra": "mean: 1.4022073602026868 usec\nrounds: 73449"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 105122.00621865268,
            "unit": "iter/sec",
            "range": "stddev: 9.154272628948752e-7",
            "extra": "mean: 9.51275604386783 usec\nrounds: 107794"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21436.716930268107,
            "unit": "iter/sec",
            "range": "stddev: 0.000005583323977411597",
            "extra": "mean: 46.64893431456498 usec\nrounds: 22486"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4232.808276885461,
            "unit": "iter/sec",
            "range": "stddev: 0.00002847135832619841",
            "extra": "mean: 236.24977428361322 usec\nrounds: 4643"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2122410.4210743527,
            "unit": "iter/sec",
            "range": "stddev: 5.4223688788866e-8",
            "extra": "mean: 471.1624057583698 nsec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 974628.6755471836,
            "unit": "iter/sec",
            "range": "stddev: 9.110719670631998e-8",
            "extra": "mean: 1.026031785324338 usec\nrounds: 99612"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 361949.38297192793,
            "unit": "iter/sec",
            "range": "stddev: 3.2910316701943634e-7",
            "extra": "mean: 2.762817253034406 usec\nrounds: 184843"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 124958.54259569873,
            "unit": "iter/sec",
            "range": "stddev: 0.000001603279244810435",
            "extra": "mean: 8.002654154150015 usec\nrounds: 125235"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 900638.0575286704,
            "unit": "iter/sec",
            "range": "stddev: 9.77244151166862e-8",
            "extra": "mean: 1.110323943831504 usec\nrounds: 95795"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 723216.671155442,
            "unit": "iter/sec",
            "range": "stddev: 1.1697087637905501e-7",
            "extra": "mean: 1.3827114886640584 usec\nrounds: 76371"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 455967.0078804765,
            "unit": "iter/sec",
            "range": "stddev: 1.4655988924282392e-7",
            "extra": "mean: 2.1931411323999304 usec\nrounds: 47262"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 201588.03195798927,
            "unit": "iter/sec",
            "range": "stddev: 7.34550987021444e-7",
            "extra": "mean: 4.960611948473205 usec\nrounds: 199641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5364067.394054809,
            "unit": "iter/sec",
            "range": "stddev: 4.174562244267323e-8",
            "extra": "mean: 186.42569649822033 nsec\nrounds: 55176"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 420718.90896070143,
            "unit": "iter/sec",
            "range": "stddev: 3.022546162357048e-7",
            "extra": "mean: 2.376883897304017 usec\nrounds: 43117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 57224.16364953558,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011938464246461552",
            "extra": "mean: 17.475135261467745 usec\nrounds: 58679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4139.704497234237,
            "unit": "iter/sec",
            "range": "stddev: 0.00000767557051010643",
            "extra": "mean: 241.56313588762347 usec\nrounds: 4202"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4382812.905315277,
            "unit": "iter/sec",
            "range": "stddev: 1.253235944198868e-8",
            "extra": "mean: 228.16397176964657 nsec\nrounds: 44421"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 400366.3460878812,
            "unit": "iter/sec",
            "range": "stddev: 2.845205888537933e-7",
            "extra": "mean: 2.497712432054662 usec\nrounds: 137666"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2534648.682040284,
            "unit": "iter/sec",
            "range": "stddev: 4.626828637604071e-8",
            "extra": "mean: 394.53199454647586 nsec\nrounds: 199641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 47166.734474696124,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019781115124081",
            "extra": "mean: 21.20138294790107 usec\nrounds: 49683"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 114719.9287042279,
            "unit": "iter/sec",
            "range": "stddev: 9.482697859352871e-7",
            "extra": "mean: 8.71688128902355 usec\nrounds: 120410"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13966.16482781663,
            "unit": "iter/sec",
            "range": "stddev: 0.000003718097195034364",
            "extra": "mean: 71.6016180768742 usec\nrounds: 14372"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1331.1061586009669,
            "unit": "iter/sec",
            "range": "stddev: 0.000013535720674313242",
            "extra": "mean: 751.254881917931 usec\nrounds: 1355"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 50554.74194835511,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017961647414045833",
            "extra": "mean: 19.7805381149322 usec\nrounds: 53260"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 48590.564722123534,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018132835317159774",
            "extra": "mean: 20.580127144410298 usec\nrounds: 51005"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 36044.9689765912,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024857214711737644",
            "extra": "mean: 27.74312278225106 usec\nrounds: 37766"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 57794.468147942884,
            "unit": "iter/sec",
            "range": "stddev: 0.000006426770901487473",
            "extra": "mean: 17.30269404746817 usec\nrounds: 130651"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 252523.02344536278,
            "unit": "iter/sec",
            "range": "stddev: 4.4349324298576457e-7",
            "extra": "mean: 3.96003495584776 usec\nrounds: 130822"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 34098.78443931361,
            "unit": "iter/sec",
            "range": "stddev: 0.0000034576277356649133",
            "extra": "mean: 29.326558598583564 usec\nrounds: 50641"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32912.04840020412,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020580942344342927",
            "extra": "mean: 30.38400976566983 usec\nrounds: 33383"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8837.708268158589,
            "unit": "iter/sec",
            "range": "stddev: 0.00003774046171761698",
            "extra": "mean: 113.15150598520022 usec\nrounds: 12364"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3188.7339291574713,
            "unit": "iter/sec",
            "range": "stddev: 0.000007026185924147293",
            "extra": "mean: 313.60408933968995 usec\nrounds: 3246"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 17265.13815545953,
            "unit": "iter/sec",
            "range": "stddev: 0.000010645661511702297",
            "extra": "mean: 57.920185230825 usec\nrounds: 22804"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 27897.627580027496,
            "unit": "iter/sec",
            "range": "stddev: 0.0000073910719748148716",
            "extra": "mean: 35.84534194283679 usec\nrounds: 29075"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 31889.17305683193,
            "unit": "iter/sec",
            "range": "stddev: 0.000004932462973903616",
            "extra": "mean: 31.358605574933847 usec\nrounds: 47104"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24819.159802836744,
            "unit": "iter/sec",
            "range": "stddev: 0.00000210462272165541",
            "extra": "mean: 40.29145256906334 usec\nrounds: 25859"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28202.5740752135,
            "unit": "iter/sec",
            "range": "stddev: 0.000003958791453480401",
            "extra": "mean: 35.45775634993806 usec\nrounds: 40509"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23440.510089799973,
            "unit": "iter/sec",
            "range": "stddev: 0.000002231919122822309",
            "extra": "mean: 42.66118766908342 usec\nrounds: 23925"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16462.16090438506,
            "unit": "iter/sec",
            "range": "stddev: 0.00000559162397371969",
            "extra": "mean: 60.745366650718864 usec\nrounds: 20930"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21237.10572562347,
            "unit": "iter/sec",
            "range": "stddev: 0.000002318744615083119",
            "extra": "mean: 47.08739566114499 usec\nrounds: 21756"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 35068.12334024798,
            "unit": "iter/sec",
            "range": "stddev: 0.000006172746012310632",
            "extra": "mean: 28.515925711151233 usec\nrounds: 51798"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 48447.1556548088,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014949250220230018",
            "extra": "mean: 20.64104665143002 usec\nrounds: 50181"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4601.509512440384,
            "unit": "iter/sec",
            "range": "stddev: 0.00007361028774114037",
            "extra": "mean: 217.31998973303342 usec\nrounds: 6331"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 933.2284775373803,
            "unit": "iter/sec",
            "range": "stddev: 0.000011498497651081033",
            "extra": "mean: 1.0715489551270634 msec\nrounds: 936"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1039.3256323318165,
            "unit": "iter/sec",
            "range": "stddev: 0.00015291015006525292",
            "extra": "mean: 962.1623569086947 usec\nrounds: 1230"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 282.16693271364227,
            "unit": "iter/sec",
            "range": "stddev: 0.000019656523076630058",
            "extra": "mean: 3.5440013838008873 msec\nrounds: 284"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 60975.9307346926,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026553526721948615",
            "extra": "mean: 16.39991367005152 usec\nrounds: 114456"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25601.30979090161,
            "unit": "iter/sec",
            "range": "stddev: 0.000007341109624462091",
            "extra": "mean: 39.06050151994128 usec\nrounds: 39143"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5934.979782317931,
            "unit": "iter/sec",
            "range": "stddev: 0.00005867916701944186",
            "extra": "mean: 168.49257060307053 usec\nrounds: 8477"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1707.085419698499,
            "unit": "iter/sec",
            "range": "stddev: 0.0001099862625343081",
            "extra": "mean: 585.793767822478 usec\nrounds: 1964"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 32959.04546940354,
            "unit": "iter/sec",
            "range": "stddev: 0.000004163647769336977",
            "extra": "mean: 30.340684499747343 usec\nrounds: 48057"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1674.2804984345123,
            "unit": "iter/sec",
            "range": "stddev: 0.00010908572569719006",
            "extra": "mean: 597.2714852350135 usec\nrounds: 1964"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1128.3817068340868,
            "unit": "iter/sec",
            "range": "stddev: 0.00015245451377736248",
            "extra": "mean: 886.2249307512359 usec\nrounds: 1343"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Joe Howarth",
            "username": "JoeHowarth",
            "email": "josephehowarth@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f9dfb0553351970c1eee00f13e9120c571a99926",
          "message": "Add Rust flags for macOS target configurations (#2)\n\nAdded target-specific Rust flags to prevent link errors on macOS.",
          "timestamp": "2026-01-05T04:54:01Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/f9dfb0553351970c1eee00f13e9120c571a99926"
        },
        "date": 1769997505816,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5233397.587556526,
            "unit": "iter/sec",
            "range": "stddev: 4.522233370375198e-8",
            "extra": "mean: 191.0804564854844 nsec\nrounds: 53408"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4961028.917309053,
            "unit": "iter/sec",
            "range": "stddev: 1.2984668818069124e-8",
            "extra": "mean: 201.5710887131742 nsec\nrounds: 49710"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3200292.480833371,
            "unit": "iter/sec",
            "range": "stddev: 3.869202374679132e-8",
            "extra": "mean: 312.4714400291079 nsec\nrounds: 190877"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 694604.7912824207,
            "unit": "iter/sec",
            "range": "stddev: 1.323929511484297e-7",
            "extra": "mean: 1.4396675815520377 usec\nrounds: 70938"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 72230.32345625105,
            "unit": "iter/sec",
            "range": "stddev: 0.000001557613103198638",
            "extra": "mean: 13.844600884359693 usec\nrounds: 76900"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 418967.48942812544,
            "unit": "iter/sec",
            "range": "stddev: 2.0320544192815338e-7",
            "extra": "mean: 2.386820040297067 usec\nrounds: 43682"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 418543.9297221685,
            "unit": "iter/sec",
            "range": "stddev: 1.7840100176968678e-7",
            "extra": "mean: 2.3892354636795408 usec\nrounds: 43154"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 364516.7201150002,
            "unit": "iter/sec",
            "range": "stddev: 4.1952453447018955e-7",
            "extra": "mean: 2.7433583833534803 usec\nrounds: 191976"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 214713.84670962894,
            "unit": "iter/sec",
            "range": "stddev: 5.792166523815484e-7",
            "extra": "mean: 4.65736148517875 usec\nrounds: 112284"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 40068.695268876516,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020849472130890293",
            "extra": "mean: 24.957139065537607 usec\nrounds: 41520"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3758151.978362738,
            "unit": "iter/sec",
            "range": "stddev: 8.670411961759713e-8",
            "extra": "mean: 266.0882278729953 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3529981.643095456,
            "unit": "iter/sec",
            "range": "stddev: 3.474602862232794e-8",
            "extra": "mean: 283.287592148312 nsec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2257139.8086649743,
            "unit": "iter/sec",
            "range": "stddev: 5.296684392672925e-8",
            "extra": "mean: 443.0385730476803 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 504932.95538049436,
            "unit": "iter/sec",
            "range": "stddev: 1.2592162340032052e-7",
            "extra": "mean: 1.9804609490114524 usec\nrounds: 52370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1485173.0452209709,
            "unit": "iter/sec",
            "range": "stddev: 7.431782731358541e-8",
            "extra": "mean: 673.3222119926049 nsec\nrounds: 153093"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1473422.4306211043,
            "unit": "iter/sec",
            "range": "stddev: 8.808641804808762e-8",
            "extra": "mean: 678.6919889487898 nsec\nrounds: 151241"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1259454.9781074033,
            "unit": "iter/sec",
            "range": "stddev: 1.0039298997735665e-7",
            "extra": "mean: 793.9942414636652 nsec\nrounds: 130311"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 443163.11969120573,
            "unit": "iter/sec",
            "range": "stddev: 1.4440394730808816e-7",
            "extra": "mean: 2.2565054616836995 usec\nrounds: 45041"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2775603.9752708897,
            "unit": "iter/sec",
            "range": "stddev: 4.0372363035849305e-8",
            "extra": "mean: 360.28194544681503 nsec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 660244.2475668882,
            "unit": "iter/sec",
            "range": "stddev: 1.4451036821571984e-7",
            "extra": "mean: 1.5145910073206654 usec\nrounds: 66543"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2990298.4417040064,
            "unit": "iter/sec",
            "range": "stddev: 4.092465090433575e-8",
            "extra": "mean: 334.4147814992523 nsec\nrounds: 194932"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2863321.0051011634,
            "unit": "iter/sec",
            "range": "stddev: 4.289859291269696e-8",
            "extra": "mean: 349.2448098618807 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1854755.5906958259,
            "unit": "iter/sec",
            "range": "stddev: 6.836744865565513e-8",
            "extra": "mean: 539.1545953635639 nsec\nrounds: 189430"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 502901.43980220746,
            "unit": "iter/sec",
            "range": "stddev: 1.4375366521875548e-7",
            "extra": "mean: 1.9884611990637702 usec\nrounds: 51290"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 34444.806200752406,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022375205839154563",
            "extra": "mean: 29.031953153452676 usec\nrounds: 37356"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1203547.167522521,
            "unit": "iter/sec",
            "range": "stddev: 9.575493763741716e-8",
            "extra": "mean: 830.8772825733505 nsec\nrounds: 124147"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1193374.6606758863,
            "unit": "iter/sec",
            "range": "stddev: 9.198605538005536e-8",
            "extra": "mean: 837.9598067162233 nsec\nrounds: 122926"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 900662.879607647,
            "unit": "iter/sec",
            "range": "stddev: 1.1317566356728602e-7",
            "extra": "mean: 1.1102933435378335 usec\nrounds: 93458"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 350763.7091231234,
            "unit": "iter/sec",
            "range": "stddev: 3.844915567863869e-7",
            "extra": "mean: 2.8509220708718894 usec\nrounds: 181819"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 33376.039362975476,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024328736278810546",
            "extra": "mean: 29.961613753048077 usec\nrounds: 36021"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2824751.6894759648,
            "unit": "iter/sec",
            "range": "stddev: 4.524069311196446e-8",
            "extra": "mean: 354.0134177902515 nsec\nrounds: 194970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2303842.7776078037,
            "unit": "iter/sec",
            "range": "stddev: 5.503981758580206e-8",
            "extra": "mean: 434.0573973708233 nsec\nrounds: 197239"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 793873.1311228132,
            "unit": "iter/sec",
            "range": "stddev: 9.67379220959595e-8",
            "extra": "mean: 1.2596471158882487 usec\nrounds: 80822"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107527.58857196504,
            "unit": "iter/sec",
            "range": "stddev: 9.183693686534919e-7",
            "extra": "mean: 9.29993886481263 usec\nrounds: 109446"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 445294.2631141971,
            "unit": "iter/sec",
            "range": "stddev: 1.6471432758340143e-7",
            "extra": "mean: 2.245706003500819 usec\nrounds: 45640"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 441408.00264489173,
            "unit": "iter/sec",
            "range": "stddev: 1.80220687393426e-7",
            "extra": "mean: 2.265477730372018 usec\nrounds: 45003"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 331766.46121954307,
            "unit": "iter/sec",
            "range": "stddev: 4.391690146893354e-7",
            "extra": "mean: 3.014168449469219 usec\nrounds: 173883"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 138422.65227458542,
            "unit": "iter/sec",
            "range": "stddev: 9.584240644710406e-7",
            "extra": "mean: 7.224251114740426 usec\nrounds: 144655"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2677806.337550908,
            "unit": "iter/sec",
            "range": "stddev: 5.232403963246107e-7",
            "extra": "mean: 373.44000048733506 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2778858.4471207717,
            "unit": "iter/sec",
            "range": "stddev: 1.6374882477355828e-7",
            "extra": "mean: 359.8599997189922 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1956679.1256528313,
            "unit": "iter/sec",
            "range": "stddev: 1.5182520809462654e-7",
            "extra": "mean: 511.0699996180302 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 500718.5303778851,
            "unit": "iter/sec",
            "range": "stddev: 1.7638826684065803e-7",
            "extra": "mean: 1.9971300028487347 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 51743.631009423385,
            "unit": "iter/sec",
            "range": "stddev: 0.000002273581121839975",
            "extra": "mean: 19.326049998653616 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 856384.3444213984,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014287146952179437",
            "extra": "mean: 1.1677000011900418 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 937101.730102603,
            "unit": "iter/sec",
            "range": "stddev: 4.164508844989672e-7",
            "extra": "mean: 1.067120001891908 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 768214.3626275448,
            "unit": "iter/sec",
            "range": "stddev: 4.562275707576227e-7",
            "extra": "mean: 1.3017199998444084 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 379715.5928345488,
            "unit": "iter/sec",
            "range": "stddev: 6.530761467100924e-7",
            "extra": "mean: 2.633550001291951 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 53037.089365723084,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025197809015623092",
            "extra": "mean: 18.854730000441577 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1624906.604759021,
            "unit": "iter/sec",
            "range": "stddev: 1.368183313153326e-7",
            "extra": "mean: 615.4199860294775 nsec\nrounds: 167477"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 373885.8478970998,
            "unit": "iter/sec",
            "range": "stddev: 3.329793361168008e-7",
            "extra": "mean: 2.674613135598591 usec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 44636.02880701932,
            "unit": "iter/sec",
            "range": "stddev: 0.000002216022087607573",
            "extra": "mean: 22.40342670992145 usec\nrounds: 45661"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4561.984151385596,
            "unit": "iter/sec",
            "range": "stddev: 0.000006095302749334368",
            "extra": "mean: 219.2028658618363 usec\nrounds: 4637"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 206570.16025660746,
            "unit": "iter/sec",
            "range": "stddev: 6.016802103766458e-7",
            "extra": "mean: 4.84097024835422 usec\nrounds: 107910"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 135815.71413037035,
            "unit": "iter/sec",
            "range": "stddev: 9.852847222000562e-7",
            "extra": "mean: 7.362918248473764 usec\nrounds: 141563"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31705.58736216799,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020023514021300303",
            "extra": "mean: 31.54018213184811 usec\nrounds: 32471"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3715.4665418311884,
            "unit": "iter/sec",
            "range": "stddev: 0.0000060965007709819475",
            "extra": "mean: 269.1452039040955 usec\nrounds: 3791"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 144024.78322907136,
            "unit": "iter/sec",
            "range": "stddev: 8.838018815714088e-7",
            "extra": "mean: 6.943249471235102 usec\nrounds: 149881"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17691.72310881564,
            "unit": "iter/sec",
            "range": "stddev: 0.000002718391375250616",
            "extra": "mean: 56.523606765115396 usec\nrounds: 18152"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1608.4307646748534,
            "unit": "iter/sec",
            "range": "stddev: 0.000009570049576922139",
            "extra": "mean: 621.7239945681786 usec\nrounds: 1657"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1255433.0988730213,
            "unit": "iter/sec",
            "range": "stddev: 8.484735207487955e-8",
            "extra": "mean: 796.5378648194819 nsec\nrounds: 127324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 105600.40558241673,
            "unit": "iter/sec",
            "range": "stddev: 9.395128248224488e-7",
            "extra": "mean: 9.469660599168263 usec\nrounds: 107678"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17117.467755532638,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025125437250444953",
            "extra": "mean: 58.41985592038192 usec\nrounds: 17414"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2366.272681673171,
            "unit": "iter/sec",
            "range": "stddev: 0.000006203157014694528",
            "extra": "mean: 422.6055634859921 usec\nrounds: 2410"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 721685.4097289352,
            "unit": "iter/sec",
            "range": "stddev: 1.2040323772282457e-7",
            "extra": "mean: 1.385645305446286 usec\nrounds: 74267"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 109085.60115851524,
            "unit": "iter/sec",
            "range": "stddev: 9.931529637139086e-7",
            "extra": "mean: 9.16711270213264 usec\nrounds: 111533"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21623.081751607057,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029425665401293387",
            "extra": "mean: 46.24687690160903 usec\nrounds: 22941"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4336.356743133474,
            "unit": "iter/sec",
            "range": "stddev: 0.00002487755950384834",
            "extra": "mean: 230.6083330398215 usec\nrounds: 4540"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2190253.694883552,
            "unit": "iter/sec",
            "range": "stddev: 5.542830415614585e-8",
            "extra": "mean: 456.56811461431397 nsec\nrounds: 188715"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 1002678.8803807439,
            "unit": "iter/sec",
            "range": "stddev: 8.963731352935511e-8",
            "extra": "mean: 997.3282768459982 nsec\nrounds: 102271"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 364886.97674706916,
            "unit": "iter/sec",
            "range": "stddev: 4.2322523009201e-7",
            "extra": "mean: 2.7405746538692606 usec\nrounds: 186916"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 125567.0494141221,
            "unit": "iter/sec",
            "range": "stddev: 9.80007112873851e-7",
            "extra": "mean: 7.963872725096727 usec\nrounds: 128800"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 893399.2445666494,
            "unit": "iter/sec",
            "range": "stddev: 1.1449508287929866e-7",
            "extra": "mean: 1.1193204002372499 usec\nrounds: 91744"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 725037.9974950543,
            "unit": "iter/sec",
            "range": "stddev: 1.2066806838433004e-7",
            "extra": "mean: 1.3792380584947328 usec\nrounds: 74488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 496179.04148897727,
            "unit": "iter/sec",
            "range": "stddev: 1.729009850348046e-7",
            "extra": "mean: 2.0154015312680817 usec\nrounds: 50873"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 198347.62266805337,
            "unit": "iter/sec",
            "range": "stddev: 8.814478150732914e-7",
            "extra": "mean: 5.041653570376086 usec\nrounds: 105297"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5510171.630845182,
            "unit": "iter/sec",
            "range": "stddev: 1.253777574520676e-8",
            "extra": "mean: 181.4825502715793 nsec\nrounds: 56045"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 416948.04032996454,
            "unit": "iter/sec",
            "range": "stddev: 3.4924168717855036e-7",
            "extra": "mean: 2.398380381422649 usec\nrounds: 44305"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54420.90115409921,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016338626637967399",
            "extra": "mean: 18.375292925936343 usec\nrounds: 55823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4059.8820232421813,
            "unit": "iter/sec",
            "range": "stddev: 0.0000065010595068032995",
            "extra": "mean: 246.3125761475724 usec\nrounds: 4117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4378997.470785885,
            "unit": "iter/sec",
            "range": "stddev: 1.4592858899162377e-8",
            "extra": "mean: 228.36277176942778 nsec\nrounds: 44661"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 408758.49334418983,
            "unit": "iter/sec",
            "range": "stddev: 1.7657347624638204e-7",
            "extra": "mean: 2.446432346441701 usec\nrounds: 42456"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2435162.014632894,
            "unit": "iter/sec",
            "range": "stddev: 4.998424229506576e-8",
            "extra": "mean: 410.6502951306996 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48910.9671822956,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019530589333725844",
            "extra": "mean: 20.445312321731638 usec\nrounds: 51162"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 118061.93146864782,
            "unit": "iter/sec",
            "range": "stddev: 9.926543390378608e-7",
            "extra": "mean: 8.47013078272023 usec\nrounds: 122325"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14488.316136166877,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032022072988687602",
            "extra": "mean: 69.02113334645709 usec\nrounds: 14796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1345.6015978084106,
            "unit": "iter/sec",
            "range": "stddev: 0.000010571780306998164",
            "extra": "mean: 743.1620188536532 usec\nrounds: 1379"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 52098.119006617766,
            "unit": "iter/sec",
            "range": "stddev: 0.000001848954520567709",
            "extra": "mean: 19.1945509563018 usec\nrounds: 53320"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49477.73555109605,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018552102424875989",
            "extra": "mean: 20.21111089385431 usec\nrounds: 52176"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 36293.11348957599,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023242234300547024",
            "extra": "mean: 27.553436557247075 usec\nrounds: 37924"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 69239.45483837319,
            "unit": "iter/sec",
            "range": "stddev: 0.0000030912751944583956",
            "extra": "mean: 14.442632489442856 usec\nrounds: 156937"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 246965.7301675541,
            "unit": "iter/sec",
            "range": "stddev: 4.935538588680405e-7",
            "extra": "mean: 4.049144791552857 usec\nrounds: 127649"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 32893.82668879515,
            "unit": "iter/sec",
            "range": "stddev: 0.000006216413404067779",
            "extra": "mean: 30.400841150556587 usec\nrounds: 50488"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 31701.699025676833,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018629350209270585",
            "extra": "mean: 31.54405065766503 usec\nrounds: 32157"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8628.141031678784,
            "unit": "iter/sec",
            "range": "stddev: 0.000039315890641822106",
            "extra": "mean: 115.89982086853176 usec\nrounds: 12315"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3051.485505752707,
            "unit": "iter/sec",
            "range": "stddev: 0.000007629851353895008",
            "extra": "mean: 327.7092413235405 usec\nrounds: 3083"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 16740.296178522283,
            "unit": "iter/sec",
            "range": "stddev: 0.000012409647715439051",
            "extra": "mean: 59.736099608739 usec\nrounds: 23010"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 27503.544893017886,
            "unit": "iter/sec",
            "range": "stddev: 0.000002224293935164436",
            "extra": "mean: 36.35894950595486 usec\nrounds: 28340"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 32002.994123839148,
            "unit": "iter/sec",
            "range": "stddev: 0.000004604626241245823",
            "extra": "mean: 31.247076324496042 usec\nrounds: 46905"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 23657.747807394582,
            "unit": "iter/sec",
            "range": "stddev: 0.000002027422198779312",
            "extra": "mean: 42.26945050481242 usec\nrounds: 24861"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28328.35922691801,
            "unit": "iter/sec",
            "range": "stddev: 0.000003985931256212676",
            "extra": "mean: 35.30031485373801 usec\nrounds: 38186"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 22688.587744907905,
            "unit": "iter/sec",
            "range": "stddev: 0.00000231047192451658",
            "extra": "mean: 44.07502182344664 usec\nrounds: 22957"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16470.258063434845,
            "unit": "iter/sec",
            "range": "stddev: 0.000007388616971343025",
            "extra": "mean: 60.71550282627762 usec\nrounds: 21228"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21009.443267465464,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022974793747950454",
            "extra": "mean: 47.59764393893137 usec\nrounds: 21457"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 34028.36429539354,
            "unit": "iter/sec",
            "range": "stddev: 0.000009310004208330757",
            "extra": "mean: 29.387248570609994 usec\nrounds: 55795"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47358.94869037667,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016482263001066762",
            "extra": "mean: 21.11533358854311 usec\nrounds: 48338"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4423.644892884638,
            "unit": "iter/sec",
            "range": "stddev: 0.00008266481455365964",
            "extra": "mean: 226.0579282953937 usec\nrounds: 6485"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 876.5924001770554,
            "unit": "iter/sec",
            "range": "stddev: 0.000014057522505782523",
            "extra": "mean: 1.1407810514875767 msec\nrounds: 874"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1035.446783370466,
            "unit": "iter/sec",
            "range": "stddev: 0.00015638657371746246",
            "extra": "mean: 965.7666777861015 usec\nrounds: 1229"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 271.6424756918518,
            "unit": "iter/sec",
            "range": "stddev: 0.00002357723081233249",
            "extra": "mean: 3.6813094029314795 msec\nrounds: 273"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 60104.41803630657,
            "unit": "iter/sec",
            "range": "stddev: 0.0000030001842330168513",
            "extra": "mean: 16.63771204632481 usec\nrounds: 96071"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25505.00714177866,
            "unit": "iter/sec",
            "range": "stddev: 0.000005860426160137369",
            "extra": "mean: 39.207987452861474 usec\nrounds: 36901"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 6013.1816944087095,
            "unit": "iter/sec",
            "range": "stddev: 0.00005494619075016853",
            "extra": "mean: 166.30131115609544 usec\nrounds: 8417"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1723.2789909549613,
            "unit": "iter/sec",
            "range": "stddev: 0.00009772000510960331",
            "extra": "mean: 580.2890914638531 usec\nrounds: 1968"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 30465.45738527033,
            "unit": "iter/sec",
            "range": "stddev: 0.000008524280088869576",
            "extra": "mean: 32.82405996252948 usec\nrounds: 48080"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1716.5134804160095,
            "unit": "iter/sec",
            "range": "stddev: 0.00009388322539370015",
            "extra": "mean: 582.5762578675716 usec\nrounds: 1970"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1139.41126417491,
            "unit": "iter/sec",
            "range": "stddev: 0.00014981130723947574",
            "extra": "mean: 877.6462296290683 usec\nrounds: 1350"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Joe Howarth",
            "username": "JoeHowarth",
            "email": "josephehowarth@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f9dfb0553351970c1eee00f13e9120c571a99926",
          "message": "Add Rust flags for macOS target configurations (#2)\n\nAdded target-specific Rust flags to prevent link errors on macOS.",
          "timestamp": "2026-01-05T04:54:01Z",
          "url": "https://github.com/horizonanalytic/astrora/commit/f9dfb0553351970c1eee00f13e9120c571a99926"
        },
        "date": 1770602423312,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5371980.5350632,
            "unit": "iter/sec",
            "range": "stddev: 1.8375415837037718e-8",
            "extra": "mean: 186.1510840318456 nsec\nrounds: 55146"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5063448.238596602,
            "unit": "iter/sec",
            "range": "stddev: 1.286787822005462e-8",
            "extra": "mean: 197.49387233334053 nsec\nrounds: 51478"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3194845.996908914,
            "unit": "iter/sec",
            "range": "stddev: 3.695894664631945e-8",
            "extra": "mean: 313.00413258343775 nsec\nrounds: 192345"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 697966.3657254146,
            "unit": "iter/sec",
            "range": "stddev: 9.898882859180143e-8",
            "extra": "mean: 1.4327337950743313 usec\nrounds: 70892"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 75726.22022050236,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016479531264654996",
            "extra": "mean: 13.205465650974837 usec\nrounds: 79158"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 426050.3386982771,
            "unit": "iter/sec",
            "range": "stddev: 1.7749419669690804e-7",
            "extra": "mean: 2.3471404882702718 usec\nrounds: 43951"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 415849.83073049376,
            "unit": "iter/sec",
            "range": "stddev: 4.0748967667185213e-7",
            "extra": "mean: 2.4047142167723785 usec\nrounds: 42893"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 365777.62949593633,
            "unit": "iter/sec",
            "range": "stddev: 3.9116617749183823e-7",
            "extra": "mean: 2.733901472810299 usec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 216166.5944413178,
            "unit": "iter/sec",
            "range": "stddev: 5.151771084477243e-7",
            "extra": "mean: 4.626061684436017 usec\nrounds: 111657"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39403.62114515909,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022813627379386765",
            "extra": "mean: 25.37837820326456 usec\nrounds: 41520"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3649748.0595392236,
            "unit": "iter/sec",
            "range": "stddev: 3.238118803168238e-8",
            "extra": "mean: 273.99151494497 nsec\nrounds: 198020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3408802.100128725,
            "unit": "iter/sec",
            "range": "stddev: 3.4266005774897255e-8",
            "extra": "mean: 293.35818584514095 nsec\nrounds: 196117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2248025.8087394526,
            "unit": "iter/sec",
            "range": "stddev: 5.5498256090347294e-8",
            "extra": "mean: 444.83475061202165 nsec\nrounds: 191976"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 506637.8365479017,
            "unit": "iter/sec",
            "range": "stddev: 1.4492644101188718e-7",
            "extra": "mean: 1.9737965226081942 usec\nrounds: 51878"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1501670.891735927,
            "unit": "iter/sec",
            "range": "stddev: 7.965005761356022e-8",
            "extra": "mean: 665.9248744203467 nsec\nrounds: 153093"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1472608.782678922,
            "unit": "iter/sec",
            "range": "stddev: 7.740066572451436e-8",
            "extra": "mean: 679.0669808316605 nsec\nrounds: 150558"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1270351.6593515903,
            "unit": "iter/sec",
            "range": "stddev: 8.598484183389317e-8",
            "extra": "mean: 787.1836059240933 nsec\nrounds: 129803"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 429856.67489099805,
            "unit": "iter/sec",
            "range": "stddev: 2.0330803111845625e-7",
            "extra": "mean: 2.326356803121843 usec\nrounds: 44362"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2725320.10930343,
            "unit": "iter/sec",
            "range": "stddev: 4.71295789202314e-8",
            "extra": "mean: 366.9293733922183 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 639962.661091101,
            "unit": "iter/sec",
            "range": "stddev: 1.2452381467255324e-7",
            "extra": "mean: 1.5625911647642754 usec\nrounds: 66099"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2960662.2938737385,
            "unit": "iter/sec",
            "range": "stddev: 3.9730731037990987e-8",
            "extra": "mean: 337.76226423027714 nsec\nrounds: 191976"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2830582.29870968,
            "unit": "iter/sec",
            "range": "stddev: 4.2749908287737475e-8",
            "extra": "mean: 353.28419896359577 nsec\nrounds: 194175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1798244.9143123566,
            "unit": "iter/sec",
            "range": "stddev: 8.393539334436372e-8",
            "extra": "mean: 556.0977773611517 nsec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 497250.95474851667,
            "unit": "iter/sec",
            "range": "stddev: 1.4995293687258094e-7",
            "extra": "mean: 2.011056973245547 usec\nrounds: 50873"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47530.95723746426,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022515510152603547",
            "extra": "mean: 21.03891985604263 usec\nrounds: 49461"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1210855.9429534124,
            "unit": "iter/sec",
            "range": "stddev: 9.867276159357866e-8",
            "extra": "mean: 825.8620737003968 nsec\nrounds: 125235"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1197979.5727334355,
            "unit": "iter/sec",
            "range": "stddev: 8.321144490758589e-8",
            "extra": "mean: 834.738774149791 nsec\nrounds: 123229"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 874003.3479208163,
            "unit": "iter/sec",
            "range": "stddev: 1.0907548751169662e-7",
            "extra": "mean: 1.144160376935574 usec\nrounds: 90253"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 344595.9081248399,
            "unit": "iter/sec",
            "range": "stddev: 4.2078540383574443e-7",
            "extra": "mean: 2.90194972262329 usec\nrounds: 179534"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 44087.76761043117,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021887550225306875",
            "extra": "mean: 22.68202846731119 usec\nrounds: 45807"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2800420.1651796647,
            "unit": "iter/sec",
            "range": "stddev: 4.360072987585699e-8",
            "extra": "mean: 357.08927268641196 nsec\nrounds: 190840"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2258116.2388165374,
            "unit": "iter/sec",
            "range": "stddev: 5.689729379779859e-8",
            "extra": "mean: 442.8469991093735 nsec\nrounds: 194970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 784038.2297334005,
            "unit": "iter/sec",
            "range": "stddev: 1.1696388136971562e-7",
            "extra": "mean: 1.275448010156352 usec\nrounds: 79981"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107328.74680488122,
            "unit": "iter/sec",
            "range": "stddev: 8.467139537263151e-7",
            "extra": "mean: 9.31716832413924 usec\nrounds: 109806"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 435884.89555539796,
            "unit": "iter/sec",
            "range": "stddev: 1.623172986187877e-7",
            "extra": "mean: 2.294183648474033 usec\nrounds: 45164"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 433659.6959078685,
            "unit": "iter/sec",
            "range": "stddev: 1.654625615236138e-7",
            "extra": "mean: 2.3059555901465814 usec\nrounds: 44659"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 326772.6803841218,
            "unit": "iter/sec",
            "range": "stddev: 4.481754219374126e-7",
            "extra": "mean: 3.060231347444647 usec\nrounds: 170620"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 126479.24550179222,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010136350346774433",
            "extra": "mean: 7.906435526497743 usec\nrounds: 132031"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2654843.7473604213,
            "unit": "iter/sec",
            "range": "stddev: 6.354662979948489e-7",
            "extra": "mean: 376.6700021401448 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2887419.5168598755,
            "unit": "iter/sec",
            "range": "stddev: 1.5811925324594259e-7",
            "extra": "mean: 346.32999955874766 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2045868.3668425195,
            "unit": "iter/sec",
            "range": "stddev: 1.4567053573496937e-7",
            "extra": "mean: 488.79000047463705 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 480455.08686696144,
            "unit": "iter/sec",
            "range": "stddev: 1.8527656205324843e-7",
            "extra": "mean: 2.0813600008295907 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 52184.903634143775,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012002786152988334",
            "extra": "mean: 19.162630001403613 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 837766.5141499458,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013538270140980413",
            "extra": "mean: 1.1936500004594564 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 751715.7921938528,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020240295606693658",
            "extra": "mean: 1.3302899984068972 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 750874.7681519832,
            "unit": "iter/sec",
            "range": "stddev: 4.260106610123161e-7",
            "extra": "mean: 1.3317800016920955 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 376446.495716826,
            "unit": "iter/sec",
            "range": "stddev: 4.4975252784966396e-7",
            "extra": "mean: 2.6564199995959825 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 53871.386368515734,
            "unit": "iter/sec",
            "range": "stddev: 0.000002492446246207943",
            "extra": "mean: 18.562730002145145 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1615673.2482519124,
            "unit": "iter/sec",
            "range": "stddev: 7.523840111038132e-8",
            "extra": "mean: 618.9370289332584 nsec\nrounds: 167758"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 372771.59934323974,
            "unit": "iter/sec",
            "range": "stddev: 4.2056283066618035e-7",
            "extra": "mean: 2.682607799955335 usec\nrounds: 191976"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 45244.82018515445,
            "unit": "iter/sec",
            "range": "stddev: 0.000001382396217083946",
            "extra": "mean: 22.10197755030787 usec\nrounds: 46103"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4608.722794841062,
            "unit": "iter/sec",
            "range": "stddev: 0.000005743379553933533",
            "extra": "mean: 216.97985418419734 usec\nrounds: 4684"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 210310.58030466366,
            "unit": "iter/sec",
            "range": "stddev: 5.604493281399013e-7",
            "extra": "mean: 4.754872524964569 usec\nrounds: 108484"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 136383.07792689616,
            "unit": "iter/sec",
            "range": "stddev: 9.345109710102957e-7",
            "extra": "mean: 7.332287958305343 usec\nrounds: 140371"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31689.235328734016,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020283263450341124",
            "extra": "mean: 31.55645725200748 usec\nrounds: 32481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3716.9717782676794,
            "unit": "iter/sec",
            "range": "stddev: 0.000005509480473644315",
            "extra": "mean: 269.0362100263395 usec\nrounds: 3790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 138209.28142577814,
            "unit": "iter/sec",
            "range": "stddev: 8.692877334967235e-7",
            "extra": "mean: 7.235404089247256 usec\nrounds: 145943"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16528.57959566616,
            "unit": "iter/sec",
            "range": "stddev: 0.000002560012123665316",
            "extra": "mean: 60.50126656147771 usec\nrounds: 17103"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1541.8453457616506,
            "unit": "iter/sec",
            "range": "stddev: 0.00006848150894403622",
            "extra": "mean: 648.5734790125878 usec\nrounds: 1620"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1249228.7198831816,
            "unit": "iter/sec",
            "range": "stddev: 8.428881303128445e-8",
            "extra": "mean: 800.4939240378022 nsec\nrounds: 127486"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 105063.86454892036,
            "unit": "iter/sec",
            "range": "stddev: 8.75177321309544e-7",
            "extra": "mean: 9.518020342135568 usec\nrounds: 107216"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 16974.74869726938,
            "unit": "iter/sec",
            "range": "stddev: 0.000003889911228151923",
            "extra": "mean: 58.91103413865937 usec\nrounds: 17341"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2364.498628957605,
            "unit": "iter/sec",
            "range": "stddev: 0.000005962113594000102",
            "extra": "mean: 422.92263897012805 usec\nrounds: 2407"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 713840.9084581651,
            "unit": "iter/sec",
            "range": "stddev: 2.3276735671016092e-7",
            "extra": "mean: 1.4008723626667814 usec\nrounds: 73180"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 106342.99580729415,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010203855567236337",
            "extra": "mean: 9.40353421876619 usec\nrounds: 109326"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21811.556141937985,
            "unit": "iter/sec",
            "range": "stddev: 0.000006539201101545364",
            "extra": "mean: 45.847256082625776 usec\nrounds: 23180"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4486.946711338106,
            "unit": "iter/sec",
            "range": "stddev: 0.0000189478837012997",
            "extra": "mean: 222.8687043403237 usec\nrounds: 4654"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2108427.3958167722,
            "unit": "iter/sec",
            "range": "stddev: 5.7004841436909393e-8",
            "extra": "mean: 474.287140256305 nsec\nrounds: 198060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 928777.9715569447,
            "unit": "iter/sec",
            "range": "stddev: 9.650471472129002e-8",
            "extra": "mean: 1.0766835892152888 usec\nrounds: 97381"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 361021.296022712,
            "unit": "iter/sec",
            "range": "stddev: 3.288715852915017e-7",
            "extra": "mean: 2.7699197000752265 usec\nrounds: 184502"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 122006.76212779398,
            "unit": "iter/sec",
            "range": "stddev: 9.692942378767595e-7",
            "extra": "mean: 8.196267014713221 usec\nrounds: 126199"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 904295.7001029054,
            "unit": "iter/sec",
            "range": "stddev: 9.734851249970984e-8",
            "extra": "mean: 1.1058329702177896 usec\nrounds: 93633"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 723672.2614410595,
            "unit": "iter/sec",
            "range": "stddev: 1.3756476442470927e-7",
            "extra": "mean: 1.3818409980350528 usec\nrounds: 74206"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 452261.5757553938,
            "unit": "iter/sec",
            "range": "stddev: 2.725699945623968e-7",
            "extra": "mean: 2.2111097948786522 usec\nrounds: 46555"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 201321.23890629777,
            "unit": "iter/sec",
            "range": "stddev: 9.067041324823447e-7",
            "extra": "mean: 4.967185804302726 usec\nrounds: 198413"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5577678.879627535,
            "unit": "iter/sec",
            "range": "stddev: 1.1992092081043942e-8",
            "extra": "mean: 179.28604740088366 nsec\nrounds: 57003"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 425765.82348823216,
            "unit": "iter/sec",
            "range": "stddev: 1.708380441484743e-7",
            "extra": "mean: 2.3487089494575977 usec\nrounds: 43511"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 56079.02533523807,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016650331705941188",
            "extra": "mean: 17.831978962224145 usec\nrounds: 58371"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4260.568760263449,
            "unit": "iter/sec",
            "range": "stddev: 0.000006537725912008552",
            "extra": "mean: 234.7104474234951 usec\nrounds: 4327"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4513365.202711999,
            "unit": "iter/sec",
            "range": "stddev: 1.3754648472224437e-8",
            "extra": "mean: 221.5641666663472 nsec\nrounds: 45744"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 414304.2915566829,
            "unit": "iter/sec",
            "range": "stddev: 1.8452551146767624e-7",
            "extra": "mean: 2.413684869743103 usec\nrounds: 42187"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2465807.845617756,
            "unit": "iter/sec",
            "range": "stddev: 4.877917847239211e-8",
            "extra": "mean: 405.5466048488899 nsec\nrounds: 195734"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48489.44136908258,
            "unit": "iter/sec",
            "range": "stddev: 0.0000033546094808644955",
            "extra": "mean: 20.623046415164755 usec\nrounds: 50156"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 112932.45235379974,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013974178643502811",
            "extra": "mean: 8.854850657693646 usec\nrounds: 121419"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13829.060467249607,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032894681038200618",
            "extra": "mean: 72.31149233660737 usec\nrounds: 14354"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1311.203557954259,
            "unit": "iter/sec",
            "range": "stddev: 0.000014502689197421202",
            "extra": "mean: 762.6580891529924 usec\nrounds: 1346"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51324.48065877327,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017832144364457909",
            "extra": "mean: 19.48387956710991 usec\nrounds: 53291"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49203.64780003237,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019098228429992473",
            "extra": "mean: 20.323696406902215 usec\nrounds: 51905"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35306.13844247876,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025575343269459358",
            "extra": "mean: 28.323686591475116 usec\nrounds: 36872"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_small",
            "value": 69513.0463171199,
            "unit": "iter/sec",
            "range": "stddev: 0.0000027688026899386086",
            "extra": "mean: 14.385788754502 usec\nrounds: 119818"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_small",
            "value": 248148.02737445684,
            "unit": "iter/sec",
            "range": "stddev: 5.846340760257997e-7",
            "extra": "mean: 4.029852707597768 usec\nrounds: 128469"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_medium",
            "value": 33129.198519423575,
            "unit": "iter/sec",
            "range": "stddev: 0.000005455308508874359",
            "extra": "mean: 30.184853382846015 usec\nrounds: 48289"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_medium",
            "value": 32114.352432136875,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023624130758924713",
            "extra": "mean: 31.138725344475535 usec\nrounds: 32954"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_eccentric_large",
            "value": 8606.988504823526,
            "unit": "iter/sec",
            "range": "stddev: 0.00003934913919711454",
            "extra": "mean: 116.18465615930359 usec\nrounds: 12468"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_eccentric_large",
            "value": 3128.5069140971264,
            "unit": "iter/sec",
            "range": "stddev: 0.000007096231813434823",
            "extra": "mean: 319.641294540209 usec\nrounds: 3205"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_mean_to_true_medium",
            "value": 17059.19952306948,
            "unit": "iter/sec",
            "range": "stddev: 0.000011560563514138451",
            "extra": "mean: 58.61939762458848 usec\nrounds: 23658"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_mean_to_true_medium",
            "value": 28121.175571577594,
            "unit": "iter/sec",
            "range": "stddev: 0.000002046132301089711",
            "extra": "mean: 35.560391046052565 usec\nrounds: 28948"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_batch_variable_eccentricities",
            "value": 30967.849581300685,
            "unit": "iter/sec",
            "range": "stddev: 0.000007888000158834354",
            "extra": "mean: 32.29155441919448 usec\nrounds: 47327"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchEllipticalBenchmark::test_benchmark_sequential_variable_eccentricities",
            "value": 24308.94138974534,
            "unit": "iter/sec",
            "range": "stddev: 0.000004418621097254078",
            "extra": "mean: 41.13712662213449 usec\nrounds: 25430"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_hyperbolic_medium",
            "value": 28023.161700190292,
            "unit": "iter/sec",
            "range": "stddev: 0.000004367782149329724",
            "extra": "mean: 35.68476714721342 usec\nrounds: 39467"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_hyperbolic_medium",
            "value": 23168.497927487955,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023252468988697723",
            "extra": "mean: 43.1620557849615 usec\nrounds: 23519"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_batch_mean_to_true_hyperbolic",
            "value": 16566.391351678776,
            "unit": "iter/sec",
            "range": "stddev: 0.00000578914648491832",
            "extra": "mean: 60.36317619036953 usec\nrounds: 21210"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchHyperbolicBenchmark::test_benchmark_sequential_mean_to_true_hyperbolic",
            "value": 21265.052405092807,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022165446957112865",
            "extra": "mean: 47.02551307893829 usec\nrounds: 21638"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_batch_mean_to_true_parabolic",
            "value": 32321.154532210934,
            "unit": "iter/sec",
            "range": "stddev: 0.0000072853937285156015",
            "extra": "mean: 30.939488841693766 usec\nrounds: 57894"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchParabolicBenchmark::test_benchmark_sequential_mean_to_true_parabolic",
            "value": 47233.275386779584,
            "unit": "iter/sec",
            "range": "stddev: 0.000001524260751818191",
            "extra": "mean: 21.17151503492591 usec\nrounds: 47622"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_constellation_analysis",
            "value": 4340.254584899323,
            "unit": "iter/sec",
            "range": "stddev: 0.00008722026270745298",
            "extra": "mean: 230.40123118104972 usec\nrounds: 6350"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_constellation_analysis",
            "value": 896.1788954950407,
            "unit": "iter/sec",
            "range": "stddev: 0.000011488695576242117",
            "extra": "mean: 1.1158486380641774 msec\nrounds: 909"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_batch_propagation_grid",
            "value": 1054.6505523670903,
            "unit": "iter/sec",
            "range": "stddev: 0.00015239627872613623",
            "extra": "mean: 948.1813646762608 usec\nrounds: 1223"
          },
          {
            "name": "tests/benchmark_batch_anomaly.py::TestBatchLargeScaleBenchmark::test_benchmark_sequential_propagation_grid",
            "value": 276.958038110262,
            "unit": "iter/sec",
            "range": "stddev: 0.000017976161066094993",
            "extra": "mean: 3.6106552704633255 msec\nrounds: 281"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_small",
            "value": 59942.56721008837,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032406841189822536",
            "extra": "mean: 16.682635505002185 usec\nrounds: 97277"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_medium",
            "value": 25833.568181692503,
            "unit": "iter/sec",
            "range": "stddev: 0.000005796407629370162",
            "extra": "mean: 38.709325516583924 usec\nrounds: 36588"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_large",
            "value": 5891.611587617885,
            "unit": "iter/sec",
            "range": "stddev: 0.00006377328464478188",
            "extra": "mean: 169.73284561080905 usec\nrounds: 8498"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_propagation_very_large",
            "value": 1701.6883667614675,
            "unit": "iter/sec",
            "range": "stddev: 0.0001113420799038189",
            "extra": "mean: 587.6516638020679 usec\nrounds: 1978"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_small",
            "value": 32486.374611551408,
            "unit": "iter/sec",
            "range": "stddev: 0.000005223486287693795",
            "extra": "mean: 30.782135955682264 usec\nrounds: 48810"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_anomaly_conversion_large",
            "value": 1695.9559523546548,
            "unit": "iter/sec",
            "range": "stddev: 0.00010113379967730937",
            "extra": "mean: 589.6379552851041 usec\nrounds: 1968"
          },
          {
            "name": "tests/benchmark_parallel_batch.py::TestParallelBatchPerformance::test_batch_mean_to_true_large",
            "value": 1155.4452569722096,
            "unit": "iter/sec",
            "range": "stddev: 0.00014110057994314146",
            "extra": "mean: 865.4672248345657 usec\nrounds: 1361"
          }
        ]
      }
    ]
  }
}