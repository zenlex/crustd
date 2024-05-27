# crustd
A library for streamlining basic CRUD op setup in a JSON API. 

This is under active development and breaking changes should be expected. Contributors are welcome!

The basic end goal is to be able to define data models and relationships in a toml file (and potentially controller and service properties) and have the boilerplate templated from stubs to create db tables with migrations, routers, and controllers for the core CRUD operations for each of the defined structs. 

Currently the start is a set of three traits - CrudService, CrudController, and CrudRouter. You are required to implement the CrudService methods to define your DB queries and then the Router and Controller can be simple empty impl blocks. 

The traits are currently coupled to a sqlx + postgres impelementation. They were a byproduct of another project I extracted to start work on a library. 

Next steps: 
- [ ] Document the traits
- [ ] Genericize DB Driver to work with other sqlx drivers
- [ ] Autogenerate the test suites
- [ ] Add DB adapter for ORM (Diesel? Others?)
- [ ] TOML parser / stubs and CLI for rapid greenfield iteration? 
- [ ] Token validation middleware option?
