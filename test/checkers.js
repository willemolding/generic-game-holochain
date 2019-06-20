
let { createGame, makeMove, getState, renderState, lastResult, results } = require('./helpers')

module.exports = scenario => {

  scenario("Can create a new game of checkers and make a move", async (s, t, { alice, bob }) => {

    let game_address = await createGame(alice, bob);

    // agent 2 must go first
    await makeMove(bob, {
      game: game_address,
      timestamp: 0,
      move_type: {MovePiece: { from: {x: 1, y: 5}, to: {x: 0, y: 4} }},
    })
    t.notEqual(lastResult().Ok, undefined, "Bob made the first move")

    await renderState(alice, game_address)

    await makeMove(alice, {
    	game: game_address,
    	timestamp: 1,
    	move_type: {MovePiece: { from: {x: 0, y: 2}, to: {x: 1, y: 3} }},
    })
    t.notEqual(lastResult().Ok, undefined, "Alice made the second move")

    await renderState(alice, game_address)

    await makeMove(bob, {
      game: game_address,
      timestamp: 2,
      move_type: {MovePiece: { from: {x: 5, y: 5}, to: {x: 6, y: 4} }},
    })
    t.notEqual(lastResult().Ok, undefined, "Bob made the third move")

    let state = await getState(alice, game_address)

    t.equal(state.Ok.moves.length, 3, "There were three moves in the game")

    // both agents should see the same game state
    t.deepEqual(await getState(bob, game_address), await getState(alice, game_address), "Alice and Bob both see the same game state")


    // finally print all the outputs
    results().forEach((result, i) => {
      console.log(`${i}: ${JSON.stringify(result, null, 2)}\n`)
    })

  })

  // test the matchmaking
  scenario("Bob can accept Alices proposal, create a game and Alice can see the game", async (s, t, { alice, bob }) => {
    const addr = await alice.callSync("main", "create_proposal", {message : "sup"})
    t.equal(addr.Ok.length, 46, "Proposal was created successfully")

    const proposals = await bob.callSync("main", "get_proposals", {})
    console.log(proposals)
    t.equal(proposals.Ok.length, 1, "Bob could retrieve Alices Proposal")

    const acceptance = await bob.callSync("main", "accept_proposal", { proposal_addr: proposals.Ok[0].address, created_at: 0 })
    t.notEqual(acceptance.Ok, undefined, "Bob could accept the proposal by creating a game") // check it returned Ok

    const games = await bob.callSync("main", "check_responses", { proposal_addr: proposals.Ok[0].address })
    t.deepEqual(
      games.Ok, 
      [{ 
        entry: { 
          player_1: bob.agentId,
          player_2: alice.agentId,
          created_at: 0
        }, 
        address: games.Ok[0].address
      }],
      "The game was created as expected"
    )
  })

}