# 586-mvc
Single-page web app for issue tracking. Create projects, workers, and tasks and assign things to things. The project consists of four crates in one workspace.

## frontend
This is a webassembly application made with the [Yew wasm framework](https://docs.rs/yew/0.4.0/yew/) for Rust. It uses a simple message-based architecture. Output is created with an html! macro that lets you write something similar to regular HTML with static checking and embedded Rust code.

The frontend is thin and leaves all the business logic to the backend.

### Types
#### Model
The client's local context. Implements the Yew traits (Component, Renderable).
#### View
Variant type of different client modes for viewing Model data.
#### Msg
Variant type for Yew's message system. Component::update fields these. Msgs are sent by page events or by other Msgs.

## backend
This is a standalone REST server using the [Rouille web library](https://docs.rs/rouille/3.0.0/rouille/). It uses SQLite as a backing data store via the [Diesel ORM](https://docs.diesel.rs/diesel/). Diesel allows you to define migrations to create a database schema, and generates Rust code to integrate it with your own types and to statically check as much as possible.

### Types
#### Model
This trait represents the data store. It has methods for adding, getting, deleting, etc of the business types. On the production /my module, it's implemented using a database. On the /test module, it's implemented with regular data structures (hash maps, etc).

#### Action
This trait is for controller actions which are triggered via the API and read or modify the model. Each one is defined as its own module under /actions, and is wired up using macro magic in /actions/mod.rs (listed in routable_actions!()). Each action has an associated (Something)Action type (created by the above macro) which holds a (Something)Params object. Params are what gets deserialized out of the request body.

Action::perms() returns the permission requirments to perform this action.

Action::execute() takes the Model, does something with it, and returns something with the Serializable trait. This is serialized and sent to the client in the response body. This needs a little more work done on synchronizing the types between frontend and backend.

#### Logger
This trait is for logging. For now, both the production and test modules implement this by printing to the console.

#### Actor
This trait is not yet developed.

#### Env
This is the application context. There's no requirement on it, but the server expecs to be handed something that implements the Model and Logger traits. For now this is the extent of any kind of dependency injection. The real one lives in /my and the test one lives in /test.

#### Perm
A variant type representing a single permission such as CreateProject or ViewTask. Perms are collected into PermReqs, which are nestable All/Any sets. PermReqs can be conveniently constructed with the perms! macro using infix & and | operators.

## common
Things shared by the frontend and backend, mostly types which get serialized as JSON between them. This contains all the business types like Project, Worker, WorkerID, TaskName, etc.

## procs
Compile-time functions. Rust requires you to separate these into their own crate.
