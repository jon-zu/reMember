# reMember - shroom server

Server targeting a 2d sidescroller with shrooms.


# Core structure

* `shroom-meta` contains meta data for the game like Ids or Field data
* `shroom-proto95` contains the packet stuctures 
* `shroom-data` contains the actual persistence layer(database stuff to load/save stuff)
* `shroom-srv` is the actual core server logic for tick handling and other stuff
* `shroom-scripts` `scripts-lib` are the actual scripting libraries, the first contains the actual script api code and the lib crate handles the reloading and provides the glue code to the actual game
*  `shroom-login` is the login server
* `shroom-game` is the game server
* `mono` is the actual server, which combines all of the crates to produce a functional server

# Design notes

* Server uses tick based scheduling with groups, fields/maps are essentially the scheduler groups, each client is attached to a group
* Tick rate is 50ms for now
* Avoid any I/O in the game logic handlers
* Based on `tokio` which also adds good message passing features
* A World to manage all field/groups and client
* Scripting is done in rust, but scripts are built as shared library(dll/so) and reloaded when the library changes
* Service-oriented design for data
* Strongly typed for several concepts like Ids

# Scripts

* Scripts are placed into `crates/script-lib/scripts/src`
* `cargo install cargo-watch` to automatically rebuild them if they are updated
* To watch and rebuild: ` cargo watch -w crates/scripts-lib/scripts/src -x 'build -p scripts' `
* Right now hot-reloading is abit quirky never edit scripts when there's an active script(will be handled later)
* Scripts have access to the `shroom-meta` crate which implements plenty of the game logic already

# Skills

* Skill data is generated in the meta crate which strongly typed buff types, to ensure the compiler can check It
* Those are then applied in the `shroom-game` crate
* Passive skills are still missing and summons need to be redesigned



# Requirements

* A client with patched IP checks and disabled Shanda Encryption(You can lookup Hendi's Client and my DLL for that)
* Latest Rust Stable compiler(For windows you also need vc++ build tools)
* Git

# Building Instructions

1. Clone the repo
2. Init the submodules( `git submodule init` `git submodule update`)
3. Build and run the server `RUST_LOG=info cargo r -p mono`
4. Launch the client with the IP and Port passed as argument