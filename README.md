# spake-cli-redux
rust implemented CLI for uncommon Spake. 


## Usage
    
If you have this repository cloned, you can run the CLI  locally with ```cargo install --path .```

### Installing from a distribution
Currently the package is not distributed, but you can build it yourself with ```cargo build --release```

### Running the CLI
The CLI is run with ```spake-cli```

run ```spake-cli --help``` for more information


## Contributing
The code is extremely simple at the moment. If you want to contribute, please open an issue or a PR.
A few good things in the near term are: 
1) ~~Adding a test suite~~
2) ~~Making the CLI use an asynchronous runtime and use a non blocking client~~
3) Clean up the code broadly. (Take code chunks from main() and move to their own functions, etc.)
4) Expand State and configuration
5) ~~Allow reliance of Env variables for API Key~~
