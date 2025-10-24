window.BENCHMARK_DATA = {
  "lastUpdate": 1761286939608,
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
      },
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
          "id": "829b7c1a5d0578ae2b5752dab2bb1a16b0b10526",
          "message": "Testing updates across the projects",
          "timestamp": "2025-10-24T00:08:19-05:00",
          "tree_id": "8b17acfd5198e6c582acd2b5f996d07c42bb13c9",
          "url": "https://github.com/cachemcclure/astrora/commit/829b7c1a5d0578ae2b5752dab2bb1a16b0b10526"
        },
        "date": 1761282842190,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5662756.873441472,
            "unit": "iter/sec",
            "range": "stddev: 8.875289937654992e-9",
            "extra": "mean: 176.59243056862974 nsec\nrounds: 58439"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5247245.134074704,
            "unit": "iter/sec",
            "range": "stddev: 9.553980279871583e-9",
            "extra": "mean: 190.57619273511017 nsec\nrounds: 54178"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3002501.0743613597,
            "unit": "iter/sec",
            "range": "stddev: 2.9339768403503565e-8",
            "extra": "mean: 333.055667669562 nsec\nrounds: 194251"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 576412.198437661,
            "unit": "iter/sec",
            "range": "stddev: 9.022019701935698e-8",
            "extra": "mean: 1.7348695997594459 usec\nrounds: 58914"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 65183.146799721195,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028719702536351635",
            "extra": "mean: 15.341388826663355 usec\nrounds: 66086"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 450290.5424717091,
            "unit": "iter/sec",
            "range": "stddev: 1.642275847852124e-7",
            "extra": "mean: 2.220788370350513 usec\nrounds: 46519"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 447910.44330271543,
            "unit": "iter/sec",
            "range": "stddev: 1.4911362912425184e-7",
            "extra": "mean: 2.2325891591774383 usec\nrounds: 46205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 379956.30208702286,
            "unit": "iter/sec",
            "range": "stddev: 2.458160024303319e-7",
            "extra": "mean: 2.631881599297611 usec\nrounds: 197161"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 208394.3993555358,
            "unit": "iter/sec",
            "range": "stddev: 3.2704303921728847e-7",
            "extra": "mean: 4.798593451131708 usec\nrounds: 106644"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 36484.32853034941,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014061777479414443",
            "extra": "mean: 27.4090284865227 usec\nrounds: 37035"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3758696.696511161,
            "unit": "iter/sec",
            "range": "stddev: 2.5237311554616156e-8",
            "extra": "mean: 266.0496658132018 nsec\nrounds: 197045"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3601703.773362986,
            "unit": "iter/sec",
            "range": "stddev: 2.6463013869636664e-8",
            "extra": "mean: 277.64637597230654 nsec\nrounds: 198966"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2352848.5744658983,
            "unit": "iter/sec",
            "range": "stddev: 3.3576307114285614e-8",
            "extra": "mean: 425.01672689539 nsec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 307503.95024413895,
            "unit": "iter/sec",
            "range": "stddev: 2.4154534264738456e-7",
            "extra": "mean: 3.2519907442036518 usec\nrounds: 157307"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1610423.3854958604,
            "unit": "iter/sec",
            "range": "stddev: 6.269257219463421e-8",
            "extra": "mean: 620.9547184960129 nsec\nrounds: 165646"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1553038.028809808,
            "unit": "iter/sec",
            "range": "stddev: 5.5566942091154435e-8",
            "extra": "mean: 643.899235852162 nsec\nrounds: 159262"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1318045.2669398629,
            "unit": "iter/sec",
            "range": "stddev: 6.555376269398794e-8",
            "extra": "mean: 758.6992837671892 nsec\nrounds: 136129"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 308804.9243862019,
            "unit": "iter/sec",
            "range": "stddev: 2.438639691817327e-7",
            "extra": "mean: 3.2382903283930995 usec\nrounds: 157978"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2901520.1113209818,
            "unit": "iter/sec",
            "range": "stddev: 2.9312095210087953e-8",
            "extra": "mean: 344.6469304480425 nsec\nrounds: 198650"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 517339.7016031973,
            "unit": "iter/sec",
            "range": "stddev: 1.1318404808622785e-7",
            "extra": "mean: 1.932965896298084 usec\nrounds: 53308"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3171686.289332069,
            "unit": "iter/sec",
            "range": "stddev: 2.976444725971806e-8",
            "extra": "mean: 315.2896941174255 nsec\nrounds: 193125"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2992916.7754264134,
            "unit": "iter/sec",
            "range": "stddev: 2.8652642700247736e-8",
            "extra": "mean: 334.12222090857364 nsec\nrounds: 193611"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1805474.8425296943,
            "unit": "iter/sec",
            "range": "stddev: 5.4031910654599054e-8",
            "extra": "mean: 553.8709133154504 nsec\nrounds: 194742"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 461752.8303054861,
            "unit": "iter/sec",
            "range": "stddev: 1.0632789325520201e-7",
            "extra": "mean: 2.1656607915936887 usec\nrounds: 47347"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 22377.737291413083,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020441734958903967",
            "extra": "mean: 44.68727052148055 usec\nrounds: 23037"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1278133.5147877217,
            "unit": "iter/sec",
            "range": "stddev: 6.389099546607478e-8",
            "extra": "mean: 782.3908757811428 nsec\nrounds: 133458"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1262594.5330980178,
            "unit": "iter/sec",
            "range": "stddev: 5.855835534349024e-8",
            "extra": "mean: 792.0199032909694 nsec\nrounds: 131114"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 944912.9806386004,
            "unit": "iter/sec",
            "range": "stddev: 7.691649678267591e-8",
            "extra": "mean: 1.0582985105402682 usec\nrounds: 96881"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 368494.7882531401,
            "unit": "iter/sec",
            "range": "stddev: 2.351279210203672e-7",
            "extra": "mean: 2.713742587081158 usec\nrounds: 190006"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 20766.585947306026,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021569112099896254",
            "extra": "mean: 48.154280272040886 usec\nrounds: 21315"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2936575.6359770265,
            "unit": "iter/sec",
            "range": "stddev: 3.2487480773458057e-8",
            "extra": "mean: 340.532689758999 nsec\nrounds: 196040"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2443836.1404411322,
            "unit": "iter/sec",
            "range": "stddev: 3.835553749797101e-8",
            "extra": "mean: 409.1927373737458 nsec\nrounds: 199881"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 874680.3781831142,
            "unit": "iter/sec",
            "range": "stddev: 6.576572379907821e-8",
            "extra": "mean: 1.1432747606356521 usec\nrounds: 91492"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 111704.52067536273,
            "unit": "iter/sec",
            "range": "stddev: 5.105835308019855e-7",
            "extra": "mean: 8.952189167940787 usec\nrounds: 113238"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 376634.63664591045,
            "unit": "iter/sec",
            "range": "stddev: 2.634874000047119e-7",
            "extra": "mean: 2.6550930336769336 usec\nrounds: 193387"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 359807.83012198046,
            "unit": "iter/sec",
            "range": "stddev: 2.834618620808319e-7",
            "extra": "mean: 2.77926136199144 usec\nrounds: 183925"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 278191.253274334,
            "unit": "iter/sec",
            "range": "stddev: 4.226933183374918e-7",
            "extra": "mean: 3.5946493221117395 usec\nrounds: 142715"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 89266.59396892018,
            "unit": "iter/sec",
            "range": "stddev: 7.490412208723946e-7",
            "extra": "mean: 11.202398966271398 usec\nrounds: 96349"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2491404.6625910075,
            "unit": "iter/sec",
            "range": "stddev: 7.909736829475063e-7",
            "extra": "mean: 401.3799986068989 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2587991.727016441,
            "unit": "iter/sec",
            "range": "stddev: 1.6049695376335906e-7",
            "extra": "mean: 386.3999987174793 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1999160.352942073,
            "unit": "iter/sec",
            "range": "stddev: 1.681529052681338e-7",
            "extra": "mean: 500.2099999273924 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 538662.5009630859,
            "unit": "iter/sec",
            "range": "stddev: 1.792994970580212e-7",
            "extra": "mean: 1.856450000161658 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 57791.20845579585,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017019953478839404",
            "extra": "mean: 17.30366999964872 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 919354.2460214634,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012484824063140873",
            "extra": "mean: 1.0877199994752118 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 990677.7223563502,
            "unit": "iter/sec",
            "range": "stddev: 5.546341252075842e-7",
            "extra": "mean: 1.0094100002788764 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 783674.4936648762,
            "unit": "iter/sec",
            "range": "stddev: 5.716189648838647e-7",
            "extra": "mean: 1.276039998856504 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 412823.9630031762,
            "unit": "iter/sec",
            "range": "stddev: 6.041554969821565e-7",
            "extra": "mean: 2.422340003533918 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 25968.636640952132,
            "unit": "iter/sec",
            "range": "stddev: 0.000002546292530075544",
            "extra": "mean: 38.50798999678773 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1688577.996931468,
            "unit": "iter/sec",
            "range": "stddev: 5.7974854661604373e-8",
            "extra": "mean: 592.2142784148759 nsec\nrounds: 178031"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 427462.5828015268,
            "unit": "iter/sec",
            "range": "stddev: 1.338237630292982e-7",
            "extra": "mean: 2.3393860427412188 usec\nrounds: 44321"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 50237.41870932695,
            "unit": "iter/sec",
            "range": "stddev: 0.000004354408183467933",
            "extra": "mean: 19.905481326299565 usec\nrounds: 52989"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 5213.672823061645,
            "unit": "iter/sec",
            "range": "stddev: 0.000007301168086633904",
            "extra": "mean: 191.80336663564668 usec\nrounds: 5395"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 233318.2272473055,
            "unit": "iter/sec",
            "range": "stddev: 3.8006717571303134e-7",
            "extra": "mean: 4.285991762401189 usec\nrounds: 121759"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 147299.0659725182,
            "unit": "iter/sec",
            "range": "stddev: 0.000024614786381938996",
            "extra": "mean: 6.788909307724811 usec\nrounds: 153894"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 35658.0388104326,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011812595922743277",
            "extra": "mean: 28.04416713202484 usec\nrounds: 36893"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 4153.3532829732685,
            "unit": "iter/sec",
            "range": "stddev: 0.000004652592163587875",
            "extra": "mean: 240.76930900617444 usec\nrounds: 4275"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 139811.8167384672,
            "unit": "iter/sec",
            "range": "stddev: 6.332249043148751e-7",
            "extra": "mean: 7.152471252630997 usec\nrounds: 148744"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17618.739015143045,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020722523492610707",
            "extra": "mean: 56.75775088901168 usec\nrounds: 18000"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1737.2021271854335,
            "unit": "iter/sec",
            "range": "stddev: 0.0000108893847406406",
            "extra": "mean: 575.6382543810099 usec\nrounds: 1769"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1162780.5304321938,
            "unit": "iter/sec",
            "range": "stddev: 6.591571130256872e-8",
            "extra": "mean: 860.0075197581209 nsec\nrounds: 129685"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 110910.62373088607,
            "unit": "iter/sec",
            "range": "stddev: 9.583575083599158e-7",
            "extra": "mean: 9.016268833059703 usec\nrounds: 115581"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 18935.38497369245,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014066872565129366",
            "extra": "mean: 52.81117872117904 usec\nrounds: 19315"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2569.3523978587764,
            "unit": "iter/sec",
            "range": "stddev: 0.000005957678098695597",
            "extra": "mean: 389.2031318216103 usec\nrounds: 2602"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 880261.6799730078,
            "unit": "iter/sec",
            "range": "stddev: 8.921969571381382e-8",
            "extra": "mean: 1.1360258236285654 usec\nrounds: 91819"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 159423.70263723322,
            "unit": "iter/sec",
            "range": "stddev: 5.797960602834861e-7",
            "extra": "mean: 6.272592992495529 usec\nrounds: 170503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 31716.946425224894,
            "unit": "iter/sec",
            "range": "stddev: 0.000001490352385615504",
            "extra": "mean: 31.528886374909256 usec\nrounds: 36198"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 5948.446308225767,
            "unit": "iter/sec",
            "range": "stddev: 0.000027350026073648236",
            "extra": "mean: 168.11112485241011 usec\nrounds: 6760"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2177501.4399520243,
            "unit": "iter/sec",
            "range": "stddev: 3.780105010383181e-8",
            "extra": "mean: 459.2419465963809 nsec\nrounds: 189754"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 994088.364408083,
            "unit": "iter/sec",
            "range": "stddev: 7.04107236015217e-8",
            "extra": "mean: 1.0059467908523754 usec\nrounds: 102691"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 329655.54717718245,
            "unit": "iter/sec",
            "range": "stddev: 2.39322814846949e-7",
            "extra": "mean: 3.0334693547945135 usec\nrounds: 172474"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 99405.22811991119,
            "unit": "iter/sec",
            "range": "stddev: 6.008893143292746e-7",
            "extra": "mean: 10.059833058214137 usec\nrounds: 101730"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 882290.0118286278,
            "unit": "iter/sec",
            "range": "stddev: 7.918904391946568e-8",
            "extra": "mean: 1.1334141683497287 usec\nrounds: 91175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 728645.614756963,
            "unit": "iter/sec",
            "range": "stddev: 8.141046122083849e-8",
            "extra": "mean: 1.372409275163966 usec\nrounds: 75535"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 384225.3681823067,
            "unit": "iter/sec",
            "range": "stddev: 2.2657241114742117e-7",
            "extra": "mean: 2.6026391873363277 usec\nrounds: 198531"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 148545.74534179378,
            "unit": "iter/sec",
            "range": "stddev: 4.939482121371116e-7",
            "extra": "mean: 6.731932965828589 usec\nrounds: 152370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5948587.790194191,
            "unit": "iter/sec",
            "range": "stddev: 9.230524023996728e-9",
            "extra": "mean: 168.10712647603674 nsec\nrounds: 61047"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 452669.2726315868,
            "unit": "iter/sec",
            "range": "stddev: 1.1957712769533813e-7",
            "extra": "mean: 2.2091183573970268 usec\nrounds: 46499"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 59217.53580949623,
            "unit": "iter/sec",
            "range": "stddev: 8.806483865376924e-7",
            "extra": "mean: 16.88688977564038 usec\nrounds: 60894"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4616.3526916970295,
            "unit": "iter/sec",
            "range": "stddev: 0.000004609420530456685",
            "extra": "mean: 216.62123039225312 usec\nrounds: 4692"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4627442.583695751,
            "unit": "iter/sec",
            "range": "stddev: 1.081415282990533e-8",
            "extra": "mean: 216.102087041206 nsec\nrounds: 47311"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 440954.6410913796,
            "unit": "iter/sec",
            "range": "stddev: 1.1791005275347086e-7",
            "extra": "mean: 2.2678069506762752 usec\nrounds: 45492"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2609110.4049472343,
            "unit": "iter/sec",
            "range": "stddev: 3.265172947920474e-8",
            "extra": "mean: 383.2723974055991 nsec\nrounds: 194667"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 52987.21749390106,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012828038040829223",
            "extra": "mean: 18.87247618003535 usec\nrounds: 54660"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 115793.21755935582,
            "unit": "iter/sec",
            "range": "stddev: 7.665794322023312e-7",
            "extra": "mean: 8.636084401812207 usec\nrounds: 122130"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14213.004939233324,
            "unit": "iter/sec",
            "range": "stddev: 0.000002131412354081721",
            "extra": "mean: 70.35809839477491 usec\nrounds: 14513"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1398.9514221919608,
            "unit": "iter/sec",
            "range": "stddev: 0.000007737022593473191",
            "extra": "mean: 714.8211039616659 usec\nrounds: 1414"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 54641.981073179144,
            "unit": "iter/sec",
            "range": "stddev: 0.00000123408022186428",
            "extra": "mean: 18.300947007407224 usec\nrounds: 56574"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 51451.06299281179,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026800087251619527",
            "extra": "mean: 19.43594440681837 usec\nrounds: 54485"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 37426.42048966507,
            "unit": "iter/sec",
            "range": "stddev: 0.000001267770755294137",
            "extra": "mean: 26.719092740277954 usec\nrounds: 38527"
          }
        ]
      },
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
          "id": "1c28c48cf4be031b156d4526b9cf3a6e82618f4e",
          "message": "Fixing workflow issue with free-threading",
          "timestamp": "2025-10-24T00:20:06-05:00",
          "tree_id": "220fbb6ccc82c845d6e162ec241d01d68874e2b8",
          "url": "https://github.com/cachemcclure/astrora/commit/1c28c48cf4be031b156d4526b9cf3a6e82618f4e"
        },
        "date": 1761283533928,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5173516.603149019,
            "unit": "iter/sec",
            "range": "stddev: 1.2694476064456753e-8",
            "extra": "mean: 193.29212153127028 nsec\nrounds: 53320"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4904130.155107784,
            "unit": "iter/sec",
            "range": "stddev: 1.2907008832306421e-8",
            "extra": "mean: 203.90975940112753 nsec\nrounds: 49834"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3150283.043436941,
            "unit": "iter/sec",
            "range": "stddev: 1.2903839571222974e-7",
            "extra": "mean: 317.4317946074488 nsec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 706207.6392484785,
            "unit": "iter/sec",
            "range": "stddev: 1.0272802598268754e-7",
            "extra": "mean: 1.4160141358202119 usec\nrounds: 71860"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 73621.3185932409,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012535429088113427",
            "extra": "mean: 13.583022134186674 usec\nrounds: 77979"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 419211.1338159388,
            "unit": "iter/sec",
            "range": "stddev: 1.782511479881127e-7",
            "extra": "mean: 2.3854328268844593 usec\nrounds: 42913"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 404551.3015057553,
            "unit": "iter/sec",
            "range": "stddev: 6.995792071404071e-7",
            "extra": "mean: 2.4718743859628356 usec\nrounds: 140194"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 357181.3346907063,
            "unit": "iter/sec",
            "range": "stddev: 4.0188398650469294e-7",
            "extra": "mean: 2.799698368521774 usec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 213113.53388731496,
            "unit": "iter/sec",
            "range": "stddev: 5.245320023932199e-7",
            "extra": "mean: 4.692334558765076 usec\nrounds: 110169"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39570.9023264015,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019497781974051684",
            "extra": "mean: 25.271094193189658 usec\nrounds: 41468"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3641087.692959409,
            "unit": "iter/sec",
            "range": "stddev: 6.725896795149084e-8",
            "extra": "mean: 274.6432067356954 nsec\nrounds: 198060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3426592.442802527,
            "unit": "iter/sec",
            "range": "stddev: 3.7453026724732996e-8",
            "extra": "mean: 291.83511511573 nsec\nrounds: 196464"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2177459.3444244997,
            "unit": "iter/sec",
            "range": "stddev: 8.285867391680854e-8",
            "extra": "mean: 459.2508248480352 nsec\nrounds: 189430"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 507298.64198920655,
            "unit": "iter/sec",
            "range": "stddev: 1.3050578199702508e-7",
            "extra": "mean: 1.971225462143598 usec\nrounds: 52097"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1553682.3705386994,
            "unit": "iter/sec",
            "range": "stddev: 7.450901778274879e-8",
            "extra": "mean: 643.6321985512079 nsec\nrounds: 160206"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1539795.944236887,
            "unit": "iter/sec",
            "range": "stddev: 7.723013202952888e-8",
            "extra": "mean: 649.4367021440776 nsec\nrounds: 158958"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1299635.0485609106,
            "unit": "iter/sec",
            "range": "stddev: 8.365467252975302e-8",
            "extra": "mean: 769.4467774682927 nsec\nrounds: 136352"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 446647.1413034321,
            "unit": "iter/sec",
            "range": "stddev: 1.519387786551611e-7",
            "extra": "mean: 2.23890384047179 usec\nrounds: 45515"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2710570.5946329725,
            "unit": "iter/sec",
            "range": "stddev: 4.4602508601366454e-8",
            "extra": "mean: 368.926012102409 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 519546.07843872515,
            "unit": "iter/sec",
            "range": "stddev: 1.5242902190729168e-7",
            "extra": "mean: 1.9247570937405278 usec\nrounds: 53519"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2941438.185126604,
            "unit": "iter/sec",
            "range": "stddev: 4.093099537774445e-8",
            "extra": "mean: 339.96974849123285 nsec\nrounds: 189036"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2799440.572721378,
            "unit": "iter/sec",
            "range": "stddev: 7.549654501210265e-8",
            "extra": "mean: 357.2142269225532 nsec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1808432.120211775,
            "unit": "iter/sec",
            "range": "stddev: 6.690016388145231e-8",
            "extra": "mean: 552.9651839423084 nsec\nrounds: 185874"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 493468.23300341656,
            "unit": "iter/sec",
            "range": "stddev: 1.5838016604796487e-7",
            "extra": "mean: 2.0264728975838033 usec\nrounds: 50490"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 36134.68473369052,
            "unit": "iter/sec",
            "range": "stddev: 0.000001988971905602588",
            "extra": "mean: 27.674241725641522 usec\nrounds: 49097"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1245296.7621257054,
            "unit": "iter/sec",
            "range": "stddev: 8.87672912953119e-8",
            "extra": "mean: 803.0214406829493 nsec\nrounds: 129133"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1206730.659518772,
            "unit": "iter/sec",
            "range": "stddev: 3.375782490350646e-7",
            "extra": "mean: 828.6853343054499 nsec\nrounds: 126663"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 921022.8112594875,
            "unit": "iter/sec",
            "range": "stddev: 1.1106646353706832e-7",
            "extra": "mean: 1.0857494383146957 usec\nrounds: 95786"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 355040.5376079879,
            "unit": "iter/sec",
            "range": "stddev: 3.805143458709162e-7",
            "extra": "mean: 2.8165797819519227 usec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34962.73082375143,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018635292865038008",
            "extra": "mean: 28.601884819610955 usec\nrounds: 36152"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2818726.5358715765,
            "unit": "iter/sec",
            "range": "stddev: 4.5303491587968165e-8",
            "extra": "mean: 354.7701372495458 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2291852.172563303,
            "unit": "iter/sec",
            "range": "stddev: 6.516468169639826e-8",
            "extra": "mean: 436.3283164470266 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 786030.1036789509,
            "unit": "iter/sec",
            "range": "stddev: 1.0207845777291248e-7",
            "extra": "mean: 1.2722159053700157 usec\nrounds: 80432"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107138.87458254074,
            "unit": "iter/sec",
            "range": "stddev: 9.114697371096518e-7",
            "extra": "mean: 9.333680271483448 usec\nrounds: 109446"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 364781.1639108835,
            "unit": "iter/sec",
            "range": "stddev: 4.5543104846752956e-7",
            "extra": "mean: 2.7413696180987603 usec\nrounds: 191242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 358774.16759727243,
            "unit": "iter/sec",
            "range": "stddev: 4.5009004176027784e-7",
            "extra": "mean: 2.7872686785033807 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 293144.47780890594,
            "unit": "iter/sec",
            "range": "stddev: 4.6264983609112806e-7",
            "extra": "mean: 3.411287183283994 usec\nrounds: 153799"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 128679.31658456559,
            "unit": "iter/sec",
            "range": "stddev: 9.83870075780128e-7",
            "extra": "mean: 7.771256690991353 usec\nrounds: 132381"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2674583.393027512,
            "unit": "iter/sec",
            "range": "stddev: 4.547864149992729e-7",
            "extra": "mean: 373.89000567600306 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2813731.0028988174,
            "unit": "iter/sec",
            "range": "stddev: 1.9105438463906172e-7",
            "extra": "mean: 355.4000005578928 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1850412.6365934324,
            "unit": "iter/sec",
            "range": "stddev: 1.475637263901248e-7",
            "extra": "mean: 540.4200015846072 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 426850.6102400773,
            "unit": "iter/sec",
            "range": "stddev: 3.592809225712761e-7",
            "extra": "mean: 2.3427400032005607 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 46971.886862389925,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025453908256388563",
            "extra": "mean: 21.289329997102868 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 893854.7471132532,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013310908972864985",
            "extra": "mean: 1.1187500018650098 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 811866.2390352618,
            "unit": "iter/sec",
            "range": "stddev: 0.000002165669403565507",
            "extra": "mean: 1.2317299967889994 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 806191.549985483,
            "unit": "iter/sec",
            "range": "stddev: 4.354889986293655e-7",
            "extra": "mean: 1.2404000017340877 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 420535.59492421924,
            "unit": "iter/sec",
            "range": "stddev: 6.245010284322526e-7",
            "extra": "mean: 2.3779199955242802 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 51142.4716832755,
            "unit": "iter/sec",
            "range": "stddev: 0.000002576426035996442",
            "extra": "mean: 19.553219996737425 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1656752.6779855816,
            "unit": "iter/sec",
            "range": "stddev: 7.573959114462448e-8",
            "extra": "mean: 603.5903929945057 nsec\nrounds: 169751"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 359739.4005527013,
            "unit": "iter/sec",
            "range": "stddev: 4.3958396613791164e-7",
            "extra": "mean: 2.7797900326280818 usec\nrounds: 184843"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43352.42365508967,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013844164318043732",
            "extra": "mean: 23.066761110196843 usec\nrounds: 43991"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4421.748568633786,
            "unit": "iter/sec",
            "range": "stddev: 0.0000058533999868739825",
            "extra": "mean: 226.1548761713007 usec\nrounds: 4482"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 206788.87146000782,
            "unit": "iter/sec",
            "range": "stddev: 5.839205820380417e-7",
            "extra": "mean: 4.8358501738493995 usec\nrounds: 108132"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 130890.84971305069,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010209724040621075",
            "extra": "mean: 7.639953458872636 usec\nrounds: 137685"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31823.93909648611,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019372912287588933",
            "extra": "mean: 31.422885676349743 usec\nrounds: 32513"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3733.8231122437905,
            "unit": "iter/sec",
            "range": "stddev: 0.000005369100392138325",
            "extra": "mean: 267.82200707924363 usec\nrounds: 3814"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 131277.74834816955,
            "unit": "iter/sec",
            "range": "stddev: 8.969850260297649e-7",
            "extra": "mean: 7.617437171056897 usec\nrounds: 138065"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16520.54703706108,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028101223625223923",
            "extra": "mean: 60.53068326107287 usec\nrounds: 17074"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1581.6970202279615,
            "unit": "iter/sec",
            "range": "stddev: 0.000012156349575815095",
            "extra": "mean: 632.2323347715957 usec\nrounds: 1622"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1267557.4704216048,
            "unit": "iter/sec",
            "range": "stddev: 9.149230966263621e-8",
            "extra": "mean: 788.9188643000017 nsec\nrounds: 131338"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104953.64554073925,
            "unit": "iter/sec",
            "range": "stddev: 9.011473154007705e-7",
            "extra": "mean: 9.528015866888929 usec\nrounds: 107331"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17143.039143485865,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024085102837652244",
            "extra": "mean: 58.33271403221331 usec\nrounds: 17460"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2360.194960899084,
            "unit": "iter/sec",
            "range": "stddev: 0.00000610183051286702",
            "extra": "mean: 423.6938119802881 usec\nrounds: 2404"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 726477.4152882778,
            "unit": "iter/sec",
            "range": "stddev: 1.5392377856062637e-7",
            "extra": "mean: 1.3765052828286055 usec\nrounds: 74941"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 102677.41541093259,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017300995428395257",
            "extra": "mean: 9.739240085055012 usec\nrounds: 106758"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 22294.046601753937,
            "unit": "iter/sec",
            "range": "stddev: 0.000007555702597491595",
            "extra": "mean: 44.85502420728443 usec\nrounds: 22390"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4287.4450251707785,
            "unit": "iter/sec",
            "range": "stddev: 0.00001682060278760602",
            "extra": "mean: 233.23914222320965 usec\nrounds: 4507"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2193695.21585681,
            "unit": "iter/sec",
            "range": "stddev: 6.151236903201238e-8",
            "extra": "mean: 455.8518397504044 nsec\nrounds: 189754"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 996140.7773450469,
            "unit": "iter/sec",
            "range": "stddev: 9.464496406059465e-8",
            "extra": "mean: 1.0038741739548231 usec\nrounds: 102052"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 360891.46610381745,
            "unit": "iter/sec",
            "range": "stddev: 3.4408952343202044e-7",
            "extra": "mean: 2.7709161726543305 usec\nrounds: 185220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 116927.64747945257,
            "unit": "iter/sec",
            "range": "stddev: 9.63129118090174e-7",
            "extra": "mean: 8.55229726721157 usec\nrounds: 122775"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 896635.6495013232,
            "unit": "iter/sec",
            "range": "stddev: 1.0526573067765694e-7",
            "extra": "mean: 1.1152802150529968 usec\nrounds: 92166"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 722898.6382120806,
            "unit": "iter/sec",
            "range": "stddev: 1.257415328229065e-7",
            "extra": "mean: 1.3833198005093104 usec\nrounds: 74488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 500879.15871281776,
            "unit": "iter/sec",
            "range": "stddev: 1.4476982449363798e-7",
            "extra": "mean: 1.9964895376558576 usec\nrounds: 51346"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 184140.95046777677,
            "unit": "iter/sec",
            "range": "stddev: 6.685121399560816e-7",
            "extra": "mean: 5.4306225609223855 usec\nrounds: 95887"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5494510.122980557,
            "unit": "iter/sec",
            "range": "stddev: 1.3646143784313845e-8",
            "extra": "mean: 181.99984668647457 nsec\nrounds: 56616"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 418875.5426031342,
            "unit": "iter/sec",
            "range": "stddev: 1.9065239379187403e-7",
            "extra": "mean: 2.3873439680565203 usec\nrounds: 43568"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 52449.97710153935,
            "unit": "iter/sec",
            "range": "stddev: 0.000001326632188517085",
            "extra": "mean: 19.06578525409215 usec\nrounds: 53263"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4149.966500007536,
            "unit": "iter/sec",
            "range": "stddev: 0.000008551460140413007",
            "extra": "mean: 240.96580056686824 usec\nrounds: 4232"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4391392.206723692,
            "unit": "iter/sec",
            "range": "stddev: 1.37473819260444e-8",
            "extra": "mean: 227.71821621144724 nsec\nrounds: 44882"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 395454.1839961419,
            "unit": "iter/sec",
            "range": "stddev: 3.335162113417418e-7",
            "extra": "mean: 2.528737943533193 usec\nrounds: 136538"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2488689.2874037027,
            "unit": "iter/sec",
            "range": "stddev: 1.6682472387939303e-7",
            "extra": "mean: 401.8179388891257 nsec\nrounds: 187266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 47737.58897386812,
            "unit": "iter/sec",
            "range": "stddev: 0.000003206024614923279",
            "extra": "mean: 20.947853075433006 usec\nrounds: 49488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 108578.42026976193,
            "unit": "iter/sec",
            "range": "stddev: 9.986736109139162e-7",
            "extra": "mean: 9.209933221679876 usec\nrounds: 112791"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13102.96194577063,
            "unit": "iter/sec",
            "range": "stddev: 0.000003420481542199691",
            "extra": "mean: 76.31862201376381 usec\nrounds: 13646"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1255.5583408641685,
            "unit": "iter/sec",
            "range": "stddev: 0.000009688578564418174",
            "extra": "mean: 796.458410137856 usec\nrounds: 1302"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51885.31216719798,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020338015404337846",
            "extra": "mean: 19.273277122773145 usec\nrounds: 54189"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 48559.02545192878,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019838253842046247",
            "extra": "mean: 20.593494014618447 usec\nrounds: 50539"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 34113.11139085874,
            "unit": "iter/sec",
            "range": "stddev: 0.00000326168118654492",
            "extra": "mean: 29.31424192130328 usec\nrounds: 36206"
          }
        ]
      },
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
          "id": "0cccc37933d116d359f88685ea193993acc94bb7",
          "message": "Updated for release",
          "timestamp": "2025-10-24T00:30:41-05:00",
          "tree_id": "38bc8f4e5f41934652e1bd15738bf77dd2b015aa",
          "url": "https://github.com/cachemcclure/astrora/commit/0cccc37933d116d359f88685ea193993acc94bb7"
        },
        "date": 1761284166892,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5178243.468431036,
            "unit": "iter/sec",
            "range": "stddev: 1.203613082024443e-8",
            "extra": "mean: 193.1156783369 nsec\nrounds: 52813"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4890628.038431622,
            "unit": "iter/sec",
            "range": "stddev: 1.604603655299025e-8",
            "extra": "mean: 204.47271641632076 nsec\nrounds: 50158"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3159644.705995421,
            "unit": "iter/sec",
            "range": "stddev: 3.692741434660016e-8",
            "extra": "mean: 316.4912808397044 nsec\nrounds: 191205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 704307.5094519504,
            "unit": "iter/sec",
            "range": "stddev: 1.1279127361851502e-7",
            "extra": "mean: 1.4198343572660432 usec\nrounds: 71912"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 78215.96457390556,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012855482505717753",
            "extra": "mean: 12.785113697026762 usec\nrounds: 80952"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 399949.8599256814,
            "unit": "iter/sec",
            "range": "stddev: 3.5294485036002325e-7",
            "extra": "mean: 2.50031341475108 usec\nrounds: 138447"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 408348.47256276035,
            "unit": "iter/sec",
            "range": "stddev: 1.925243153034037e-7",
            "extra": "mean: 2.44888879765876 usec\nrounds: 42134"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 354380.5377063044,
            "unit": "iter/sec",
            "range": "stddev: 3.946128130510044e-7",
            "extra": "mean: 2.8218253927611503 usec\nrounds: 183824"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 212535.80343331184,
            "unit": "iter/sec",
            "range": "stddev: 5.344601889770061e-7",
            "extra": "mean: 4.705089607708255 usec\nrounds: 110303"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 40667.3277727099,
            "unit": "iter/sec",
            "range": "stddev: 0.000001844806995377499",
            "extra": "mean: 24.589764185859714 usec\nrounds: 41626"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3635726.070524159,
            "unit": "iter/sec",
            "range": "stddev: 3.770303148445571e-8",
            "extra": "mean: 275.0482243719939 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3304730.200201985,
            "unit": "iter/sec",
            "range": "stddev: 3.477318716003769e-8",
            "extra": "mean: 302.5965629323307 nsec\nrounds: 194591"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2145136.2805190026,
            "unit": "iter/sec",
            "range": "stddev: 6.073549731138377e-8",
            "extra": "mean: 466.1708484824405 nsec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 504265.4091154254,
            "unit": "iter/sec",
            "range": "stddev: 1.5287141179225125e-7",
            "extra": "mean: 1.983082682102204 usec\nrounds: 51825"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1529741.4101356657,
            "unit": "iter/sec",
            "range": "stddev: 7.644388573828755e-8",
            "extra": "mean: 653.7052559172857 nsec\nrounds: 159211"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1511664.0785014099,
            "unit": "iter/sec",
            "range": "stddev: 7.596316675225479e-8",
            "extra": "mean: 661.5226320594226 nsec\nrounds: 157679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1292854.642735426,
            "unit": "iter/sec",
            "range": "stddev: 9.469602076513284e-8",
            "extra": "mean: 773.4821587399942 nsec\nrounds: 134699"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 444485.34120094625,
            "unit": "iter/sec",
            "range": "stddev: 1.4524671512203426e-7",
            "extra": "mean: 2.2497929792197637 usec\nrounds: 45579"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2713294.496352242,
            "unit": "iter/sec",
            "range": "stddev: 4.624731334059098e-8",
            "extra": "mean: 368.5556438287056 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 497604.3530831694,
            "unit": "iter/sec",
            "range": "stddev: 1.5120849753550354e-7",
            "extra": "mean: 2.0096287217022573 usec\nrounds: 51905"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2966261.1516885296,
            "unit": "iter/sec",
            "range": "stddev: 4.286581744514651e-8",
            "extra": "mean: 337.1247334142359 nsec\nrounds: 192716"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2792303.410273739,
            "unit": "iter/sec",
            "range": "stddev: 5.012016626731669e-8",
            "extra": "mean: 358.12727095524195 nsec\nrounds: 191608"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1776337.9520851015,
            "unit": "iter/sec",
            "range": "stddev: 6.995734524274956e-8",
            "extra": "mean: 562.9559391140654 nsec\nrounds: 183487"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 491027.98795419076,
            "unit": "iter/sec",
            "range": "stddev: 1.520544938923287e-7",
            "extra": "mean: 2.036543790846581 usec\nrounds: 50184"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47594.24962778235,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019307609864232187",
            "extra": "mean: 21.010941612078 usec\nrounds: 49291"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1227344.9793252572,
            "unit": "iter/sec",
            "range": "stddev: 9.864671476024139e-8",
            "extra": "mean: 814.7668478260722 nsec\nrounds: 126040"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1193196.6911253473,
            "unit": "iter/sec",
            "range": "stddev: 8.614073401949659e-8",
            "extra": "mean: 838.0847914159687 nsec\nrounds: 124147"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 909922.8069673702,
            "unit": "iter/sec",
            "range": "stddev: 1.004679133168953e-7",
            "extra": "mean: 1.0989943238512936 usec\nrounds: 91400"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 364222.0662595634,
            "unit": "iter/sec",
            "range": "stddev: 4.2837407302659506e-7",
            "extra": "mean: 2.745577746756696 usec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 43467.00057105497,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018426920622945732",
            "extra": "mean: 23.00595824101809 usec\nrounds: 45164"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2530833.79668633,
            "unit": "iter/sec",
            "range": "stddev: 4.620111178582548e-8",
            "extra": "mean: 395.12669749757015 nsec\nrounds: 195695"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2288094.1019061976,
            "unit": "iter/sec",
            "range": "stddev: 5.406397295898407e-8",
            "extra": "mean: 437.0449620786579 nsec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 781716.650057959,
            "unit": "iter/sec",
            "range": "stddev: 1.0183013061396721e-7",
            "extra": "mean: 1.279235896953035 usec\nrounds: 80238"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 106906.69910877108,
            "unit": "iter/sec",
            "range": "stddev: 8.920214035313021e-7",
            "extra": "mean: 9.35395076582208 usec\nrounds: 108969"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 367671.4272373381,
            "unit": "iter/sec",
            "range": "stddev: 4.025537018617254e-7",
            "extra": "mean: 2.7198197246763023 usec\nrounds: 193088"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 358490.5682391048,
            "unit": "iter/sec",
            "range": "stddev: 4.185378983928162e-7",
            "extra": "mean: 2.7894736670813147 usec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 295839.52577969653,
            "unit": "iter/sec",
            "range": "stddev: 4.3294486729173306e-7",
            "extra": "mean: 3.3802109348453735 usec\nrounds: 154991"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 120623.55419530447,
            "unit": "iter/sec",
            "range": "stddev: 9.979727678020822e-7",
            "extra": "mean: 8.290254806958151 usec\nrounds: 124922"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2859103.373977632,
            "unit": "iter/sec",
            "range": "stddev: 5.219981258217801e-7",
            "extra": "mean: 349.760001370214 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2892346.854036829,
            "unit": "iter/sec",
            "range": "stddev: 1.629513553407757e-7",
            "extra": "mean: 345.739999545458 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2042441.9422139667,
            "unit": "iter/sec",
            "range": "stddev: 1.585065970726513e-7",
            "extra": "mean: 489.61000032932134 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 496664.89519220265,
            "unit": "iter/sec",
            "range": "stddev: 5.833737335077103e-7",
            "extra": "mean: 2.0134300001473093 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 56032.64237816306,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013279347424845935",
            "extra": "mean: 17.846739999356487 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 946933.8267525525,
            "unit": "iter/sec",
            "range": "stddev: 7.275127412245375e-7",
            "extra": "mean: 1.0560400016856875 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 995797.7329738293,
            "unit": "iter/sec",
            "range": "stddev: 4.092365782009649e-7",
            "extra": "mean: 1.0042200005955237 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 815228.4682081174,
            "unit": "iter/sec",
            "range": "stddev: 4.4317186323873306e-7",
            "extra": "mean: 1.2266499993529578 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 405089.5450195386,
            "unit": "iter/sec",
            "range": "stddev: 6.182896604347147e-7",
            "extra": "mean: 2.4685900001486516 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 53443.726697191894,
            "unit": "iter/sec",
            "range": "stddev: 0.000003549077425198119",
            "extra": "mean: 18.71126999930084 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1644585.8428302596,
            "unit": "iter/sec",
            "range": "stddev: 7.04424439075463e-8",
            "extra": "mean: 608.055824121068 nsec\nrounds: 169751"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 358076.9365750403,
            "unit": "iter/sec",
            "range": "stddev: 3.264294746543221e-7",
            "extra": "mean: 2.792695920504881 usec\nrounds: 183184"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43269.63492586851,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013359793257699257",
            "extra": "mean: 23.110895243586988 usec\nrounds: 47577"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4400.031975994656,
            "unit": "iter/sec",
            "range": "stddev: 0.000007638724303177785",
            "extra": "mean: 227.27107563211365 usec\nrounds: 4469"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 212969.96497853714,
            "unit": "iter/sec",
            "range": "stddev: 5.553493193462515e-7",
            "extra": "mean: 4.695497790501955 usec\nrounds: 110657"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 135104.87899024828,
            "unit": "iter/sec",
            "range": "stddev: 9.840388674133994e-7",
            "extra": "mean: 7.401657197533028 usec\nrounds: 140985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31662.41274641345,
            "unit": "iter/sec",
            "range": "stddev: 0.000002696223108593132",
            "extra": "mean: 31.583190074902763 usec\nrounds: 32866"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3748.588409616794,
            "unit": "iter/sec",
            "range": "stddev: 0.000005041089368657661",
            "extra": "mean: 266.76708422684015 usec\nrounds: 3823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 134970.0480355502,
            "unit": "iter/sec",
            "range": "stddev: 9.019459754285037e-7",
            "extra": "mean: 7.409051226955234 usec\nrounds: 142796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16574.385980129522,
            "unit": "iter/sec",
            "range": "stddev: 0.000002679821948568044",
            "extra": "mean: 60.33406010930762 usec\nrounds: 17019"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1637.3442429393413,
            "unit": "iter/sec",
            "range": "stddev: 0.000013907447009129476",
            "extra": "mean: 610.7451162528973 usec\nrounds: 1772"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1269849.3030360881,
            "unit": "iter/sec",
            "range": "stddev: 8.126521131332202e-8",
            "extra": "mean: 787.4950181955445 nsec\nrounds: 131338"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104793.65721875388,
            "unit": "iter/sec",
            "range": "stddev: 8.855744953744282e-7",
            "extra": "mean: 9.542562274666372 usec\nrounds: 107090"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17135.825413043498,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023824514349144736",
            "extra": "mean: 58.35727056595809 usec\nrounds: 17456"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2373.079998970097,
            "unit": "iter/sec",
            "range": "stddev: 0.000005289701343799933",
            "extra": "mean: 421.3932949727755 usec\nrounds: 2407"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 727780.9064668199,
            "unit": "iter/sec",
            "range": "stddev: 1.331220711661366e-7",
            "extra": "mean: 1.3740398945813506 usec\nrounds: 74935"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 100362.68529811157,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011548920250427816",
            "extra": "mean: 9.963862535459842 usec\nrounds: 111396"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21014.765101236953,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029621346879268198",
            "extra": "mean: 47.58559018778367 usec\nrounds: 22054"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4317.42380717605,
            "unit": "iter/sec",
            "range": "stddev: 0.00001461389414793347",
            "extra": "mean: 231.6196057329109 usec\nrounds: 4535"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2200444.8290874674,
            "unit": "iter/sec",
            "range": "stddev: 5.484056220499483e-8",
            "extra": "mean: 454.4535662885416 nsec\nrounds: 190151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 998679.729288021,
            "unit": "iter/sec",
            "range": "stddev: 9.45999254720479e-8",
            "extra": "mean: 1.0013220161311605 usec\nrounds: 102166"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 356014.7271458989,
            "unit": "iter/sec",
            "range": "stddev: 3.4199010063865877e-7",
            "extra": "mean: 2.8088725655166185 usec\nrounds: 182482"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 119470.54242342304,
            "unit": "iter/sec",
            "range": "stddev: 9.826121179226062e-7",
            "extra": "mean: 8.370264164833513 usec\nrounds: 124922"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 914207.7362963941,
            "unit": "iter/sec",
            "range": "stddev: 1.195054762829684e-7",
            "extra": "mean: 1.0938432921724632 usec\nrounds: 93458"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 725131.9610580185,
            "unit": "iter/sec",
            "range": "stddev: 1.2381856620500187e-7",
            "extra": "mean: 1.3790593349945772 usec\nrounds: 74767"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 459325.582105923,
            "unit": "iter/sec",
            "range": "stddev: 1.5912122353738576e-7",
            "extra": "mean: 2.1771049533431035 usec\nrounds: 47039"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 169996.59021588208,
            "unit": "iter/sec",
            "range": "stddev: 9.077257291129832e-7",
            "extra": "mean: 5.882470929152637 usec\nrounds: 193424"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5404328.333690097,
            "unit": "iter/sec",
            "range": "stddev: 1.2090332838495491e-8",
            "extra": "mean: 185.03687012613457 nsec\nrounds: 55146"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 399460.0936645491,
            "unit": "iter/sec",
            "range": "stddev: 3.340350329346257e-7",
            "extra": "mean: 2.503378975422184 usec\nrounds: 139199"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 51436.776706856625,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012723015087128634",
            "extra": "mean: 19.441342635039142 usec\nrounds: 53897"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4054.7060906937086,
            "unit": "iter/sec",
            "range": "stddev: 0.000011595026569894817",
            "extra": "mean: 246.62700023934724 usec\nrounds: 4175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4365251.40470882,
            "unit": "iter/sec",
            "range": "stddev: 1.3121784696606698e-8",
            "extra": "mean: 229.08188035201883 nsec\nrounds: 44481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 383293.48468063335,
            "unit": "iter/sec",
            "range": "stddev: 3.8571301042917364e-7",
            "extra": "mean: 2.608966862124508 usec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2543621.056745556,
            "unit": "iter/sec",
            "range": "stddev: 4.7531313447254533e-8",
            "extra": "mean: 393.14032149089815 nsec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48475.36318385572,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020399776447031108",
            "extra": "mean: 20.629035747648423 usec\nrounds: 50437"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 109022.25595666189,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010417800606322235",
            "extra": "mean: 9.172439069665888 usec\nrounds: 115394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13145.830908428386,
            "unit": "iter/sec",
            "range": "stddev: 0.00000519649521938939",
            "extra": "mean: 76.06974461833788 usec\nrounds: 13564"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1269.935215165991,
            "unit": "iter/sec",
            "range": "stddev: 0.000012799004163731462",
            "extra": "mean: 787.4417435296428 usec\nrounds: 1314"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51065.27172906713,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018561610756214974",
            "extra": "mean: 19.582780354241898 usec\nrounds: 53436"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49066.38589505463,
            "unit": "iter/sec",
            "range": "stddev: 0.000002757380232768114",
            "extra": "mean: 20.380551405168593 usec\nrounds: 51133"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35760.27681512674,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026864969300512935",
            "extra": "mean: 27.963989349685235 usec\nrounds: 37370"
          }
        ]
      },
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
          "id": "e4e2caa36fc7c2a9d140477f36fe1dd292cd2795",
          "message": "Updating documentation and including crates.io release action",
          "timestamp": "2025-10-24T00:57:39-05:00",
          "tree_id": "6be78bc796ccfa9fb1f01cea8293e1face00f36c",
          "url": "https://github.com/cachemcclure/astrora/commit/e4e2caa36fc7c2a9d140477f36fe1dd292cd2795"
        },
        "date": 1761285783336,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5619936.243962531,
            "unit": "iter/sec",
            "range": "stddev: 1.2501502263462162e-8",
            "extra": "mean: 177.93796167606874 nsec\nrounds: 58293"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 5258408.794574515,
            "unit": "iter/sec",
            "range": "stddev: 8.815150514071452e-9",
            "extra": "mean: 190.1715973531013 nsec\nrounds: 53958"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3057463.0733956136,
            "unit": "iter/sec",
            "range": "stddev: 1.401775242137424e-7",
            "extra": "mean: 327.0685453902806 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 579048.7869395203,
            "unit": "iter/sec",
            "range": "stddev: 7.236815457875215e-8",
            "extra": "mean: 1.7269702010522445 usec\nrounds: 58841"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 64975.493809140164,
            "unit": "iter/sec",
            "range": "stddev: 7.23550530309138e-7",
            "extra": "mean: 15.390417854113009 usec\nrounds: 65968"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 454517.1253580703,
            "unit": "iter/sec",
            "range": "stddev: 1.2122950918404226e-7",
            "extra": "mean: 2.20013712181296 usec\nrounds: 46571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 450945.67694723496,
            "unit": "iter/sec",
            "range": "stddev: 1.106275718268179e-7",
            "extra": "mean: 2.217562006070649 usec\nrounds: 46160"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 388021.3900214273,
            "unit": "iter/sec",
            "range": "stddev: 2.529503170498734e-7",
            "extra": "mean: 2.5771775106129535 usec\nrounds: 199681"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 210675.595094763,
            "unit": "iter/sec",
            "range": "stddev: 3.335426233341144e-7",
            "extra": "mean: 4.746634272233548 usec\nrounds: 107539"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 35325.056919929986,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029355032320507686",
            "extra": "mean: 28.308517726288834 usec\nrounds: 36330"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3732760.2480652714,
            "unit": "iter/sec",
            "range": "stddev: 6.047930786301748e-8",
            "extra": "mean: 267.89826657587355 nsec\nrounds: 194932"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3534319.646073293,
            "unit": "iter/sec",
            "range": "stddev: 2.3973740487143703e-8",
            "extra": "mean: 282.9398866373127 nsec\nrounds: 193987"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2359926.953309255,
            "unit": "iter/sec",
            "range": "stddev: 3.957540498422398e-8",
            "extra": "mean: 423.74192921425924 nsec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 300300.2366937846,
            "unit": "iter/sec",
            "range": "stddev: 2.4358926409804825e-7",
            "extra": "mean: 3.3300007053264418 usec\nrounds: 153823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1612542.2772548874,
            "unit": "iter/sec",
            "range": "stddev: 6.153317306715816e-8",
            "extra": "mean: 620.1387796804692 nsec\nrounds: 165481"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1548524.4376870128,
            "unit": "iter/sec",
            "range": "stddev: 6.504647868897441e-8",
            "extra": "mean: 645.7760534239152 nsec\nrounds: 159338"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1387141.0131660018,
            "unit": "iter/sec",
            "range": "stddev: 5.933745012523327e-8",
            "extra": "mean: 720.907240510184 nsec\nrounds: 143001"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 404138.0594412983,
            "unit": "iter/sec",
            "range": "stddev: 1.0575275180603983e-7",
            "extra": "mean: 2.4744019441832674 usec\nrounds: 40994"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2894347.7583581964,
            "unit": "iter/sec",
            "range": "stddev: 3.08686797849929e-8",
            "extra": "mean: 345.5009845006565 nsec\nrounds: 199323"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 487963.4088320755,
            "unit": "iter/sec",
            "range": "stddev: 2.5511886305560705e-7",
            "extra": "mean: 2.049333990828282 usec\nrounds: 55362"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 3145484.6285512126,
            "unit": "iter/sec",
            "range": "stddev: 2.904413774151459e-8",
            "extra": "mean: 317.9160345986404 nsec\nrounds: 194894"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2975231.7652587504,
            "unit": "iter/sec",
            "range": "stddev: 2.9949912701988917e-8",
            "extra": "mean: 336.10826950586545 nsec\nrounds: 194667"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1751519.098192824,
            "unit": "iter/sec",
            "range": "stddev: 5.4150800001692467e-8",
            "extra": "mean: 570.9329695758223 nsec\nrounds: 194364"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 453249.6643451421,
            "unit": "iter/sec",
            "range": "stddev: 1.715771078612992e-7",
            "extra": "mean: 2.2062895544441434 usec\nrounds: 47446"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 22258.80313308781,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020766910849390557",
            "extra": "mean: 44.92604539520346 usec\nrounds: 23020"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1289227.7822078995,
            "unit": "iter/sec",
            "range": "stddev: 7.300155338515748e-8",
            "extra": "mean: 775.658121707113 nsec\nrounds: 133174"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1273071.8746734175,
            "unit": "iter/sec",
            "range": "stddev: 5.934256938024639e-8",
            "extra": "mean: 785.5016043430728 nsec\nrounds: 130583"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 913209.0495544857,
            "unit": "iter/sec",
            "range": "stddev: 7.718554355178874e-8",
            "extra": "mean: 1.0950395207842603 usec\nrounds: 95823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 327558.74582025036,
            "unit": "iter/sec",
            "range": "stddev: 2.9630787931382123e-7",
            "extra": "mean: 3.0528874980757053 usec\nrounds: 168606"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 20022.873175513945,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022937519695325767",
            "extra": "mean: 49.9428823842776 usec\nrounds: 20686"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2895283.356068742,
            "unit": "iter/sec",
            "range": "stddev: 3.666845277335331e-8",
            "extra": "mean: 345.3893374214725 nsec\nrounds: 196117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2422691.384720853,
            "unit": "iter/sec",
            "range": "stddev: 3.841228269028241e-8",
            "extra": "mean: 412.76408803311125 nsec\nrounds: 197590"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 893866.3197471013,
            "unit": "iter/sec",
            "range": "stddev: 6.67301952066333e-8",
            "extra": "mean: 1.1187355177258858 usec\nrounds: 92337"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 109390.91305864786,
            "unit": "iter/sec",
            "range": "stddev: 6.025194359196986e-7",
            "extra": "mean: 9.141527134560702 usec\nrounds: 111334"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 377055.95900567307,
            "unit": "iter/sec",
            "range": "stddev: 2.5834499626728007e-7",
            "extra": "mean: 2.6521262325016175 usec\nrounds: 191682"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 357982.481409274,
            "unit": "iter/sec",
            "range": "stddev: 2.72604763277946e-7",
            "extra": "mean: 2.793432784932066 usec\nrounds: 183084"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 274685.5251822427,
            "unit": "iter/sec",
            "range": "stddev: 3.1070553620342016e-7",
            "extra": "mean: 3.6405267417587464 usec\nrounds: 143390"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 98537.99546241724,
            "unit": "iter/sec",
            "range": "stddev: 6.59752294596377e-7",
            "extra": "mean: 10.14836962439939 usec\nrounds: 100594"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2840182.900616219,
            "unit": "iter/sec",
            "range": "stddev: 4.1853064408874013e-7",
            "extra": "mean: 352.0900008879835 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2900316.1183228493,
            "unit": "iter/sec",
            "range": "stddev: 1.7726999947934647e-7",
            "extra": "mean: 344.79000191822706 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2092006.4461390786,
            "unit": "iter/sec",
            "range": "stddev: 1.6267843387317893e-7",
            "extra": "mean: 478.00999936953303 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 534142.3807484122,
            "unit": "iter/sec",
            "range": "stddev: 1.7795168548669227e-7",
            "extra": "mean: 1.8721600008575479 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 56929.80952925991,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017681124352542517",
            "extra": "mean: 17.565490000208683 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 928169.0018634651,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013127878558027452",
            "extra": "mean: 1.0773899990113023 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 1054240.68424919,
            "unit": "iter/sec",
            "range": "stddev: 4.939003555105318e-7",
            "extra": "mean: 948.5499990091739 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 832396.8872082011,
            "unit": "iter/sec",
            "range": "stddev: 4.475752724011665e-7",
            "extra": "mean: 1.201349999462309 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 379933.4355500259,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010870961787775013",
            "extra": "mean: 2.6320400007762146 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 26489.420654495276,
            "unit": "iter/sec",
            "range": "stddev: 0.0000024428153609486036",
            "extra": "mean: 37.75092000097402 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1665442.3088586128,
            "unit": "iter/sec",
            "range": "stddev: 5.755261454957964e-8",
            "extra": "mean: 600.4410928441804 nsec\nrounds: 179019"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 424238.3456408119,
            "unit": "iter/sec",
            "range": "stddev: 1.2537042582161282e-7",
            "extra": "mean: 2.357165518570719 usec\nrounds: 44392"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 51448.43939838675,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011356214871748025",
            "extra": "mean: 19.436935535723105 usec\nrounds: 52944"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 5223.498024052498,
            "unit": "iter/sec",
            "range": "stddev: 0.00000469607352433481",
            "extra": "mean: 191.44259180252914 usec\nrounds: 5392"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 234273.82154354313,
            "unit": "iter/sec",
            "range": "stddev: 3.3334890857911665e-7",
            "extra": "mean: 4.26850935973713 usec\nrounds: 120890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 147894.62045477936,
            "unit": "iter/sec",
            "range": "stddev: 5.408650128355226e-7",
            "extra": "mean: 6.7615711573888015 usec\nrounds: 154369"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 34633.601683634915,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012169404619702429",
            "extra": "mean: 28.873693505360155 usec\nrounds: 35583"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 4166.551789877516,
            "unit": "iter/sec",
            "range": "stddev: 0.0000042893617637353985",
            "extra": "mean: 240.00661708549097 usec\nrounds: 4296"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 141732.1562329603,
            "unit": "iter/sec",
            "range": "stddev: 6.177854086240269e-7",
            "extra": "mean: 7.055561889260574 usec\nrounds: 150535"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 17789.821362037426,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020892544566477147",
            "extra": "mean: 56.21191914461542 usec\nrounds: 18193"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1737.226887901642,
            "unit": "iter/sec",
            "range": "stddev: 0.000011413816726822861",
            "extra": "mean: 575.6300498018874 usec\nrounds: 1767"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1280311.3677612925,
            "unit": "iter/sec",
            "range": "stddev: 5.626615250154398e-8",
            "extra": "mean: 781.0600024184454 nsec\nrounds: 132363"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 110597.24435644233,
            "unit": "iter/sec",
            "range": "stddev: 6.596689987367974e-7",
            "extra": "mean: 9.041816600575633 usec\nrounds: 113948"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 18595.318964432052,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015311590387767744",
            "extra": "mean: 53.77697483505051 usec\nrounds: 18796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2561.332631173053,
            "unit": "iter/sec",
            "range": "stddev: 0.000004701371048818597",
            "extra": "mean: 390.42176241748604 usec\nrounds: 2597"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 786285.4517944646,
            "unit": "iter/sec",
            "range": "stddev: 1.6682189807161538e-7",
            "extra": "mean: 1.2718027501561795 usec\nrounds: 90613"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 158870.13630578908,
            "unit": "iter/sec",
            "range": "stddev: 6.077661897747623e-7",
            "extra": "mean: 6.294449185057827 usec\nrounds: 188537"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 28252.613960748826,
            "unit": "iter/sec",
            "range": "stddev: 0.000001811329812023651",
            "extra": "mean: 35.39495500803194 usec\nrounds: 32028"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 5890.235275894373,
            "unit": "iter/sec",
            "range": "stddev: 0.000010788929289236595",
            "extra": "mean: 169.7725053687537 usec\nrounds: 6985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2148608.05318979,
            "unit": "iter/sec",
            "range": "stddev: 3.928851700750109e-8",
            "extra": "mean: 465.41759839139695 nsec\nrounds: 188360"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 996214.9509070212,
            "unit": "iter/sec",
            "range": "stddev: 6.068618315989405e-8",
            "extra": "mean: 1.0037994301225064 usec\nrounds: 102828"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 325288.0977443029,
            "unit": "iter/sec",
            "range": "stddev: 2.354456519962343e-7",
            "extra": "mean: 3.0741979400244257 usec\nrounds: 169665"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 98946.18442417686,
            "unit": "iter/sec",
            "range": "stddev: 5.58369558093044e-7",
            "extra": "mean: 10.106503912400047 usec\nrounds: 101730"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 885062.2692617116,
            "unit": "iter/sec",
            "range": "stddev: 7.228570363828209e-8",
            "extra": "mean: 1.129864004748702 usec\nrounds: 91626"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 730133.9933126088,
            "unit": "iter/sec",
            "range": "stddev: 7.360323017139888e-8",
            "extra": "mean: 1.369611618085347 usec\nrounds: 74823"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 487688.743007264,
            "unit": "iter/sec",
            "range": "stddev: 8.902429277118421e-8",
            "extra": "mean: 2.050488173734812 usec\nrounds: 49796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 201919.2180079604,
            "unit": "iter/sec",
            "range": "stddev: 3.2017749392364915e-7",
            "extra": "mean: 4.9524755982393724 usec\nrounds: 103189"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5828729.928571926,
            "unit": "iter/sec",
            "range": "stddev: 9.229215316613954e-9",
            "extra": "mean: 171.56396200449427 nsec\nrounds: 60111"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 454970.73683735187,
            "unit": "iter/sec",
            "range": "stddev: 1.4686108349431688e-7",
            "extra": "mean: 2.197943557757839 usec\nrounds: 46653"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 59044.84284244559,
            "unit": "iter/sec",
            "range": "stddev: 8.695995717939464e-7",
            "extra": "mean: 16.936280153516297 usec\nrounds: 61245"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4583.53823575346,
            "unit": "iter/sec",
            "range": "stddev: 0.000013402160978892454",
            "extra": "mean: 218.1720645853009 usec\nrounds: 4676"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4681481.711053755,
            "unit": "iter/sec",
            "range": "stddev: 8.731848983395583e-9",
            "extra": "mean: 213.60758446173233 nsec\nrounds: 47654"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 443949.83823027066,
            "unit": "iter/sec",
            "range": "stddev: 1.0921800000186147e-7",
            "extra": "mean: 2.2525067336128286 usec\nrounds: 45102"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2597724.8677712264,
            "unit": "iter/sec",
            "range": "stddev: 3.088606439370976e-8",
            "extra": "mean: 384.95223740071106 nsec\nrounds: 192902"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 53223.035127022675,
            "unit": "iter/sec",
            "range": "stddev: 0.000001130589836774467",
            "extra": "mean: 18.788857073133634 usec\nrounds: 55182"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 116243.35534181762,
            "unit": "iter/sec",
            "range": "stddev: 6.656391062724842e-7",
            "extra": "mean: 8.602642250470707 usec\nrounds: 121996"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 14241.07878465844,
            "unit": "iter/sec",
            "range": "stddev: 0.000002413410726721812",
            "extra": "mean: 70.21939946552891 usec\nrounds: 14602"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1386.8964582465615,
            "unit": "iter/sec",
            "range": "stddev: 0.000008256426552907001",
            "extra": "mean: 721.0343598860217 usec\nrounds: 1406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 54918.66821410121,
            "unit": "iter/sec",
            "range": "stddev: 0.000002097332705959611",
            "extra": "mean: 18.208744540226025 usec\nrounds: 57238"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 52424.5157720472,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013029753588893503",
            "extra": "mean: 19.0750450485458 usec\nrounds: 54630"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 37695.92457318879,
            "unit": "iter/sec",
            "range": "stddev: 0.000001502876167444732",
            "extra": "mean: 26.52806666297421 usec\nrounds: 38882"
          }
        ]
      },
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
          "id": "5c3198955d183b575a375a94de77f68cf93b5fc3",
          "message": "Updating to combine release workflows",
          "timestamp": "2025-10-24T01:04:55-05:00",
          "tree_id": "fa225731157be1a77c463d39b1b2f2ec31f155f2",
          "url": "https://github.com/cachemcclure/astrora/commit/5c3198955d183b575a375a94de77f68cf93b5fc3"
        },
        "date": 1761286222670,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5141300.875225961,
            "unit": "iter/sec",
            "range": "stddev: 1.3975191735809722e-8",
            "extra": "mean: 194.5033026210283 nsec\nrounds: 52313"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4828931.781932844,
            "unit": "iter/sec",
            "range": "stddev: 1.3445999036546487e-8",
            "extra": "mean: 207.085137077604 nsec\nrounds: 49242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3103339.3171588853,
            "unit": "iter/sec",
            "range": "stddev: 4.024704225932478e-8",
            "extra": "mean: 322.23353549218155 nsec\nrounds: 199243"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 676441.3642378066,
            "unit": "iter/sec",
            "range": "stddev: 1.1257294370313913e-7",
            "extra": "mean: 1.478324734216676 usec\nrounds: 69701"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 75811.75363341965,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013148873107895828",
            "extra": "mean: 13.190566793051676 usec\nrounds: 79911"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 410600.21868650906,
            "unit": "iter/sec",
            "range": "stddev: 1.9755053847703496e-7",
            "extra": "mean: 2.435459004865978 usec\nrounds: 42366"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 395831.3942504425,
            "unit": "iter/sec",
            "range": "stddev: 3.372663973495316e-7",
            "extra": "mean: 2.526328165287717 usec\nrounds: 136931"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 355327.0446583379,
            "unit": "iter/sec",
            "range": "stddev: 4.2190595541291854e-7",
            "extra": "mean: 2.8143087193420433 usec\nrounds: 185186"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 212970.70103739144,
            "unit": "iter/sec",
            "range": "stddev: 5.560301400365101e-7",
            "extra": "mean: 4.695481562153609 usec\nrounds: 110425"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39627.18869700996,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019068668493257331",
            "extra": "mean: 25.235199187255347 usec\nrounds: 41092"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 2954059.697200637,
            "unit": "iter/sec",
            "range": "stddev: 4.248646910173258e-8",
            "extra": "mean: 338.5171941337653 nsec\nrounds: 196117"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 2795676.8124361755,
            "unit": "iter/sec",
            "range": "stddev: 4.562218089007837e-8",
            "extra": "mean: 357.69513684550407 nsec\nrounds: 197629"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 1882841.2704236761,
            "unit": "iter/sec",
            "range": "stddev: 6.609736743105596e-8",
            "extra": "mean: 531.1122162597011 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 485954.5128287014,
            "unit": "iter/sec",
            "range": "stddev: 1.5533943587185647e-7",
            "extra": "mean: 2.057805769060717 usec\nrounds: 49956"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1539994.5874606534,
            "unit": "iter/sec",
            "range": "stddev: 8.129548432004942e-8",
            "extra": "mean: 649.3529315898235 nsec\nrounds: 158958"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1528558.6280784959,
            "unit": "iter/sec",
            "range": "stddev: 8.054696444201065e-8",
            "extra": "mean: 654.2110859411782 nsec\nrounds: 157208"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1344904.8455767801,
            "unit": "iter/sec",
            "range": "stddev: 9.055752808335372e-8",
            "extra": "mean: 743.5470273520475 nsec\nrounds: 138428"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 445090.53106740396,
            "unit": "iter/sec",
            "range": "stddev: 1.71336778545445e-7",
            "extra": "mean: 2.246733934334276 usec\nrounds: 45370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2390018.899890419,
            "unit": "iter/sec",
            "range": "stddev: 4.884085077262048e-8",
            "extra": "mean: 418.4067331207641 nsec\nrounds: 187266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 500155.0379946105,
            "unit": "iter/sec",
            "range": "stddev: 1.6018052308330282e-7",
            "extra": "mean: 1.9993800402561943 usec\nrounds: 51664"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2552920.754049092,
            "unit": "iter/sec",
            "range": "stddev: 4.845758436269986e-8",
            "extra": "mean: 391.70820262006265 nsec\nrounds: 189072"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2438376.712358862,
            "unit": "iter/sec",
            "range": "stddev: 5.230079724823021e-8",
            "extra": "mean: 410.108903571618 nsec\nrounds: 192716"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1615852.1375626312,
            "unit": "iter/sec",
            "range": "stddev: 7.40295387370886e-8",
            "extra": "mean: 618.8685070580875 nsec\nrounds: 168606"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 472619.8314757848,
            "unit": "iter/sec",
            "range": "stddev: 1.6091988585680341e-7",
            "extra": "mean: 2.115865508388471 usec\nrounds: 48406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 47514.0990540173,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020334271689802045",
            "extra": "mean: 21.04638454499855 usec\nrounds: 49266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1209841.7090946476,
            "unit": "iter/sec",
            "range": "stddev: 9.146692646194732e-8",
            "extra": "mean: 826.5544099552711 nsec\nrounds: 125251"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1184508.1987947908,
            "unit": "iter/sec",
            "range": "stddev: 8.883176224172488e-8",
            "extra": "mean: 844.2322315856554 nsec\nrounds: 122926"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 887961.3903512104,
            "unit": "iter/sec",
            "range": "stddev: 1.1089546519782965e-7",
            "extra": "mean: 1.1261750914693052 usec\nrounds: 91567"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 351388.3335204364,
            "unit": "iter/sec",
            "range": "stddev: 4.4132854890746745e-7",
            "extra": "mean: 2.8458543002306045 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 43286.3212661244,
            "unit": "iter/sec",
            "range": "stddev: 0.0000018706442179644058",
            "extra": "mean: 23.10198627996123 usec\nrounds: 44679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2399514.3922552005,
            "unit": "iter/sec",
            "range": "stddev: 5.186223444231291e-8",
            "extra": "mean: 416.7509906286511 nsec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2011542.840254387,
            "unit": "iter/sec",
            "range": "stddev: 5.913532842719386e-8",
            "extra": "mean: 497.13084901217235 nsec\nrounds: 190477"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 745081.5304246258,
            "unit": "iter/sec",
            "range": "stddev: 1.0272475415806427e-7",
            "extra": "mean: 1.3421350002195025 usec\nrounds: 76191"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 106281.74425729092,
            "unit": "iter/sec",
            "range": "stddev: 9.536754339941406e-7",
            "extra": "mean: 9.408953597704999 usec\nrounds: 108378"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 357061.1432855068,
            "unit": "iter/sec",
            "range": "stddev: 6.623878338630673e-7",
            "extra": "mean: 2.800640783252066 usec\nrounds: 188680"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 358643.2863200082,
            "unit": "iter/sec",
            "range": "stddev: 4.787300000806752e-7",
            "extra": "mean: 2.788285848763179 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 289836.52014616085,
            "unit": "iter/sec",
            "range": "stddev: 5.287774218334555e-7",
            "extra": "mean: 3.4502208330948516 usec\nrounds: 151447"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 130528.19973851363,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011241460074139957",
            "extra": "mean: 7.66117974509182 usec\nrounds: 135631"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2801748.301184286,
            "unit": "iter/sec",
            "range": "stddev: 5.68416490794534e-7",
            "extra": "mean: 356.9199986941385 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2945594.833398252,
            "unit": "iter/sec",
            "range": "stddev: 1.580952631864819e-7",
            "extra": "mean: 339.4900033981685 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2050524.9391969894,
            "unit": "iter/sec",
            "range": "stddev: 1.5870555486749175e-7",
            "extra": "mean: 487.67999885512836 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 490188.8705390009,
            "unit": "iter/sec",
            "range": "stddev: 3.4125254674310956e-7",
            "extra": "mean: 2.04002999680597 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 56946.53802273116,
            "unit": "iter/sec",
            "range": "stddev: 0.0000019070244869039515",
            "extra": "mean: 17.560329999355417 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 885951.4677532222,
            "unit": "iter/sec",
            "range": "stddev: 8.967010812670361e-7",
            "extra": "mean: 1.1287299997775335 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 978042.9372329125,
            "unit": "iter/sec",
            "range": "stddev: 4.080553378371811e-7",
            "extra": "mean: 1.022449998799857 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 807911.0658302791,
            "unit": "iter/sec",
            "range": "stddev: 4.0901379342707886e-7",
            "extra": "mean: 1.2377599989576993 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 418978.0288258008,
            "unit": "iter/sec",
            "range": "stddev: 6.106309092599841e-7",
            "extra": "mean: 2.386759999808419 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 58360.62665525271,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014395253947906709",
            "extra": "mean: 17.13483999935761 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1505058.3471429516,
            "unit": "iter/sec",
            "range": "stddev: 7.857096270010871e-8",
            "extra": "mean: 664.4260681974587 nsec\nrounds: 155473"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 352810.8486543932,
            "unit": "iter/sec",
            "range": "stddev: 3.5985459358206756e-7",
            "extra": "mean: 2.8343799625605643 usec\nrounds: 180148"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43216.76976214696,
            "unit": "iter/sec",
            "range": "stddev: 0.000001547468150626191",
            "extra": "mean: 23.13916577994424 usec\nrounds: 43799"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4391.425102281444,
            "unit": "iter/sec",
            "range": "stddev: 0.000006017037898047412",
            "extra": "mean: 227.71651040580826 usec\nrounds: 4469"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 204238.92581819493,
            "unit": "iter/sec",
            "range": "stddev: 6.214839384153474e-7",
            "extra": "mean: 4.896226299633787 usec\nrounds: 106180"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 130302.73565588202,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010434169274699141",
            "extra": "mean: 7.674435958435374 usec\nrounds: 136185"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31806.235951781244,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021588498374081177",
            "extra": "mean: 31.440375450776877 usec\nrounds: 32726"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3728.170722249866,
            "unit": "iter/sec",
            "range": "stddev: 0.000005209719619244189",
            "extra": "mean: 268.22805995228754 usec\nrounds: 3803"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 135988.17541724868,
            "unit": "iter/sec",
            "range": "stddev: 9.184059366955028e-7",
            "extra": "mean: 7.353580536923363 usec\nrounds: 140985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16463.08270984747,
            "unit": "iter/sec",
            "range": "stddev: 0.000003060990469965002",
            "extra": "mean: 60.741965379414964 usec\nrounds: 17013"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1578.2766132572299,
            "unit": "iter/sec",
            "range": "stddev: 0.000009100416410782037",
            "extra": "mean: 633.6024950253878 usec\nrounds: 1608"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1153077.1059449087,
            "unit": "iter/sec",
            "range": "stddev: 8.74266534250101e-8",
            "extra": "mean: 867.2446923491406 nsec\nrounds: 118414"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 102966.5195175091,
            "unit": "iter/sec",
            "range": "stddev: 9.201965433035073e-7",
            "extra": "mean: 9.71189474681577 usec\nrounds: 105175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17092.885673231704,
            "unit": "iter/sec",
            "range": "stddev: 0.000002517209970008356",
            "extra": "mean: 58.50387226108046 usec\nrounds: 17387"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2377.304585331718,
            "unit": "iter/sec",
            "range": "stddev: 0.000005539681841125557",
            "extra": "mean: 420.64445850570917 usec\nrounds: 2410"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 715448.329252499,
            "unit": "iter/sec",
            "range": "stddev: 1.3737284925701892e-7",
            "extra": "mean: 1.3977249776301661 usec\nrounds: 73714"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 101567.03296794814,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010205027473178524",
            "extra": "mean: 9.845714409276615 usec\nrounds: 105175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 22415.410204385556,
            "unit": "iter/sec",
            "range": "stddev: 0.000006768400320296191",
            "extra": "mean: 44.61216595555994 usec\nrounds: 23681"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 3596.502027114312,
            "unit": "iter/sec",
            "range": "stddev: 0.0004709442596735521",
            "extra": "mean: 278.0479456040678 usec\nrounds: 4596"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 1853337.812313981,
            "unit": "iter/sec",
            "range": "stddev: 2.275378435060942e-7",
            "extra": "mean: 539.5670413433207 nsec\nrounds: 194213"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 922453.5244066676,
            "unit": "iter/sec",
            "range": "stddev: 9.663785883134292e-8",
            "extra": "mean: 1.0840654553769609 usec\nrounds: 94527"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 351027.23362994986,
            "unit": "iter/sec",
            "range": "stddev: 3.492127866234947e-7",
            "extra": "mean: 2.8487818157556175 usec\nrounds: 179534"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 120689.16001983544,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010249764181938894",
            "extra": "mean: 8.28574827959403 usec\nrounds: 123077"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 876151.9324457848,
            "unit": "iter/sec",
            "range": "stddev: 1.6975605069798469e-7",
            "extra": "mean: 1.141354556176691 usec\nrounds: 90745"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 713920.9031437197,
            "unit": "iter/sec",
            "range": "stddev: 1.1871463333683817e-7",
            "extra": "mean: 1.4007153952160762 usec\nrounds: 73341"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 492170.03132901486,
            "unit": "iter/sec",
            "range": "stddev: 1.537639066074688e-7",
            "extra": "mean: 2.031818144838435 usec\nrounds: 50411"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 163058.85529161838,
            "unit": "iter/sec",
            "range": "stddev: 9.339056237751253e-7",
            "extra": "mean: 6.1327549381576105 usec\nrounds: 176992"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 4429900.397652032,
            "unit": "iter/sec",
            "range": "stddev: 1.376614174330485e-8",
            "extra": "mean: 225.73870973037134 nsec\nrounds: 46254"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 412107.0952243318,
            "unit": "iter/sec",
            "range": "stddev: 1.9859898641549938e-7",
            "extra": "mean: 2.4265537079764288 usec\nrounds: 42530"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54293.38552309914,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013568947793811285",
            "extra": "mean: 18.418449878660628 usec\nrounds: 56393"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4110.439611270271,
            "unit": "iter/sec",
            "range": "stddev: 0.000006366549927481262",
            "extra": "mean: 243.2829805498504 usec\nrounds: 4216"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 3577637.044053461,
            "unit": "iter/sec",
            "range": "stddev: 3.359388303673079e-8",
            "extra": "mean: 279.5141004205393 nsec\nrounds: 194970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 389427.80878321425,
            "unit": "iter/sec",
            "range": "stddev: 3.13352189759136e-7",
            "extra": "mean: 2.5678700325088397 usec\nrounds: 136352"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2102830.5942337983,
            "unit": "iter/sec",
            "range": "stddev: 6.090369960757859e-8",
            "extra": "mean: 475.54948208489935 nsec\nrounds: 197278"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 47599.89404791597,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021617286009476507",
            "extra": "mean: 21.008450123720017 usec\nrounds: 49633"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 109385.230804072,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011303545256858465",
            "extra": "mean: 9.14200201114147 usec\nrounds: 114864"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13259.27111418759,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032300290912204354",
            "extra": "mean: 75.41892698234274 usec\nrounds: 13709"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1265.814728850856,
            "unit": "iter/sec",
            "range": "stddev: 0.000016048830857561238",
            "extra": "mean: 790.0050277561784 usec\nrounds: 1297"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51055.105729984156,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020121254473800877",
            "extra": "mean: 19.58667964157618 usec\nrounds: 53206"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 48737.20212561447,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021167162024114184",
            "extra": "mean: 20.518206962775917 usec\nrounds: 51159"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35265.144980737525,
            "unit": "iter/sec",
            "range": "stddev: 0.000005953602155167984",
            "extra": "mean: 28.35661105451909 usec\nrounds: 37342"
          }
        ]
      },
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
          "id": "2e5736b1ee65daa6e0dc1d10c6a6dc96552f5001",
          "message": "Fixing test exclusion",
          "timestamp": "2025-10-24T01:09:28-05:00",
          "tree_id": "2eed4da926607afccb8cbbec56fc70f32027e11f",
          "url": "https://github.com/cachemcclure/astrora/commit/2e5736b1ee65daa6e0dc1d10c6a6dc96552f5001"
        },
        "date": 1761286490619,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5205251.004380679,
            "unit": "iter/sec",
            "range": "stddev: 4.062845322417082e-8",
            "extra": "mean: 192.11369425957008 nsec\nrounds: 54157"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4828526.758453437,
            "unit": "iter/sec",
            "range": "stddev: 1.4829328060074366e-8",
            "extra": "mean: 207.10250766431938 nsec\nrounds: 49584"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3136317.200711917,
            "unit": "iter/sec",
            "range": "stddev: 4.0165810496630084e-8",
            "extra": "mean: 318.8453003967159 nsec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 704546.8111751114,
            "unit": "iter/sec",
            "range": "stddev: 1.0416055481388709e-7",
            "extra": "mean: 1.4193521056920502 usec\nrounds: 72067"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 68590.51293972397,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014968656291948472",
            "extra": "mean: 14.579275721101267 usec\nrounds: 74151"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 397968.5993892489,
            "unit": "iter/sec",
            "range": "stddev: 3.0859442652075734e-7",
            "extra": "mean: 2.5127610608844138 usec\nrounds: 137685"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 401439.0022840365,
            "unit": "iter/sec",
            "range": "stddev: 3.556632758951902e-7",
            "extra": "mean: 2.4910384748626533 usec\nrounds: 139199"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 355377.0193797173,
            "unit": "iter/sec",
            "range": "stddev: 4.397050918256879e-7",
            "extra": "mean: 2.813912958540261 usec\nrounds: 188324"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 211043.53827166697,
            "unit": "iter/sec",
            "range": "stddev: 5.427397344697875e-7",
            "extra": "mean: 4.738358768003333 usec\nrounds: 110060"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39682.11905873271,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022219057724554573",
            "extra": "mean: 25.200267115773737 usec\nrounds: 41263"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3702190.9201182597,
            "unit": "iter/sec",
            "range": "stddev: 3.3714224341453836e-8",
            "extra": "mean: 270.1103269866861 nsec\nrounds: 199641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3474763.471156738,
            "unit": "iter/sec",
            "range": "stddev: 3.552521579886099e-8",
            "extra": "mean: 287.7893728021591 nsec\nrounds: 196890"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2224911.07664345,
            "unit": "iter/sec",
            "range": "stddev: 5.3994088448178184e-8",
            "extra": "mean: 449.45616501159566 nsec\nrounds: 189394"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 502742.60550681816,
            "unit": "iter/sec",
            "range": "stddev: 1.5181608806462574e-7",
            "extra": "mean: 1.9890894247800706 usec\nrounds: 51214"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1557425.6644547235,
            "unit": "iter/sec",
            "range": "stddev: 7.901748748673363e-8",
            "extra": "mean: 642.0852197463779 nsec\nrounds: 158705"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1538951.3548667675,
            "unit": "iter/sec",
            "range": "stddev: 7.944176823104347e-8",
            "extra": "mean: 649.7931184359157 nsec\nrounds: 158932"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1376299.270832565,
            "unit": "iter/sec",
            "range": "stddev: 8.609951589904676e-8",
            "extra": "mean: 726.5861583978595 nsec\nrounds: 141985"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 447650.4284093169,
            "unit": "iter/sec",
            "range": "stddev: 1.5760849890788895e-7",
            "extra": "mean: 2.233885944337005 usec\nrounds: 45704"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2746520.2845860138,
            "unit": "iter/sec",
            "range": "stddev: 4.46507594598981e-8",
            "extra": "mean: 364.0970742551523 nsec\nrounds: 186916"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 516454.3931751849,
            "unit": "iter/sec",
            "range": "stddev: 3.514958140390863e-7",
            "extra": "mean: 1.9362793950729151 usec\nrounds: 52701"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2964418.388919138,
            "unit": "iter/sec",
            "range": "stddev: 4.357707451830493e-8",
            "extra": "mean: 337.3342992804102 nsec\nrounds: 191242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2806202.3919582115,
            "unit": "iter/sec",
            "range": "stddev: 4.431437431507837e-8",
            "extra": "mean: 356.3534842910464 nsec\nrounds: 192679"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1801652.2276802578,
            "unit": "iter/sec",
            "range": "stddev: 6.911817353207062e-8",
            "extra": "mean: 555.0460763937432 nsec\nrounds: 184843"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 474919.8294591634,
            "unit": "iter/sec",
            "range": "stddev: 1.879265390052076e-7",
            "extra": "mean: 2.105618544373678 usec\nrounds: 48570"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 35935.232385796386,
            "unit": "iter/sec",
            "range": "stddev: 0.0000028739121327526566",
            "extra": "mean: 27.82784286084806 usec\nrounds: 37260"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1245010.2564841358,
            "unit": "iter/sec",
            "range": "stddev: 9.492700449859618e-8",
            "extra": "mean: 803.2062344803215 nsec\nrounds: 129618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1227137.5350867251,
            "unit": "iter/sec",
            "range": "stddev: 8.981250397012861e-8",
            "extra": "mean: 814.9045819295752 nsec\nrounds: 126824"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 925359.9378036907,
            "unit": "iter/sec",
            "range": "stddev: 1.0783139229614957e-7",
            "extra": "mean: 1.0806605723319682 usec\nrounds: 95329"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 360703.0317416904,
            "unit": "iter/sec",
            "range": "stddev: 4.402853783671568e-7",
            "extra": "mean: 2.772363723064374 usec\nrounds: 187618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 34341.19344653189,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021599314394946754",
            "extra": "mean: 29.119547098937232 usec\nrounds: 35712"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2828440.3334188955,
            "unit": "iter/sec",
            "range": "stddev: 4.4830405787344354e-8",
            "extra": "mean: 353.55173951693496 nsec\nrounds: 195351"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2294273.576327265,
            "unit": "iter/sec",
            "range": "stddev: 5.464476323674582e-8",
            "extra": "mean: 435.8678103248877 nsec\nrounds: 196503"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 790165.2994747431,
            "unit": "iter/sec",
            "range": "stddev: 1.0041519685390703e-7",
            "extra": "mean: 1.2655579796591374 usec\nrounds: 80561"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 107055.28536706584,
            "unit": "iter/sec",
            "range": "stddev: 9.399487096603096e-7",
            "extra": "mean: 9.340968048156144 usec\nrounds: 109446"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 370092.2547090434,
            "unit": "iter/sec",
            "range": "stddev: 7.666083389811248e-7",
            "extra": "mean: 2.7020289867621603 usec\nrounds: 194175"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 366652.00508912053,
            "unit": "iter/sec",
            "range": "stddev: 4.937080171641552e-7",
            "extra": "mean: 2.7273817846896384 usec\nrounds: 191608"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 302353.18153762183,
            "unit": "iter/sec",
            "range": "stddev: 4.600304213158326e-7",
            "extra": "mean: 3.307390366836837 usec\nrounds: 156937"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 131935.0770489287,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010329810439886914",
            "extra": "mean: 7.579485473974033 usec\nrounds: 137099"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2873067.8687225557,
            "unit": "iter/sec",
            "range": "stddev: 5.554646941950059e-7",
            "extra": "mean: 348.05999916898145 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2951158.330213893,
            "unit": "iter/sec",
            "range": "stddev: 1.6323864715258744e-7",
            "extra": "mean: 338.8499999346095 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 2089209.2343444433,
            "unit": "iter/sec",
            "range": "stddev: 1.5655104385919063e-7",
            "extra": "mean: 478.64999999092106 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 472012.04526412307,
            "unit": "iter/sec",
            "range": "stddev: 1.6294890273810028e-7",
            "extra": "mean: 2.118590002169185 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 50175.79088367294,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014615476380316089",
            "extra": "mean: 19.92992999987564 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 869860.2116796481,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013834966413664563",
            "extra": "mean: 1.1496100023578038 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 953225.2413936389,
            "unit": "iter/sec",
            "range": "stddev: 4.3138673128105056e-7",
            "extra": "mean: 1.0490699958154437 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 800826.4534652722,
            "unit": "iter/sec",
            "range": "stddev: 4.2363824007413785e-7",
            "extra": "mean: 1.2487099991176365 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 396913.6000960315,
            "unit": "iter/sec",
            "range": "stddev: 9.88724464584222e-7",
            "extra": "mean: 2.5194399984229676 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 50739.73459227768,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013390111371391792",
            "extra": "mean: 19.708419999346916 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1660772.0527639275,
            "unit": "iter/sec",
            "range": "stddev: 7.878768945720422e-8",
            "extra": "mean: 602.1295928816718 nsec\nrounds: 169463"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 360274.01906167244,
            "unit": "iter/sec",
            "range": "stddev: 3.482276711988517e-7",
            "extra": "mean: 2.7756650413051793 usec\nrounds: 184163"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43370.78006014688,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016288604163329678",
            "extra": "mean: 23.056998251200316 usec\nrounds: 44030"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4412.276200531755,
            "unit": "iter/sec",
            "range": "stddev: 0.000005599383861248077",
            "extra": "mean: 226.64039025469046 usec\nrounds: 4474"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 212864.6826493773,
            "unit": "iter/sec",
            "range": "stddev: 6.027826223819551e-7",
            "extra": "mean: 4.697820171733995 usec\nrounds: 110169"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 133270.03355894107,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010444261392995553",
            "extra": "mean: 7.503562303507127 usec\nrounds: 139005"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31933.372455705718,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020919138413346254",
            "extra": "mean: 31.315201718424333 usec\nrounds: 33051"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3702.5349806079007,
            "unit": "iter/sec",
            "range": "stddev: 0.00000978816170568459",
            "extra": "mean: 270.0852268074494 usec\nrounds: 3805"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 133908.92582235194,
            "unit": "iter/sec",
            "range": "stddev: 9.495113844397984e-7",
            "extra": "mean: 7.467762091726682 usec\nrounds: 141563"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16348.47375402057,
            "unit": "iter/sec",
            "range": "stddev: 0.000004080497561137301",
            "extra": "mean: 61.167789424628744 usec\nrounds: 17210"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1586.8516923430873,
            "unit": "iter/sec",
            "range": "stddev: 0.000013932744504123446",
            "extra": "mean: 630.178613934259 usec\nrounds: 1751"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1238308.2295052798,
            "unit": "iter/sec",
            "range": "stddev: 8.795237516829785e-8",
            "extra": "mean: 807.5533830535152 nsec\nrounds: 130311"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 104598.68647249643,
            "unit": "iter/sec",
            "range": "stddev: 9.814299494223985e-7",
            "extra": "mean: 9.560349500784064 usec\nrounds: 106758"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17045.09782547367,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026080435956099905",
            "extra": "mean: 58.667894443264125 usec\nrounds: 17384"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2349.6953979925993,
            "unit": "iter/sec",
            "range": "stddev: 0.0000061807779483201745",
            "extra": "mean: 425.58707858657925 usec\nrounds: 2405"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 724881.5852193737,
            "unit": "iter/sec",
            "range": "stddev: 1.1555178135504358e-7",
            "extra": "mean: 1.3795356653974953 usec\nrounds: 74212"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 103790.87079041691,
            "unit": "iter/sec",
            "range": "stddev: 0.00000109005704584063",
            "extra": "mean: 9.634758745008341 usec\nrounds: 107090"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21057.783088160555,
            "unit": "iter/sec",
            "range": "stddev: 0.000003359798717566982",
            "extra": "mean: 47.48837975077424 usec\nrounds: 22470"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4375.9109273873855,
            "unit": "iter/sec",
            "range": "stddev: 0.000027368026612347684",
            "extra": "mean: 228.5238471700439 usec\nrounds: 4541"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2165340.132527573,
            "unit": "iter/sec",
            "range": "stddev: 5.499023181285619e-8",
            "extra": "mean: 461.8212099697892 nsec\nrounds: 185874"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 990793.7831936564,
            "unit": "iter/sec",
            "range": "stddev: 9.482135279105707e-8",
            "extra": "mean: 1.0092917587519428 usec\nrounds: 101441"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 362463.1055743296,
            "unit": "iter/sec",
            "range": "stddev: 3.531777369771571e-7",
            "extra": "mean: 2.7589014843744746 usec\nrounds: 185874"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 120170.91529290026,
            "unit": "iter/sec",
            "range": "stddev: 9.772247353764674e-7",
            "extra": "mean: 8.321481096841412 usec\nrounds: 124767"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 898361.9620837182,
            "unit": "iter/sec",
            "range": "stddev: 1.1286852102681149e-7",
            "extra": "mean: 1.1131370674694676 usec\nrounds: 92507"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 724293.5802933496,
            "unit": "iter/sec",
            "range": "stddev: 1.5608363926450673e-7",
            "extra": "mean: 1.3806556170151092 usec\nrounds: 74488"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 498892.2201828677,
            "unit": "iter/sec",
            "range": "stddev: 1.4630383651005711e-7",
            "extra": "mean: 2.0044409584768856 usec\nrounds: 51081"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 168498.84428940938,
            "unit": "iter/sec",
            "range": "stddev: 9.284490912323949e-7",
            "extra": "mean: 5.934758806312198 usec\nrounds: 181489"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5568789.934446371,
            "unit": "iter/sec",
            "range": "stddev: 1.222356183991925e-8",
            "extra": "mean: 179.57222516404903 nsec\nrounds: 57007"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 413034.30711070745,
            "unit": "iter/sec",
            "range": "stddev: 1.775986403326351e-7",
            "extra": "mean: 2.4211063894311677 usec\nrounds: 42492"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54133.10229629097,
            "unit": "iter/sec",
            "range": "stddev: 0.000001302559528920681",
            "extra": "mean: 18.47298524526862 usec\nrounds: 55575"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4109.590226703434,
            "unit": "iter/sec",
            "range": "stddev: 0.00000645980714499709",
            "extra": "mean: 243.33326313221357 usec\nrounds: 4207"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4368716.270971158,
            "unit": "iter/sec",
            "range": "stddev: 1.3857976086218114e-8",
            "extra": "mean: 228.90019355220747 nsec\nrounds: 44639"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 392678.56549702457,
            "unit": "iter/sec",
            "range": "stddev: 3.006731528207917e-7",
            "extra": "mean: 2.546612134874888 usec\nrounds: 136538"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2540816.7071586438,
            "unit": "iter/sec",
            "range": "stddev: 4.751580251572824e-8",
            "extra": "mean: 393.5742382292123 nsec\nrounds: 186220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48748.46496159045,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020278649841757997",
            "extra": "mean: 20.513466440182537 usec\nrounds: 50641"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 109105.6509736496,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012731044659861614",
            "extra": "mean: 9.165428106391234 usec\nrounds: 114864"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13249.391709439571,
            "unit": "iter/sec",
            "range": "stddev: 0.0000035098788616442998",
            "extra": "mean: 75.47516308145278 usec\nrounds: 13760"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1244.163883915103,
            "unit": "iter/sec",
            "range": "stddev: 0.000016573385211061497",
            "extra": "mean: 803.7526349448641 usec\nrounds: 1282"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 51213.40787886064,
            "unit": "iter/sec",
            "range": "stddev: 0.000001911148398655859",
            "extra": "mean: 19.52613663916652 usec\nrounds: 53462"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49160.57660988104,
            "unit": "iter/sec",
            "range": "stddev: 0.000002153501112809617",
            "extra": "mean: 20.341502662501416 usec\nrounds: 51267"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 35344.644699738,
            "unit": "iter/sec",
            "range": "stddev: 0.0000031054134262768757",
            "extra": "mean: 28.292829323798887 usec\nrounds: 37328"
          }
        ]
      },
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
          "id": "609b1f1bfbb8bc59ae00e2dc95ca498cb34e364e",
          "message": "Fixing python test issue",
          "timestamp": "2025-10-24T01:16:50-05:00",
          "tree_id": "58441d80376e32ed017b619ada6daeaccf4d2ea5",
          "url": "https://github.com/cachemcclure/astrora/commit/609b1f1bfbb8bc59ae00e2dc95ca498cb34e364e"
        },
        "date": 1761286939222,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[tiny]",
            "value": 5209907.914128553,
            "unit": "iter/sec",
            "range": "stddev: 1.7731577350375966e-8",
            "extra": "mean: 191.94197219651272 nsec\nrounds: 53519"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[small]",
            "value": 4862865.574171188,
            "unit": "iter/sec",
            "range": "stddev: 1.3919395111558678e-8",
            "extra": "mean: 205.64006648911493 nsec\nrounds: 49933"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[medium]",
            "value": 3127181.5089455843,
            "unit": "iter/sec",
            "range": "stddev: 5.038690686478259e-8",
            "extra": "mean: 319.77676931719145 nsec\nrounds: 189430"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[large]",
            "value": 710237.3618830079,
            "unit": "iter/sec",
            "range": "stddev: 1.0967278623216295e-7",
            "extra": "mean: 1.4079799988958994 usec\nrounds: 72281"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_rust_vs_numpy[very_large]",
            "value": 69448.91460045322,
            "unit": "iter/sec",
            "range": "stddev: 0.000001384248764753117",
            "extra": "mean: 14.399073128112992 usec\nrounds: 78971"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[tiny]",
            "value": 389630.60351127695,
            "unit": "iter/sec",
            "range": "stddev: 3.6856125879238844e-7",
            "extra": "mean: 2.5665335088881194 usec\nrounds: 137476"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[small]",
            "value": 391554.96654819354,
            "unit": "iter/sec",
            "range": "stddev: 8.338143265116186e-7",
            "extra": "mean: 2.5539198463388746 usec\nrounds: 136912"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[medium]",
            "value": 341839.43039821705,
            "unit": "iter/sec",
            "range": "stddev: 6.036354978570391e-7",
            "extra": "mean: 2.925350065190185 usec\nrounds: 181819"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[large]",
            "value": 209447.850964189,
            "unit": "iter/sec",
            "range": "stddev: 6.6144192447895e-7",
            "extra": "mean: 4.7744581546027804 usec\nrounds: 110048"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_sum_array_numpy_baseline[very_large]",
            "value": 39253.86351165027,
            "unit": "iter/sec",
            "range": "stddev: 0.000002074384048892417",
            "extra": "mean: 25.47519939542275 usec\nrounds: 41024"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[tiny]",
            "value": 3657618.9390042243,
            "unit": "iter/sec",
            "range": "stddev: 3.682293382234651e-8",
            "extra": "mean: 273.40190891316473 nsec\nrounds: 199601"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[small]",
            "value": 3430098.973333736,
            "unit": "iter/sec",
            "range": "stddev: 3.8802736417047394e-8",
            "extra": "mean: 291.5367771523753 nsec\nrounds: 197239"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[medium]",
            "value": 2199173.1542087053,
            "unit": "iter/sec",
            "range": "stddev: 5.902179478633548e-8",
            "extra": "mean: 454.71635468368396 nsec\nrounds: 189754"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_rust_vs_numpy[large]",
            "value": 503767.58609822945,
            "unit": "iter/sec",
            "range": "stddev: 3.194489020188277e-7",
            "extra": "mean: 1.9850423639702162 usec\nrounds: 52285"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[tiny]",
            "value": 1549019.8298287047,
            "unit": "iter/sec",
            "range": "stddev: 8.075442865406399e-8",
            "extra": "mean: 645.5695277384928 nsec\nrounds: 159975"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[small]",
            "value": 1525355.030519394,
            "unit": "iter/sec",
            "range": "stddev: 9.00001814914466e-8",
            "extra": "mean: 655.5850801891406 nsec\nrounds: 156691"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[medium]",
            "value": 1315682.8485791744,
            "unit": "iter/sec",
            "range": "stddev: 9.502918397646296e-8",
            "extra": "mean: 760.061591651716 nsec\nrounds: 135796"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_dot_product_numpy_baseline[large]",
            "value": 445388.839378567,
            "unit": "iter/sec",
            "range": "stddev: 1.666041236616875e-7",
            "extra": "mean: 2.245229138195871 usec\nrounds: 45370"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_rust",
            "value": 2704291.29759337,
            "unit": "iter/sec",
            "range": "stddev: 4.627848762581974e-8",
            "extra": "mean: 369.78264911399197 nsec\nrounds: 198847"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestReadOnlyOperationsOverhead::test_vector_magnitude_numpy_baseline",
            "value": 523348.0007076234,
            "unit": "iter/sec",
            "range": "stddev: 1.8721252555629643e-7",
            "extra": "mean: 1.9107744725266216 usec\nrounds: 53605"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[tiny]",
            "value": 2954233.6650432865,
            "unit": "iter/sec",
            "range": "stddev: 4.337731261135402e-8",
            "extra": "mean: 338.49725965575965 nsec\nrounds: 191571"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[small]",
            "value": 2804555.7327702413,
            "unit": "iter/sec",
            "range": "stddev: 9.30715060435821e-8",
            "extra": "mean: 356.5627127018208 nsec\nrounds: 190115"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[medium]",
            "value": 1791969.2516439192,
            "unit": "iter/sec",
            "range": "stddev: 7.355060162265136e-8",
            "extra": "mean: 558.0452896067598 nsec\nrounds: 191205"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[large]",
            "value": 493769.4303447782,
            "unit": "iter/sec",
            "range": "stddev: 1.7964159276112664e-7",
            "extra": "mean: 2.025236757370259 usec\nrounds: 50564"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_rust[very_large]",
            "value": 46439.12086496008,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022454321223713494",
            "extra": "mean: 21.53356871048208 usec\nrounds: 48406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[tiny]",
            "value": 1247500.2021003917,
            "unit": "iter/sec",
            "range": "stddev: 1.0235970236755231e-7",
            "extra": "mean: 801.6030765496713 nsec\nrounds: 128618"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[small]",
            "value": 1228992.8103913504,
            "unit": "iter/sec",
            "range": "stddev: 1.0036639766138824e-7",
            "extra": "mean: 813.6744100899051 nsec\nrounds: 127308"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[medium]",
            "value": 917631.1644024535,
            "unit": "iter/sec",
            "range": "stddev: 1.6697258990375775e-7",
            "extra": "mean: 1.0897624653487368 usec\nrounds: 95248"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[large]",
            "value": 360633.58234248846,
            "unit": "iter/sec",
            "range": "stddev: 4.4193318802568374e-7",
            "extra": "mean: 2.7728976139840316 usec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_multiply_scalar_numpy_baseline[very_large]",
            "value": 41811.36238399135,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023988344428027357",
            "extra": "mean: 23.91694369621589 usec\nrounds: 44402"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[tiny]",
            "value": 2824938.6813665926,
            "unit": "iter/sec",
            "range": "stddev: 7.627015445861036e-8",
            "extra": "mean: 353.989984489235 nsec\nrounds: 193088"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[small]",
            "value": 2292099.781840067,
            "unit": "iter/sec",
            "range": "stddev: 5.3016011848415136e-8",
            "extra": "mean: 436.2811810911664 nsec\nrounds: 180832"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[medium]",
            "value": 788937.0111923304,
            "unit": "iter/sec",
            "range": "stddev: 1.0710671366549781e-7",
            "extra": "mean: 1.267528314445169 usec\nrounds: 80496"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_rust[large]",
            "value": 106659.90901080349,
            "unit": "iter/sec",
            "range": "stddev: 9.898706574260896e-7",
            "extra": "mean: 9.375593972227286 usec\nrounds: 108862"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[tiny]",
            "value": 364698.1529717722,
            "unit": "iter/sec",
            "range": "stddev: 4.513682347724595e-7",
            "extra": "mean: 2.7419935962148965 usec\nrounds: 192308"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[small]",
            "value": 358578.42014786927,
            "unit": "iter/sec",
            "range": "stddev: 4.772467081479231e-7",
            "extra": "mean: 2.7887902445094817 usec\nrounds: 189790"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[medium]",
            "value": 297436.0254725432,
            "unit": "iter/sec",
            "range": "stddev: 5.041853334472926e-7",
            "extra": "mean: 3.362067518254649 usec\nrounds: 156691"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestArrayReturnOverhead::test_normalize_vector_numpy_baseline[large]",
            "value": 119145.33252256375,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012936082456213202",
            "extra": "mean: 8.39311099165903 usec\nrounds: 125550"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[tiny]",
            "value": 2727471.085137981,
            "unit": "iter/sec",
            "range": "stddev: 5.327251937737252e-7",
            "extra": "mean: 366.6400004931347 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[small]",
            "value": 2736352.4549342045,
            "unit": "iter/sec",
            "range": "stddev: 2.1831833874304114e-7",
            "extra": "mean: 365.4499982985726 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[medium]",
            "value": 1885974.0155908978,
            "unit": "iter/sec",
            "range": "stddev: 1.693913553753258e-7",
            "extra": "mean: 530.2299987874903 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[large]",
            "value": 352830.7613089249,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010626950149939893",
            "extra": "mean: 2.834219999101606 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_rust[very_large]",
            "value": 33577.97123872408,
            "unit": "iter/sec",
            "range": "stddev: 0.000004446802167399343",
            "extra": "mean: 29.781430000355158 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[tiny]",
            "value": 871793.9789047301,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014154793220618347",
            "extra": "mean: 1.1470599983454122 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[small]",
            "value": 944688.4883273419,
            "unit": "iter/sec",
            "range": "stddev: 5.997815648994057e-7",
            "extra": "mean: 1.0585500007209703 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[medium]",
            "value": 792229.8095389663,
            "unit": "iter/sec",
            "range": "stddev: 4.526435159579714e-7",
            "extra": "mean: 1.2622600007716755 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[large]",
            "value": 356184.97391058557,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022856683794274824",
            "extra": "mean: 2.8075300005525605 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestInPlaceOperationsOverhead::test_multiply_scalar_inplace_numpy_baseline[very_large]",
            "value": 34899.32417450355,
            "unit": "iter/sec",
            "range": "stddev: 0.000005229767016274891",
            "extra": "mean: 28.653850000068815 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10]",
            "value": 1632991.00739581,
            "unit": "iter/sec",
            "range": "stddev: 8.899400746777293e-8",
            "extra": "mean: 612.3732436192453 nsec\nrounds: 171204"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[100]",
            "value": 361229.29775309016,
            "unit": "iter/sec",
            "range": "stddev: 3.63190098830444e-7",
            "extra": "mean: 2.7683247350648914 usec\nrounds: 184843"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[1000]",
            "value": 43252.804663704424,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016629443253641362",
            "extra": "mean: 23.11988801131201 usec\nrounds: 43933"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_rust[10000]",
            "value": 4383.594758618319,
            "unit": "iter/sec",
            "range": "stddev: 0.000007279218941841385",
            "extra": "mean: 228.12327668609439 usec\nrounds: 4478"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10]",
            "value": 203266.5743730096,
            "unit": "iter/sec",
            "range": "stddev: 6.538141698277879e-7",
            "extra": "mean: 4.919648019279963 usec\nrounds: 108496"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[100]",
            "value": 131695.45313821617,
            "unit": "iter/sec",
            "range": "stddev: 0.000001293699788550212",
            "extra": "mean: 7.593276579947574 usec\nrounds: 139005"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[1000]",
            "value": 31676.470514131495,
            "unit": "iter/sec",
            "range": "stddev: 0.0000020987232537964946",
            "extra": "mean: 31.569173704307754 usec\nrounds: 32705"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_batch_normalize_numpy_baseline[10000]",
            "value": 3652.5772110545013,
            "unit": "iter/sec",
            "range": "stddev: 0.00002029134687335193",
            "extra": "mean: 273.7792912285349 usec\nrounds: 3808"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[10]",
            "value": 133832.45114196787,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015767355318747272",
            "extra": "mean: 7.472029328217354 usec\nrounds: 142593"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[100]",
            "value": 16694.815453350486,
            "unit": "iter/sec",
            "range": "stddev: 0.000004604384387774975",
            "extra": "mean: 59.898835227873676 usec\nrounds: 17242"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBatchOperationsScaling::test_sequential_normalize_rust[1000]",
            "value": 1604.7241357552932,
            "unit": "iter/sec",
            "range": "stddev: 0.000015158030960613765",
            "extra": "mean: 623.160067028799 usec\nrounds: 1656"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[10]",
            "value": 1266409.311923426,
            "unit": "iter/sec",
            "range": "stddev: 9.204960006397243e-8",
            "extra": "mean: 789.6341179623894 nsec\nrounds: 141383"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[50]",
            "value": 101442.12170762851,
            "unit": "iter/sec",
            "range": "stddev: 0.000001029345102641808",
            "extra": "mean: 9.85783797860765 usec\nrounds: 104406"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[100]",
            "value": 17098.143273785357,
            "unit": "iter/sec",
            "range": "stddev: 0.0000030240425229332326",
            "extra": "mean: 58.48588258897014 usec\nrounds: 17460"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_rust[200]",
            "value": 2347.0590498150964,
            "unit": "iter/sec",
            "range": "stddev: 0.0000059284195981344885",
            "extra": "mean: 426.06512182928714 usec\nrounds: 2405"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[10]",
            "value": 728114.3208494112,
            "unit": "iter/sec",
            "range": "stddev: 1.4382821887130884e-7",
            "extra": "mean: 1.373410701266558 usec\nrounds: 74935"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[50]",
            "value": 102631.20402925486,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011522045896721818",
            "extra": "mean: 9.743625337523582 usec\nrounds: 106293"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[100]",
            "value": 21011.80446414373,
            "unit": "iter/sec",
            "range": "stddev: 0.000003257908600443481",
            "extra": "mean: 47.592295164676706 usec\nrounds: 22025"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_multiply_numpy_baseline[200]",
            "value": 4265.199127745499,
            "unit": "iter/sec",
            "range": "stddev: 0.00002484560960291581",
            "extra": "mean: 234.45564205781417 usec\nrounds: 4470"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[10]",
            "value": 2176549.9224071116,
            "unit": "iter/sec",
            "range": "stddev: 6.233346936731895e-8",
            "extra": "mean: 459.44271238863587 nsec\nrounds: 188006"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[50]",
            "value": 955720.3297100294,
            "unit": "iter/sec",
            "range": "stddev: 1.0578480822885554e-7",
            "extra": "mean: 1.0463312005755991 usec\nrounds: 100011"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[100]",
            "value": 360042.46157231345,
            "unit": "iter/sec",
            "range": "stddev: 3.7003050568115056e-7",
            "extra": "mean: 2.7774501808286107 usec\nrounds: 185220"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_rust[200]",
            "value": 120443.14694415698,
            "unit": "iter/sec",
            "range": "stddev: 0.0000010759726561395233",
            "extra": "mean: 8.30267246723175 usec\nrounds: 125392"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[10]",
            "value": 908441.0096153398,
            "unit": "iter/sec",
            "range": "stddev: 1.119002643151163e-7",
            "extra": "mean: 1.1007869409411797 usec\nrounds: 93284"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[50]",
            "value": 727036.1731252134,
            "unit": "iter/sec",
            "range": "stddev: 1.285543149951475e-7",
            "extra": "mean: 1.3754473807010672 usec\nrounds: 74600"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[100]",
            "value": 454692.04663473973,
            "unit": "iter/sec",
            "range": "stddev: 1.7344361688295862e-7",
            "extra": "mean: 2.199290723031523 usec\nrounds: 46664"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestMatrixOperationsOverhead::test_matrix_vector_multiply_numpy_baseline[200]",
            "value": 160768.24955466046,
            "unit": "iter/sec",
            "range": "stddev: 0.000001078713850632136",
            "extra": "mean: 6.220133656801461 usec\nrounds: 180148"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_rust_call",
            "value": 5447114.214490801,
            "unit": "iter/sec",
            "range": "stddev: 1.3133415184314215e-8",
            "extra": "mean: 183.5834463209466 nsec\nrounds: 56231"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_minimal_numpy_call",
            "value": 392042.1112559098,
            "unit": "iter/sec",
            "range": "stddev: 6.061070150067877e-7",
            "extra": "mean: 2.5507463899643446 usec\nrounds: 138065"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_rust",
            "value": 54214.503455287966,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016671492249525552",
            "extra": "mean: 18.445248711439817 usec\nrounds: 56266"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_repeated_tiny_calls_numpy",
            "value": 4048.592248890426,
            "unit": "iter/sec",
            "range": "stddev: 0.000008867758229924016",
            "extra": "mean: 246.99943548873412 usec\nrounds: 4131"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_rust",
            "value": 4317946.479080586,
            "unit": "iter/sec",
            "range": "stddev: 3.1053460524092565e-8",
            "extra": "mean: 231.59156901197616 nsec\nrounds: 44920"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestBoundaryCrossingOverhead::test_single_large_call_numpy",
            "value": 371762.22059066774,
            "unit": "iter/sec",
            "range": "stddev: 4.694837166844434e-7",
            "extra": "mean: 2.68989139996842 usec\nrounds: 198453"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_rust",
            "value": 2513269.8716792553,
            "unit": "iter/sec",
            "range": "stddev: 4.9785166739705044e-8",
            "extra": "mean: 397.88803075564914 nsec\nrounds: 187970"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_cross_product_numpy_baseline",
            "value": 48456.19686021921,
            "unit": "iter/sec",
            "range": "stddev: 0.0000026814582048354215",
            "extra": "mean: 20.63719533921912 usec\nrounds: 50850"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[10]",
            "value": 110887.22395108681,
            "unit": "iter/sec",
            "range": "stddev: 0.0000013421508413457511",
            "extra": "mean: 9.018171475201756 usec\nrounds: 117014"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[100]",
            "value": 13340.846227754368,
            "unit": "iter/sec",
            "range": "stddev: 0.000003571721107174711",
            "extra": "mean: 74.95776376760828 usec\nrounds: 13728"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_simulation[1000]",
            "value": 1276.5142047876802,
            "unit": "iter/sec",
            "range": "stddev: 0.00004729242279229849",
            "extra": "mean: 783.3833703137897 usec\nrounds: 1307"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[10]",
            "value": 50947.68302295808,
            "unit": "iter/sec",
            "range": "stddev: 0.0000021688892630099143",
            "extra": "mean: 19.627977970055664 usec\nrounds: 53291"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[100]",
            "value": 49131.62366207217,
            "unit": "iter/sec",
            "range": "stddev: 0.0000023559341244939162",
            "extra": "mean: 20.353489778355602 usec\nrounds: 52340"
          },
          {
            "name": "tests/benchmark_numpy_overhead.py::TestCrossProductOperations::test_batch_cross_product_numpy_baseline[1000]",
            "value": 34661.74030705055,
            "unit": "iter/sec",
            "range": "stddev: 0.0000034322180974261336",
            "extra": "mean: 28.85025365551509 usec\nrounds: 36656"
          }
        ]
      }
    ]
  }
}