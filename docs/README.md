# Choice of 

- Rust using actix? framework for blazing fast Rest API
- Postgre Database using Diesel ORM
- serialized JSON request
- C++ Agents 


git clone --recurse-submodules -j8 url

# How to install
```
git clone --recurse-submodules -j8 ...
```

## Installing C++ client
```
mkdir build
cd build
cmake .. && cmake --build .
```

## Running apps
```
cd build
./{AppName}
```

Available apps
- commandAgentApp : run a single instance of a commandAgent

## Installing Rust server
```
cargo run
```