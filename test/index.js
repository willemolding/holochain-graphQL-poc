// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs-bleeding');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('description of example test', (t) => {

  const result = app.call("users", "main", "graphql", {query: "query { apiVersion }"})
  console.log(result)
  t.equal(result.Ok, "{\"apiVersion\":\"1.0\"}")


  const result2 = app.call("users", "main", "graphql", {query: "query { users { name } }"})
  console.log(result2)
  t.equal(result2.Ok, JSON.stringify({ users: { name: "wollum" } }))

  // ends this test
  t.end()
})
