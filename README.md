# Free TON Vanity address generator
## Generating addresses for Free TON with random mnemonic or just random secret, checking, saving(file, redis, postgresql)
### Build
```cargo build --release```
### Run Args
```
    -c, --contract <contract_path>    Path to tvc file with contract for which addresses will be generated
    -f, --file <file_backend>         Path to file where to save results, redis or postgresql connection string
    -m, --mnemonic <mnemonic>         Set true if you need to addresses with random mnemonic seed(much slower if set
                                      true)
    -t, --threads <threads_amount>    Amount of threads

```
### Simple run
```./address_gen -c "/Users/pavel/CLionProjects/FreeTonVanity/SetcodeMultisigWallet.tvc" -m true -t 8```
Start address generator with 8 threads, output saves to csv
## !! PostgreSQL and Redis backend not implemented yet  
TODO:
 - Implement Redis and PostgreSQL backend for storing results
 - Save contract version, hash
 - Optimize write to csv
 - Optimize Mnemonic seed generator
 - Add ability to provide file with prefixes to search
 - Add and optimize regex
 
 
 ### Author @get_username