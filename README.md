# microfeat
A super simple and wildly incomplete do-it-yourself feature flag server


## How it (kinda) works

The server starts at `http://localhost:8080` and it keeps a list of feature flags in memory. A feature flag is identified by a string and it can be `ON` or `OFF`.

You can use the following endpoints:

  - `http://localhost:8080/on/{flag_name}`: turn `flag_name` to `ON`
  - `http://localhost:8080/off/{flag_name}`: turn `flag_name` to `OFF`
  - `http://localhost:8080/del/{flag_name}`: purges `flag_name` from the state

Clients that want to keep track of available feature flags and be notified of changes can connect using websockets to: `http://localhost:8080/websocket` (terrible name, we know!).

## Future steps

Be warned, this a learning/fun project, so have no expectation whatsoever here!

If we were to continue developing this, we'll need to:

 - refine the protocol and the implementation
 - add support for tags (tag flags and, as a client, be able to listen to changes only for a given set of tags)
 - make the api restful
 - auth
 - flags persistance (DBs all the things!!!!)
 - percentage based flag (e.g. for soft rollouts)
 - decent client libraries with local evaluation of flags

## Contributing

Feel free to open an issue if you want to give us some ideas or contribute to the project!
