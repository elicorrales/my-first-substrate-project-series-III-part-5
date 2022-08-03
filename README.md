# my-first-substrate-project-series-III-part-5

# Upload, Instantiate, Call A Smart Contract To A Local Substrate Node  
  
## Let's Try The Node From Part 1 

### From [substrate-developer-hub](https://substrate.io/developers/), the [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/README.md).  
  
We assume you have completed [Part 1](https://github.com/elicorrales/my-first-substrate-project-series-III-part-1/blob/main/README.md) in this series, and interacted with the local node via the Javascript client.js.  
  
Start up the local template node in a terminal and leave it running.  
  
In another terminal, go to your ```helloworld``` smart-contract project that we built in [Part 4](https://github.com/elicorrales/my-first-substrate-project-series-III-part-4/blob/main/README.md).  
  
Do a ```cargo contract upload --dry-run --suri //Alice```.  
(```//Alice``` is the account that is going to be deploying the contract).  
  
And immediately we have an issue.  
```
$ cargo contract upload --suri //Alice
ERROR: Metadata: Pallet not found

Caused by:
    Pallet not found
```
  
So kill that ```node-template``` that's been running, and in that same terminal, let's move on to something new.  
  

## Let's Try A New Node - substrate-contracts-node 
  
### From [parity](https://www.parity.io/), the [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node/blob/main/README.md) is different.  

```
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force --locked
```
  
Unlike the first ```node-template```, this ```contracts-node``` is installed and in your path, so you don't have refer to it via ```./target/release/...blah..blah```.  Just run it.  
  
```
substrate-contracts-node --detailed-log-output --dev
```
  
And leave it running in that terminal.  
  
In the other terminal, the one of the ```helloworld``` smart-contract, let's repeat our attempt.  
  

Do a ```cargo contract upload --dry-run --suri //Alice```.  
(```//Alice``` is the account that is going to be deploying the contract).  
  
Oh the sweet output of success(?).  Maybe. We're not sure yet.  But it's an improvment.  
```
$ cargo contract upload --suri //Alice
        Event Balances ➜ Withdraw
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          amount: 483083527
        Event Balances ➜ Reserved
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          amount: 304815000000
        Event Contracts ➜ CodeStored
          code_hash: 0xc9eb5b5b992132eda71405929fefb03026e30f652fb9b42969478705f552aae7
        Event TransactionPayment ➜ TransactionFeePaid
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          actual_fee: 483083527
          tip: 0
        Event System ➜ ExtrinsicSuccess
          dispatch_info: DispatchInfo { weight: 396785000, class: Normal, pays_fee: Yes }

    Code hash 0xc9eb5b5b992132eda71405929fefb03026e30f652fb9b42969478705f552aae7
```
 
So what we have done is to upload the contract to our local node.  But that is not enough to interact with it.  
We have to also create an instance of it. To ```instantiate``` it.  
   
That last line in the above output is important. Copy the hash.  
In that terminal, set it equal to a local Bash shell variable.  
```  
code_hash=0xc9eb5b5b992132eda71405929fefb03026e30f652fb9b42969478705f552aae7;
```
  
Now, copy-paste and run this command to instantiate the contract:  
```
cargo contract instantiate \
       --constructor new \
       --suri //Alice \
       --code-hash $code_hash
```
  
We have a different problem:  
```
$ cargo contract instantiate \
    --co>        --constructor new \
>        --suri //Alice \
      -->        --code-hash $code_hash
ERROR: Module error: Contracts: OutOfGas

The executed contract exhausted its gas limit.
```
  
But at least it seems the node can handle contracts.  
  
Do a ```cargo contract instantiate --help``` and find/read the ```--gas``` option.  
  
So now we will modify the above command.  
```
cargo contract instantiate \
       --gas 50000000000 \
       --constructor new \
       --suri //Alice \
       --code-hash $code_hash
```
  
Notice that we just took the same gas value as was the default, so of course we should get the same error.  
  
Now, take the above command, the gas value, and just add one more zero at the end of it.  
  

Now, copy-paste and run this command to instantiate the contract:  
```
cargo contract instantiate \
       --gas 500000000000 \
       --constructor new \
       --suri //Alice \
       --code-hash $code_hash
```
  
Voila!  
```
$ cargo contract instantiate        --gas 500000000000        --constructor new              --suri
//Alice        --code-hash $code_hash
        Event Balances ➜ Withdraw
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          amount: 86298152
        Event System ➜ NewAccount
          account: 5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA
        Event Balances ➜ Endowed
          account: 5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA
          free_balance: 100405000000
        Event Balances ➜ Transfer
          from: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          to: 5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA
          amount: 100405000000
        Event Balances ➜ Reserved
          who: 5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA
          amount: 100405000000
        Event Contracts ➜ Instantiated
          deployer: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          contract: 5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA
        Event TransactionPayment ➜ TransactionFeePaid
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          actual_fee: 86298152
          tip: 0
        Event System ➜ ExtrinsicSuccess
          dispatch_info: DispatchInfo { weight: 979504674, class: Normal, pays_fee: Yes }

     Contract 5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA
```
  
Ok, so we have 1) uploaded, 2) instantiated.  Now, we are ready to attempt to 3) call it.  
That is, let's try to invoke the one method that we have in that contract (see [Part 4's lib.rs](https://github.com/elicorrales/my-first-substrate-project-series-III-part-4/blob/main/my-first-smart-contract/helloworld/lib.rs).  The ```message``` is ```sayhello```.  
  

The ```call``` command is similar, but different from the ```instantiate``` command, in that instead of using the ```Code hash``` output of the ```upload```, we need to use the ```Contract``` (last line of above output).  Also, instead of a ```-constructor```, it is a ```--message```, followed by the name of it.  
  
In the Bash terminal of your contract, do:  
```
Contract=5FGohWJbeYZpaFdbo7rUfeHRCYYgZEnW2JUP4N4HzqJUbpvA;
```
  
```
cargo contract call \
       --message sayhello \
       --suri //Alice \
       --contract $Contract
```
  
The expected output, of course:  
```
$ cargo contract call        --message sayhello        --suri //Alice        --contract $Contract
ERROR: Module error: Contracts: OutOfGas

The executed contract exhausted its gas limit.
```
  
So this also costs.  Try the amended command:  
```
cargo contract call \
       --gas 500000000000 \
       --message sayhello \
       --suri //Alice \
       --contract $Contract
```
  
Eureka!  
```
$ cargo contract call \
>        --gas 500000000000 \
>        --message sayhello \
     --s>        --suri //Alice \
>        --contract $Contract
        Event Balances ➜ Withdraw
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          amount: 86298152
        Event TransactionPayment ➜ TransactionFeePaid
          who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
          actual_fee: 86298152
          tip: 0
        Event System ➜ ExtrinsicSuccess
          dispatch_info: DispatchInfo { weight: 592600674, class: Normal, pays_fee: Yes }
```
  
## What Is The Difference Between [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/README.md) and [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node/blob/main/README.md) ??  

I grabbed a copy of some of the output during building of both nodes.  
  
Here is partial output of the ```substrate-node-template```:  
```
Compiling pallet-transaction-payment
Compiling pallet-authorship
Compiling pallet-sudo
Compiling pallet-randomness-collective-flip
Compiling frame-executive
Compiling sc-rpc-api
Compiling sc-finality-grandpa
Compiling pallet-timestamp
Compiling pallet-balances
Compiling pallet-template
Compiling pallet-transaction-payment-rpc-runtime-api
Compiling pallet-session
Compiling pallet-aura
Compiling pallet-transaction-payment-rpc
Compiling sc-rpc
Compiling substrate-frame-rpc-system
Compiling pallet-grandpa
```
  
And here is a similar output from building the ```substrate-contracts-node```:  
```
Compiling pallet-transaction-payment
Compiling pallet-sudo
Compiling frame-executive
Compiling pallet-authorship
Compiling pallet-randomness-collective-flip
Compiling sc-chain-spec
Compiling sc-informant
Compiling sc-offchain
Compiling pallet-balances
Compiling pallet-contracts  <--most important difference
Compiling pallet-timestamp
```
  
So the first node is missing the one pallet that we need: ```pallet-contracts```.
  
I was going to delve into the topic of ```pallets```, but from doing google searches, it mostly seems to be used by this substrate blockchain framework.  I apologize if I am wrong.  
  
Since these blockchain series seem to be an ongoing overview of various blockchains, I didn't aspire to delve too deeply into any one of them, at least not on the first pass.
  
The purpose of all these series is to become a *generalist*, not a specialist in any one particular blockchain.  
  
Having said that, it is the ```pallets``` that partly make substrate be a framework and not just a blockchain.  
  
You can put together and configure various pallets and those seem to give your particular blockchain its purpose, capablities, and flavor.  
  
For example, I believe the polkadot is a blockchain.  
  
If you go to [Defi Llama](https://defillama.com/chains), it will rank coins and tokens by the total value locked in each *chain*.  
  
So in our up-to-now series, we have covered *NEAR*, *Solana*, and *Substrate*.  
  
If you do a search on Defi Llama, for NEAR, you'll find it.  
Also, Solana.  But you won't find *substrate*.  However, you *will* find Polkadot.  
Polkadot is build from Substrate.  So are others.  
  

## What About The Node Running In the Docker Container of Part 2?  
  
That one gave us a different error, and I never pursued it any further.  If you are interested, try to find a different, maybe newer version of the right Docker image to create a node from.  
  
