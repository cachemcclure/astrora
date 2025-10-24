window.BENCHMARK_DATA = {
  "lastUpdate": 1761283534465,
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
      }
    ]
  }
}