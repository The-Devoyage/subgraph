db = db.getSiblingDB("subgraph_mongo");

db.createUser({
  user: "subgraph_mongo",
  pwd: "subgraph_mongo",
  roles: [
    {
      role: "readWrite",
      db: "subgraph_mongo",
    },
  ],
});

