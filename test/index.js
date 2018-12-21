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

  // query the users. As none have been added should be empty
  const result2 = app.call("users", "main", "graphql", {query: "query { users { name } }"})
  console.log(result2)
  t.equal(result2.Ok, JSON.stringify({ users:[] }))

  // add a new user using a mutation
  const result3 = app.call("users", "main", "graphql", {query: "mutation { addUser(name: \"wollum\", age: 99) { address } }"})
  console.log(result3)
  let userAddress = JSON.parse(result3.Ok).addUser.address;
  t.equal(userAddress.length, 46)

  // retrieve a single user by their hash
  const result4 = app.call("users", "main", "graphql", {query: `query { user(address: "${userAddress}") { name } }`})
  console.log(result4)
  t.equal(result4.Ok, JSON.stringify({ user:{ name :"wollum"} }))

  // retrieve all the users in a single query and return the name and age
  const result5 = app.call("users", "main", "graphql", {query: `query { users { name, age } }`})
  console.log(result5)
  t.equal(result5.Ok, JSON.stringify({ users:[ { name :"wollum", age: 99} ] }))

  // ends this test
  t.end()
})
