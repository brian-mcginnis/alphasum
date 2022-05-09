# AlphaSUM
The following repository is my take on on the **#399** coding challenge taken from *r/dailyprogrammer*[[link](https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/)].

It currently prompts the user to enter a lowercase alphabetic string. It takes each character as a key into a hash table of which each letter has a value(ex. a -> 1 ... z -> 26). Each value is then added to produce a sum.

## Ex. Usage Case
The following is an example usage case that is terminated by entering >```!q```

```bash
> cargo run
```
```rust
This program takes in a lowercase(a-z) string and adds each character as a key/value pair in a Hash map and returns the sum of all character values added.
The program does not accept whitespace nor characters that are not a-z.
(Enter'!q' to Exit)

Enter Text to Sum:
sfsf
sfsf: 50
Enter Text to Sum:
jkfld
jkfld: 43
Enter Text to Sum:
!q
```
## License
[MIT](https://choosealicense.com/licenses/mit/)
