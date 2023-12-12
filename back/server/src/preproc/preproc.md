
# Message preproccessing
There is some commands and preprocessing for it that make posts look better. Actually [posting form](/front/vue_x/src/components/PostingForm.vue) have buttons for most of them.

## Standard preprocessing
#### [Bold/Italic/Strike/Sub/Sup text](./general/mod.rs)  
Make a text bold/italic/striked/subscript/supscript.  
General form is `[X]some text[/X]` where instead of `some text` can stay any text and other commands, and instead of `X` can be stay corresponding sequence
| seq instead of `X` | what it do |
| --- | --- |
| `b` | bold |
| `i` | italic |
| `s` | strike |
| `sub` | subscript |
| `sup` | supscript |


#### [Spoiler text](./general/mod.rs)  
Make a text spoilered: it will be shown only when user hover over it.  
General form is `[spoiler]some text[/spoiler]` where instead of `some text` can stay any text and other commands.

#### [Random](./general/random.rs)  
Return a random value from specified range. It will be clearly shown to board users that the number was random and wasn't written by anon.  
General form is `[random(from:to)]`* where instead of `from` stay min random number and instead of `to` — max random value.

#### [Quote](./general/quote.rs)  
Make line looks like a quote.  
General form is `>` on the start of the line.

#### [Reply](./general/reply.rs)  
Make an reply.  
General form is `>>number` where instead of `number` stay board post number.

#### Example
This post text 
```
[b]bold[/b]
[i]italic[/i]
[s]strike[/s]
[s]strike and [b]bold[/b][/s]
[spoiler]spoiler[/spoiler]
text[sup]upper[/sup]
[sub]bottom[/sub]text
[random(0:100)]
>>12345678
> can you show me how quote looks like?!
like this one
```
Will be show as next  
![dices example](/docs/pics/std_example.png)

## Board related preprocessing

### `/rp/` (role play)
#### [Dices](./board_specific/rp/dice.rs)
Make a dice roll and retrun a random value that corresponding the dice (and shows to board users quite clearly that the this value was a roll of certain dice).  
General form is `[d0]`* where instead of `0` can stay one of standard DnD dice values (`{4, 6, 8, 10, 12, 20, 100}`).  

For **example** this post text (work only in `/rp/`)  
```
Let's just some dice rolls:
[d4][D6][dice:8][D10][D12][D20][D100]
[dice:4][D6][d8][d10][D12][D20][D100]
```
Will be show as next  
![dices example](/docs/pics/dices_example.png)


### `/a/` (anime)
#### [UwU rule](./board_specific/a/kawaii.rs)
~~While you are on `/a/` make an UwU sound at least once a minute >~< pleawse~~  
Make some \~\~words\~\~ cute-kawaii by changing\~ it's color!! :3  
~~It's unallowed to post in `/a/` at least without 20 absurdly cute and UwU words, kitty. Violation of this cute-kawaii rule will result to permanent ban >\~<~~

#### [:3](./board_specific/a/cat.rs)
Make some cat-related words (like `kitty`) looks like... kitty :3 

#### [Nya\~\~n](./board_specific/a/nyan.rs)
Make text between two `~~` more purple and cute and kawaii and cute-kawaii.  
Its not work if there is some other preproccess command inside it :\(

#### Example
This post(with great philosophical meaning) text (work only in `/a/`)   
```
/a/nime board is like many
> UwU  ~~nyan / myau~~ and kawaii
> ^w^ :3 <-- WoW! it's a kitty!
> sooo cute-kawaii
and no any sense except clear kawaii
>~<
```
Will look like  
![dices example](/docs/pics/a_board_example.png)

---

###### \* there are some others forms of the command but with the same sense.
