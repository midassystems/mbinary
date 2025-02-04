# Midas Binary Encoding

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

## Disclaimer

The Midas Binary Encoding library is heavily inspired by and directly influenced by the [Databento DBN](https://github.com/databento/dbn) library. When starting this project, I was new to Rust and binary encoding, and much of the initial development was based on learning from and building upon Databento's DBN implementation. While it will continue to evolve into its own implementation, it is important to acknowledge the foundational inspiration provided by Databento's work.

## Overview

The Midas Binary Encoding library is a foundational component of the Midas ecosystem. It serves as the shared protocol for encoding and decoding data across all Midas system components. Although users typically do not interact with mbinary directly, it plays a crucial role in ensuring seamless data exchange between:

- **Midas Server**: Data storage and API backend.
- **MidasTrader**: Core backtesting and live trading engine.
- **Midas Shell**: Command-line and REPL interface.
- **Midas GUI**: Frontend visualization and analysis.

Functions similarly to protocol buffers, providing a structured and efficient binary format for data serialization and deserialization.

## Installation

You can install `mbinary` directly from [PyPI](https://pypi.org/project/mbinary/):

```bash
pip install mbinary
```

## Future Plans

- Extend schema support for additional data types.
- Optimize encoding and decoding for larger datasets.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request with suggestions or improvements.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.
