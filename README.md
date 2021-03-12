# guess-me

## How to call

```
near view guess.k2.testnet get_guess --accountId k2.testnet
```
This command will return a tuple with four parameters.  
The first is whether to start the game,  
the second is the current minimum,  
the third is the current maximum,  
and the fourth is the reward amount.

```
near call guess.k2.testnet random --accountId k2.testnet --amount 1
```
This command can generate a new guess,  
which can be called only when there is no guess currently.  
You can use the first `get_guess` command to check whether there is a guess currently.

```
near view guess.k2.testnet get_guess --accountId k2.testnet --amount 1
```
You can use this command to guess the guess.
which can be called only when there is guess currently.  
You can use the first `get_guess` command to check whether there is a guess currently.
When you guess right, you can get reward.