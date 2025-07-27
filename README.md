# cargypto\_cli

A bulk cryptocurrency wallet generator written in Rust. Generate BTC, LTC, ETH, and XMR wallets.

---

## Installation

Make sure you have the Rust toolchain installed (Rust 1.86 or newer):

```bash
# With rustup:
rustup update stable

# Clone and build from source:
git clone https://github.com/trvtsn/cargypto_cli
cd cargypto_cli
cargo build --release
./target/release/cargypto_cli
```

---

## Quick Start

### Generate 100 Bitcoin wallets in CSV format:

```bash
cargypto_cli -c 100 btc <ADDRESS_FORMAT>
```

Sample output:

```csv
"1","bc1qrdc45un9hq9t7xfqgaggy2cvyd07ljwm7vhedg","KziCbAMMbyiu278UL7PUWHnxALy9vjzQDbPeFjirUcQFa3THRZ4z"
"2","bc1q7u09xy352klqthm3xkp7mu4nym5dh5ll3e37td","L2m5eehuy9BmxA1dAdBCqHX5Y2aFASg5YF2gdjeQbJJADkFaVvgS"
"3","bc1qmx3v32dgu2ulrajt9nfp76dej2rv9ruf6tju05","L4NYXSaEZuZuCrX1rGMy1Kz6e1HJx1oriueH7gG59fB6fmq9h2NM"
"4","bc1qk6zvr5u30pk3kz4a4q2sdqs785matzgn35etr6","L3a5mSJijDxpnQGnz1a5chvWT6aTRfsyNrTD3AafdS115fvqWFFQ"
"5","bc1q08ngx3y2pamps6jmau2u89ycp5ef0fp4zez2t3","L2xzem45LZurDYJsXS1TGMD43f5Vsat8RMhCeqsDdH8EBQUiW5Lh"
...
```

### Generate 100 Litecoin wallets in CSV format:

```bash
cargypto_cli -c 100 ltc
```

Sample output:

```
"1","LbLshS2DTH8iUiueADaxKBWmgiiHz25BiY","T8YqGCkoKtqfJdofe4AXCVwayPiXY9x2e3aykbfXoSm86c9EpA3m"
"2","LT7d5V5JUriEozmmoUx7qt6mtcLrVgCW7a","T4TLPhhGUHn2CK9CveDXWukEkWkyKXLvF6N9PMMEQLaxNkdAXoMe"
"3","LbaS5LKodYFDY7wYQBtMo8ornMcfe6VRB6","TACbyMjdKT5WrnA85NQ8WdStGM5PwXPQc9o9mMp14oZXjLcoNZMk"
"4","LUwcRvUGqeXSrFfe3bV5x65jRZ1piPpvuf","TB33hCX8Yf3FHP6qi2XnHjLuh1E9vhMKQ8hakqqKKq7J2y2ZQadw"
"5","LhvvLeD2ZeEp3W5PcstxRBsazpfETEcqBA","T5B8HnTWdfWSmNPq6PcPgJybXuVwfoqqWVC3c6yHbiHfiQZnDEg5"
...
```

### Generate 100 Ethereum wallets in CSV format:

```bash
cargypto_cli -c 100 eth
```

Sample output:

```
"1","0x53d35108206087F552AA3c61fD6716EB823E1F65","4b20f5ec95c9e7bacfa326b56525d078e2d789e282a074abdc2ff96120fc6ce9"
"2","0xCEDA1e5C018A771e6a07FC389596E537447DE99B","01ff64eb9af3de524e2e39fdeaab551d305d5c587119124b555ab95d55e81661"
"3","0x40BaD0FcFb6692fFF24a2789C4964d6b356285aA","aa6426d40aa89671e4d2be18dc0cee119544681f76ced407ac32cf4432833efa"
"4","0x66F7901598c61B80D064b9Ab00FC98384Eec9Bd9","a184dd604c5cf72f85cebfc78c29a5ff2ef6dc45efbe351b3577c82aec6ab083"
"5","0x805D03d13c4809B69e420FaEEE23262576641fED","8c16b65598155b8f1e8cb5fd5ddc440d62b897f1bbbb32b56d00a0c50a92d8f1"
...
```

### Generate 100 Monero wallets in CSV format:

```bash
cargypto_cli -c 100 xmr
```

Sample output:

```
"1","467vt66ScgfTweJBxFWsjH5RLV67v9uYjDBArVGmsQNxDdfCAyzy8P11M9u3Fo4w42VLkkEtn4Tx586Nc9Ag9N2x9eDhV47","occur vacation jogger down unnoticed dual pyramid arsenic rising jeans abort edgy aptitude nibs motherly boxes fall bested square inkling annoyed corrode tipsy unsafe vacation"
"2","48MgiWzMxNTBHe6Cprm75tYqptbv8konSc68ghE6wSZZDz6J9jdvXdaAUGDbsEeQ4ZbjJCmrTnFesb7neN61ytpa3fTQKRp","vulture going locker gigantic hold vein grunt gables brunt torch mayor summon dads stylishly maximum science terminal dogs glide bypass vegan tamper foxes guest mayor"
"3","44Kq74cPGth7bRaNKLsDjUdn7kbGHxv65eDitt2wf59Kf51x2fn48Z93Qj43nL9fWnje4jgmfdf6JKodk3TNuYBHKCVdBx5","degrees nagged wallets scenic acoustic iguana tagged betting february upload alkaline damp sake phone noodles eagle hefty oars anybody peculiar binocular womanly lexicon long womanly"
"4","45abrLZ52SZFMTMb4vjZwE5dBZJjSUptcfxHWjcmFxSFhxtVGZ1M2JXL5tahm46rj3NjMGb52FSh1Q6B4XnYZ5XG8RGohVJ","nitrogen tumbling ruby upcoming kennel abort hive goes fizzle jukebox foamy illness tusks orchid balding adept aloof bypass rustled almost zigzags weird eccentric entrance ruby"
"5","465a2ReyPASM3Hoa6qLhKPhUGdTBWRZdHUq4UxwM8UcJMqEpB7Ms9WyPJxvRZRibnjZ3Sxjaax9pUUNt3ynaPVvkAswDguR","huts tacit auburn cynical viewpoint mice liquid dialect betting apex amended uncle pumpkins nylon innocent tail eden framed cinema wobbly bobsled unhappy waffle abbey wobbly"
...
```

---

## Command-Line Options

```text
Usage: cargypto_cli [OPTIONS] <COMMAND>

Commands:
  btc   
  ltc   
  eth   
  xmr   
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --count <COUNT>                  [default: 1]
  -o, --output-format <OUTPUT_FORMAT>  [default: csv] [possible values: csv]
  -h, --help                           Print help
```

---

## Security Notes

* **Protect Your Keys**: Always store generated private keys in a secure, encrypted location.
* **Offline Usage**: For maximum safety, run `cargypto_cli` on an air-gapped machine.
* **Audit**: Review the code before use, your funds depend on it.

---

## Contributing

Contributions, issues, pull requests, and feature requests are welcome!

```bash
# Fork the repo
git clone https://github.com/trvtsn/cargypto_cli.git
cd cargypto_cli

# Create a feature branch
git checkout -b feature/new-network-support

# After making changes
git commit -am "Add support for Dogecoin"
git push origin feature/new-network-support

# Open a pull request
```

---

## License

Distributed under the MIT License.
