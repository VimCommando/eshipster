use serde_json::{json, Value};
use std::cell::LazyCell;

pub const INDEX_TEMPLATE: LazyCell<Value> = LazyCell::new(|| {
    json!({
      "index_patterns": [
        "eshipster-shards"
      ],
      "template": {
        "settings": {
          "number_of_shards": 1,
          "number_of_replicas": 1
        },
        "mappings": {
          "dynamic_templates": [
            {
              "strings_as_keywords": {
                "mapping": {
                  "type": "keyword"
                },
                "match_mapping_type": "string"
              }
            }
          ],
          "properties": {
            "data_stream": {
              "properties": {
                "dataset": {
                  "type": "constant_keyword"
                },
                "namespace": {
                  "type": "constant_keyword"
                },
                "type": {
                  "type": "constant_keyword"
                }
              }
            },
            "index": {
              "properties": {
                "name": {
                  "type": "keyword",
                  "ignore_above": 256
                },
                "uuid": {
                  "type": "keyword",
                  "ignore_above": 256
                }
              }
            },
            "shard": {
              "properties": {
                "node": {
                  "type": "keyword",
                  "ignore_above": 256
                },
                "number": {
                  "type": "long"
                },
                "primary": {
                  "type": "boolean"
                },
                "state": {
                  "type": "keyword",
                  "ignore_above": 256
                }
              }
            },
            "stats": {
              "properties": {
                "docs": {
                  "properties": {
                    "count": {
                      "type": "long"
                    },
                    "deleted": {
                      "type": "long"
                    },
                    "total_size_in_bytes": {
                      "type": "long"
                    }
                  }
                },
                "indexing": {
                  "properties": {
                    "delete_current": {
                      "type": "long"
                    },
                    "delete_time_in_millis": {
                      "type": "long"
                    },
                    "delete_total": {
                      "type": "long"
                    },
                    "index_current": {
                      "type": "long"
                    },
                    "index_failed": {
                      "type": "long"
                    },
                    "index_time_in_millis": {
                      "type": "long"
                    },
                    "index_total": {
                      "type": "long"
                    },
                    "is_throttled": {
                      "type": "boolean"
                    },
                    "noop_update_total": {
                      "type": "long"
                    },
                    "throttle_time_in_millis": {
                      "type": "long"
                    },
                    "write_load": {
                      "type": "float"
                    }
                  }
                },
                "search": {
                  "properties": {
                    "fetch_current": {
                      "type": "long"
                    },
                    "fetch_time_in_millis": {
                      "type": "long"
                    },
                    "fetch_total": {
                      "type": "long"
                    },
                    "open_contexts": {
                      "type": "long"
                    },
                    "query_current": {
                      "type": "long"
                    },
                    "query_time_in_millis": {
                      "type": "long"
                    },
                    "query_total": {
                      "type": "long"
                    },
                    "scroll_current": {
                      "type": "long"
                    },
                    "scroll_time_in_millis": {
                      "type": "long"
                    },
                    "scroll_total": {
                      "type": "long"
                    },
                    "suggest_current": {
                      "type": "long"
                    },
                    "suggest_time_in_millis": {
                      "type": "long"
                    },
                    "suggest_total": {
                      "type": "long"
                    }
                  }
                }
              }
            }
          }
        }
      }
    })
});
