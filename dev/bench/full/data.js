window.BENCHMARK_DATA = {
  "lastUpdate": 1762133035014,
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
      }
    ]
  }
}