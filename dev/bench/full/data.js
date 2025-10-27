window.BENCHMARK_DATA = {
  "lastUpdate": 1761528346457,
  "repoUrl": "https://github.com/cachemcclure/astrora",
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
      }
    ]
  }
}