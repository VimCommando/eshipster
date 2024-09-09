Elasticsearch High-Performance Strategy Enforcer
================================================

Elasticsearch is a highly distributed data store and search engine. An Elasticsearch cluster achieves peak performance when its workload is as evenly divided and distributed as possible. The primary unit of work in the cluster is the index shard. The built-in shard balancing algorithms frequently leave nodes unbalanced with significant resources underutilization across the cluster.

Elasticsearch High-Performance Strategy Enforcer (`eshipster`) is an external shard balancing tool that helps Elasticsearch clusters achieve optimal performance.

`eshipster` does four things:
1. Retrieves shard allocation information (one-off or reoccurring interval)
2. Calculates the best shard strategy for optimal performance (ingest or search)
3. Calls the Elasticsearch cluster APIs to update shard allocation and index settings
4. Records activity back into Elasticsearch for visualization

### Usage

Current functionality is limited to this command:

```bash
eshipster eval test/assets/indices_stats.json target/shards.ndjson
```

This will read the indices stats from the file `test/assets/indices_stats.json` and write the extracted shard documents to `target/shards.ndjson`.
