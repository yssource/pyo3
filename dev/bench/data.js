window.BENCHMARK_DATA = {
  "lastUpdate": 1689729844284,
  "repoUrl": "https://github.com/yssource/pyo3",
  "entries": {
    "pyo3-bench": [
      {
        "commit": {
          "author": {
            "email": "1939362+davidhewitt@users.noreply.github.com",
            "name": "David Hewitt",
            "username": "davidhewitt"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "421e13a89c12f27a1b75a4141a4c44a172930817",
          "message": "Merge pull request #3323 from davidhewitt/pyerr-simplification\n\nmerge PyErr internal states for simplicity",
          "timestamp": "2023-07-17T21:46:39Z",
          "tree_id": "18411e766bcf40ca8c3473dcf98aa60ab671f1d1",
          "url": "https://github.com/yssource/pyo3/commit/421e13a89c12f27a1b75a4141a4c44a172930817"
        },
        "date": 1689729834815,
        "tool": "cargo",
        "benches": [
          {
            "name": "call_0",
            "value": 71925,
            "range": "± 4690",
            "unit": "ns/iter"
          },
          {
            "name": "call_method_0",
            "value": 211783,
            "range": "± 16235",
            "unit": "ns/iter"
          },
          {
            "name": "iter_dict",
            "value": 3825924,
            "range": "± 163039",
            "unit": "ns/iter"
          },
          {
            "name": "dict_new",
            "value": 5796400,
            "range": "± 329986",
            "unit": "ns/iter"
          },
          {
            "name": "dict_get_item",
            "value": 4174376,
            "range": "± 187759",
            "unit": "ns/iter"
          },
          {
            "name": "extract_hashmap",
            "value": 12447765,
            "range": "± 1371516",
            "unit": "ns/iter"
          },
          {
            "name": "extract_btreemap",
            "value": 17219493,
            "range": "± 1067577",
            "unit": "ns/iter"
          },
          {
            "name": "extract_hashbrown_map",
            "value": 11349833,
            "range": "± 1738371",
            "unit": "ns/iter"
          },
          {
            "name": "mapping_from_dict",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "clean_gilpool_new",
            "value": 24,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "clean_acquire_gil",
            "value": 129,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "dirty_acquire_gil",
            "value": 134,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iter_list",
            "value": 2417769,
            "range": "± 92609",
            "unit": "ns/iter"
          },
          {
            "name": "list_new",
            "value": 1620789,
            "range": "± 222184",
            "unit": "ns/iter"
          },
          {
            "name": "list_get_item",
            "value": 1437014,
            "range": "± 72731",
            "unit": "ns/iter"
          },
          {
            "name": "list_get_item_unchecked",
            "value": 1318339,
            "range": "± 65620",
            "unit": "ns/iter"
          },
          {
            "name": "sequence_from_list",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "first_time_init",
            "value": 4918,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "drop_many_objects",
            "value": 4863,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "set_new",
            "value": 2730963,
            "range": "± 201625",
            "unit": "ns/iter"
          },
          {
            "name": "iter_set",
            "value": 3279382,
            "range": "± 188692",
            "unit": "ns/iter"
          },
          {
            "name": "extract_hashset",
            "value": 13388331,
            "range": "± 871861",
            "unit": "ns/iter"
          },
          {
            "name": "extract_btreeset",
            "value": 5245773,
            "range": "± 351415",
            "unit": "ns/iter"
          },
          {
            "name": "extract_hashbrown_set",
            "value": 9828117,
            "range": "± 937879",
            "unit": "ns/iter"
          },
          {
            "name": "iter_tuple",
            "value": 1783996,
            "range": "± 67227",
            "unit": "ns/iter"
          },
          {
            "name": "tuple_new",
            "value": 1564588,
            "range": "± 91945",
            "unit": "ns/iter"
          },
          {
            "name": "tuple_get_item",
            "value": 1167308,
            "range": "± 72829",
            "unit": "ns/iter"
          },
          {
            "name": "tuple_get_item_unchecked",
            "value": 1022824,
            "range": "± 35626",
            "unit": "ns/iter"
          },
          {
            "name": "sequence_from_tuple",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "tuple_new_list",
            "value": 346052,
            "range": "± 18643",
            "unit": "ns/iter"
          },
          {
            "name": "tuple_to_list",
            "value": 257390,
            "range": "± 10676",
            "unit": "ns/iter"
          },
          {
            "name": "tuple_into_py",
            "value": 104,
            "range": "± 5",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}