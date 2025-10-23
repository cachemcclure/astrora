window.BENCHMARK_DATA = {
  "lastUpdate": 1761260824017,
  "repoUrl": "https://github.com/cachemcclure/astrora",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "cache.mcclure@gmail.com",
            "name": "Cache McClure",
            "username": "cachemcclure"
          },
          "committer": {
            "email": "cache.mcclure@gmail.com",
            "name": "Cache McClure",
            "username": "cachemcclure"
          },
          "distinct": true,
          "id": "a23158875d9274e81a65ee1543ddf381eb37b780",
          "message": "Fixed failing test",
          "timestamp": "2025-10-23T18:00:48-05:00",
          "tree_id": "9e6ff893099dcfc13d8b34629d457852a6c59cc2",
          "url": "https://github.com/cachemcclure/astrora/commit/a23158875d9274e81a65ee1543ddf381eb37b780"
        },
        "date": 1761260822604,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5227478.234208472,
            "unit": "iter/sec",
            "range": "stddev: 1.4629685215201164e-8",
            "extra": "mean: 191.29682711180476 nsec\nrounds: 54307"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4757382.109978429,
            "unit": "iter/sec",
            "range": "stddev: 2.6002832450946366e-8",
            "extra": "mean: 210.19963855806245 nsec\nrounds: 50437"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3156589.173034482,
            "unit": "iter/sec",
            "range": "stddev: 1.064140767520115e-7",
            "extra": "mean: 316.7976398520671 nsec\nrounds: 191608"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 711105.0246745896,
            "unit": "iter/sec",
            "range": "stddev: 1.077460476533301e-7",
            "extra": "mean: 1.4062620362689864 usec\nrounds: 72485"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 70043.17069520289,
            "unit": "iter/sec",
            "range": "stddev: 0.000001572842570284725",
            "extra": "mean: 14.276909369959288 usec\nrounds: 74600"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 404433.37303196517,
            "unit": "iter/sec",
            "range": "stddev: 4.875134476874837e-7",
            "extra": "mean: 2.4725951582659462 usec\nrounds: 42712"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 397695.3088301522,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012631131354135263",
            "extra": "mean: 2.514487794541923 usec\nrounds: 42948"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 347901.2754951783,
            "unit": "iter/sec",
            "range": "stddev: 6.640901635150815e-7",
            "extra": "mean: 2.874378654049687 usec\nrounds: 186916"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 201411.27983639692,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011943418319036833",
            "extra": "mean: 4.964965223458605 usec\nrounds: 110534"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 38742.620777774435,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028612430969954474",
            "extra": "mean: 25.81136691128733 usec\nrounds: 40890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3698592.852608512,
            "unit": "iter/sec",
            "range": "stddev: 3.5728239638578656e-8",
            "extra": "mean: 270.37309589098436 nsec\nrounds: 191205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3483383.280477373,
            "unit": "iter/sec",
            "range": "stddev: 3.9421309270104045e-8",
            "extra": "mean: 287.07722334328656 nsec\nrounds: 199641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2230583.9423314277,
            "unit": "iter/sec",
            "range": "stddev: 6.033026073574619e-8",
            "extra": "mean: 448.3130991047889 nsec\nrounds: 190840"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 513449.6150727455,
            "unit": "iter/sec",
            "range": "stddev: 1.4368627767437397e-7",
            "extra": "mean: 1.9476107696727751 usec\nrounds: 52425"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1560573.888728355,
            "unit": "iter/sec",
            "range": "stddev: 9.588122404516308e-8",
            "extra": "mean: 640.7899089064562 nsec\nrounds: 160488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1543530.7872186243,
            "unit": "iter/sec",
            "range": "stddev: 8.5083975637294e-8",
            "extra": "mean: 647.8652763395773 nsec\nrounds: 158932"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1313707.3804753472,
            "unit": "iter/sec",
            "range": "stddev: 9.070943489080559e-8",
            "extra": "mean: 761.2045230636982 nsec\nrounds: 135815"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 447835.6611269942,
            "unit": "iter/sec",
            "range": "stddev: 1.8636187477152084e-7",
            "extra": "mean: 2.232961969762468 usec\nrounds: 45640"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2749934.1090942537,
            "unit": "iter/sec",
            "range": "stddev: 4.852508019162159e-8",
            "extra": "mean: 363.6450766923025 nsec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 521738.8464747918,
            "unit": "iter/sec",
            "range": "stddev: 1.938850994750622e-7",
            "extra": "mean: 1.9166677098257887 usec\nrounds: 53778"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3042176.174893995,
            "unit": "iter/sec",
            "range": "stddev: 7.048654376751237e-8",
            "extra": "mean: 328.7120608768968 nsec\nrounds: 199601"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2965138.06271615,
            "unit": "iter/sec",
            "range": "stddev: 1.0307518996567825e-7",
            "extra": "mean: 337.2524242881196 nsec\nrounds: 191242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1812218.7319060746,
            "unit": "iter/sec",
            "range": "stddev: 7.77947207463958e-8",
            "extra": "mean: 551.8097690935143 nsec\nrounds: 186568"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 499451.28904054494,
            "unit": "iter/sec",
            "range": "stddev: 1.8789018769737044e-7",
            "extra": "mean: 2.0021972551537837 usec\nrounds: 51369"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36513.849185587336,
            "unit": "iter/sec",
            "range": "stddev: 0.000003128832068895581",
            "extra": "mean: 27.386868881375502 usec\nrounds: 48643"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1216105.6866691818,
            "unit": "iter/sec",
            "range": "stddev: 1.0527914853071431e-7",
            "extra": "mean: 822.2969524457101 nsec\nrounds: 126199"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1186185.7466224073,
            "unit": "iter/sec",
            "range": "stddev: 1.1418895940221853e-7",
            "extra": "mean: 843.038287087348 nsec\nrounds: 123381"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 890568.7538230328,
            "unit": "iter/sec",
            "range": "stddev: 1.2851443120230118e-7",
            "extra": "mean: 1.1228779313300643 usec\nrounds: 92842"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 357411.8221863949,
            "unit": "iter/sec",
            "range": "stddev: 4.12311990818014e-7",
            "extra": "mean: 2.797892900919453 usec\nrounds: 184502"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 35669.67250698387,
            "unit": "iter/sec",
            "range": "stddev: 0.000003404232438662893",
            "extra": "mean: 28.035020501077124 usec\nrounds: 45412"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2822652.7609836534,
            "unit": "iter/sec",
            "range": "stddev: 4.625961420464359e-8",
            "extra": "mean: 354.27666265673605 nsec\nrounds: 198413"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2305065.215238017,
            "unit": "iter/sec",
            "range": "stddev: 7.646946589089793e-8",
            "extra": "mean: 433.8272051433966 nsec\nrounds: 198808"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 789278.5798585721,
            "unit": "iter/sec",
            "range": "stddev: 1.1013325877869232e-7",
            "extra": "mean: 1.2669797781401684 usec\nrounds: 80952"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107317.5337937723,
            "unit": "iter/sec",
            "range": "stddev: 9.554171441369156e-7",
            "extra": "mean: 9.318141823140095 usec\nrounds: 109566"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 363667.3982582219,
            "unit": "iter/sec",
            "range": "stddev: 4.63670989573815e-7",
            "extra": "mean: 2.7497653207009507 usec\nrounds: 188715"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 357454.7140765416,
            "unit": "iter/sec",
            "range": "stddev: 4.6743567055225153e-7",
            "extra": "mean: 2.797557174713523 usec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 292768.98573734757,
            "unit": "iter/sec",
            "range": "stddev: 5.028685081911268e-7",
            "extra": "mean: 3.415662343746793 usec\nrounds: 153563"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 128836.48696946693,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010661976379948017",
            "extra": "mean: 7.761776368809177 usec\nrounds: 135245"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2912989.021498291,
            "unit": "iter/sec",
            "range": "stddev: 5.263309110422944e-7",
            "extra": "mean: 343.289999591434 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 3000570.120103167,
            "unit": "iter/sec",
            "range": "stddev: 1.681020520192887e-7",
            "extra": "mean: 333.26999869132123 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2079304.6815816897,
            "unit": "iter/sec",
            "range": "stddev: 1.5875584538515953e-7",
            "extra": "mean: 480.9299997532434 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 487265.32034553395,
            "unit": "iter/sec",
            "range": "stddev: 1.6448916252615658e-7",
            "extra": "mean: 2.0522700020819684 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 44600.34744540934,
            "unit": "iter/sec",
            "range": "stddev: 0.000004395704737805187",
            "extra": "mean: 22.42134999562495 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 869149.5377533984,
            "unit": "iter/sec",
            "range": "stddev: 0.000001404817463418443",
            "extra": "mean: 1.1505499992381374 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 794066.7338404872,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022180501933149447",
            "extra": "mean: 1.2593399992510967 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 780256.3936383739,
            "unit": "iter/sec",
            "range": "stddev: 5.223457707626528e-7",
            "extra": "mean: 1.2816299977203016 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 446877.44375762966,
            "unit": "iter/sec",
            "range": "stddev: 5.113981886308618e-7",
            "extra": "mean: 2.2377500005177353 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 43834.84782433933,
            "unit": "iter/sec",
            "range": "stddev: 0.0000058898805953473895",
            "extra": "mean: 22.812900001554226 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1629814.5537540612,
            "unit": "iter/sec",
            "range": "stddev: 8.597544825225053e-8",
            "extra": "mean: 613.5667384345388 nsec\nrounds: 168606"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 359405.61236481817,
            "unit": "iter/sec",
            "range": "stddev: 3.65173587830386e-7",
            "extra": "mean: 2.7823716870201243 usec\nrounds: 183824"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43284.97326339993,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017281247665356436",
            "extra": "mean: 23.102705733806257 usec\nrounds: 43933"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4390.898797784141,
            "unit": "iter/sec",
            "range": "stddev: 0.000007214899240428916",
            "extra": "mean: 227.74380509627056 usec\nrounds: 4474"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 205460.07605459637,
            "unit": "iter/sec",
            "range": "stddev: 7.125101551333226e-7",
            "extra": "mean: 4.8671256197446 usec\nrounds: 107101"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 131868.88905724185,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012183840220696089",
            "extra": "mean: 7.583289789951278 usec\nrounds: 137855"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31853.502621460895,
            "unit": "iter/sec",
            "range": "stddev: 0.000002191293600337817",
            "extra": "mean: 31.393721810871202 usec\nrounds: 32823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3708.651424921726,
            "unit": "iter/sec",
            "range": "stddev: 0.000006491878865439093",
            "extra": "mean: 269.63979231914624 usec\nrounds: 3828"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 135555.99474497896,
            "unit": "iter/sec",
            "range": "stddev: 9.754745438099461e-7",
            "extra": "mean: 7.377025279341549 usec\nrounds: 140391"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16537.59589361787,
            "unit": "iter/sec",
            "range": "stddev: 0.000008386174744126945",
            "extra": "mean: 60.46828126849542 usec\nrounds: 17062"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1564.5171279630558,
            "unit": "iter/sec",
            "range": "stddev: 0.000016560839281260715",
            "extra": "mean: 639.1748496240265 usec\nrounds: 1596"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1245048.372339132,
            "unit": "iter/sec",
            "range": "stddev: 9.676252163372239e-8",
            "extra": "mean: 803.1816451606867 nsec\nrounds: 128304"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 103799.8751073102,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010989670819622147",
            "extra": "mean: 9.63392295960069 usec\nrounds: 106191"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17035.63018585695,
            "unit": "iter/sec",
            "range": "stddev: 0.000002777885311265761",
            "extra": "mean: 58.700499429143754 usec\nrounds: 17518"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2353.934842356606,
            "unit": "iter/sec",
            "range": "stddev: 0.000007997847412697849",
            "extra": "mean: 424.8205948635627 usec\nrounds: 2414"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 726569.7881590106,
            "unit": "iter/sec",
            "range": "stddev: 1.301140436357447e-7",
            "extra": "mean: 1.3763302800324755 usec\nrounds: 74488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 102980.72807808506,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011408480747646665",
            "extra": "mean: 9.71055476750709 usec\nrounds: 106304"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 14921.14210494374,
            "unit": "iter/sec",
            "range": "stddev: 0.000012230154933880566",
            "extra": "mean: 67.0189984765761 usec\nrounds: 15754"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 2297.046307762035,
            "unit": "iter/sec",
            "range": "stddev: 0.000035357586155923875",
            "extra": "mean: 435.3416805838274 usec\nrounds: 2395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2150418.3744346825,
            "unit": "iter/sec",
            "range": "stddev: 6.693776265948859e-8",
            "extra": "mean: 465.0257884179792 nsec\nrounds: 185529"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 996813.4485702335,
            "unit": "iter/sec",
            "range": "stddev: 1.0070509199096255e-7",
            "extra": "mean: 1.0031967379998277 usec\nrounds: 101441"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 362488.4621512363,
            "unit": "iter/sec",
            "range": "stddev: 3.8460204229934016e-7",
            "extra": "mean: 2.7587084953418 usec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 117443.83520425485,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010687986834529098",
            "extra": "mean: 8.514708313644812 usec\nrounds: 122474"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 886145.286957304,
            "unit": "iter/sec",
            "range": "stddev: 1.3723613233678552e-7",
            "extra": "mean: 1.128483122032534 usec\nrounds: 91492"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 716706.9564038923,
            "unit": "iter/sec",
            "range": "stddev: 1.4175270006727888e-7",
            "extra": "mean: 1.3952703975660299 usec\nrounds: 74102"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 493132.5622821066,
            "unit": "iter/sec",
            "range": "stddev: 1.6706271974745462e-7",
            "extra": "mean: 2.0278522987251697 usec\nrounds: 50615"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 155604.22977748435,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011162308871033549",
            "extra": "mean: 6.426560521073305 usec\nrounds: 170040"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5449185.9111769665,
            "unit": "iter/sec",
            "range": "stddev: 1.3744826706704078e-8",
            "extra": "mean: 183.51365071779148 nsec\nrounds: 56619"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 401771.3236893808,
            "unit": "iter/sec",
            "range": "stddev: 3.556551101028782e-7",
            "extra": "mean: 2.488978035607963 usec\nrounds: 139802"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54623.30741554686,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014017602463600992",
            "extra": "mean: 18.30720341396575 usec\nrounds: 56648"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4101.908025494825,
            "unit": "iter/sec",
            "range": "stddev: 0.000010500215368231122",
            "extra": "mean: 243.78898643866282 usec\nrounds: 4203"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4326824.110270454,
            "unit": "iter/sec",
            "range": "stddev: 1.8578723559919405e-8",
            "extra": "mean: 231.11639727310154 nsec\nrounds: 45621"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 390414.55326266645,
            "unit": "iter/sec",
            "range": "stddev: 3.5672462362929383e-7",
            "extra": "mean: 2.5613799271647455 usec\nrounds: 136370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2576836.962274493,
            "unit": "iter/sec",
            "range": "stddev: 5.468898373782664e-8",
            "extra": "mean: 388.07266995942757 nsec\nrounds: 190513"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 46645.94957175764,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024374365791022254",
            "extra": "mean: 21.438088605349392 usec\nrounds: 49049"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 110043.95840115736,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012486740192274869",
            "extra": "mean: 9.087277616410086 usec\nrounds: 115674"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13481.931444332982,
            "unit": "iter/sec",
            "range": "stddev: 0.0000044929720220516965",
            "extra": "mean: 74.17334853904347 usec\nrounds: 13964"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1280.5735987974178,
            "unit": "iter/sec",
            "range": "stddev: 0.00006115346201597016",
            "extra": "mean: 780.900059894329 usec\nrounds: 1319"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 50159.09572678933,
            "unit": "iter/sec",
            "range": "stddev: 0.000002193680576336079",
            "extra": "mean: 19.936563558619195 usec\nrounds: 52841"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 48098.26429274322,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026587431646785703",
            "extra": "mean: 20.790771033100963 usec\nrounds: 50361"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35260.654093852965,
            "unit": "iter/sec",
            "range": "stddev: 0.000003908981901732478",
            "extra": "mean: 28.360222624864218 usec\nrounds: 36968"
          }
        ]
      }
    ]
  }
}