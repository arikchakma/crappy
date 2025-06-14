<h1 align="center">Crappy Calculator</h1>

<p align="center">
  <a href="https://github.com/arikchakma/crappy/blob/main/license">
    <img src="https://img.shields.io/badge/License-MIT-222222.svg" />
  </a>
  <a href="https://buymeacoffee.com/arikchakma">
    	<img src="https://img.shields.io/badge/-buy_me_a%C2%A0coffee-222222?logo=buy-me-a-coffee" alt="Buy me a coffee" />
  </a>
</p>

<p align="center">
  <b>A crappy calculator written in Rust</b></br>
  <sub>Just enter an expression, and it will be evaluated</sub><br>
</p>

## Start Using

You can start using the calculator by cloning the repository and running the following commands:

```bash
# Clone the repository
git clone git@github.com:arikchakma/crappy.git
cd crappy

cargo build --release
./target/release/crappy

# Start the REPL
--------------------
Crappy Calculator!
--------------------
Enter an expression to evaluate:
Type '/bye' to quit.
>> 2 + 2
4
>> 10 * 5
50
>> /bye
Bye!
```

## Features

- Basic Arithmetic Operations (`+`, `-`, `*`, `/`, `^`)
- Integer number support
- Simple expression evaluation
- Interactive command-line interface
- MIT licensed

## Roadmap

- [ ] Add floating-point number support
- [ ] Implement multi-line expression support
- [ ] Add environment variables support
- [ ] Improve error messages and user feedback

## Contributing

Feel free to submit pull requests, create issues, or spread the word.

## License

MIT &copy; [Arik Chakma](https://twitter.com/imarikchakma)
