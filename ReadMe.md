# VP8

VP8 is a virtual 8 bit computer that runs 6502 assembly created in rust.

**Table of contents**
- **[Memory map](#memory-map)**
- **[Differences](#differences)**
- **[How to run](#how-to-run)**
- **[Ways to contribute](#ways-to-contribute)**

## Memory map

The memory map is simple, the first 256 bytes (\$0000 - \$00FF) is the zero page, the rest 16 kilobytes after that (\$01FF - \$40FF) is reserved for screen memory and the rest of memory is reserved for whatever you want to use it for.

![Memory map](/misc/memory%20map.png)

🟥 **(\$0000 - \$00FF)** Zero page <br>
🟨 **(\$01FF - \$40FF)** Screen memory <br>
🟦 **(\$4100 - \$41FF)** Stack memory <br>
🟩 **(\$4200 - \$FFFF)** Free memory <br>

You may notice that I do not mention anything about where the program is saved on memory, that's because the program is not loaded in the virtual RAM and it's seperate from it. This way your program can be as big as you want it while making full use of the 64Kb or RAM.

## Diferences

There are a couple of creative liberties I took because I am new to rust. The first thing is that there is no input to this emulator. The second thing is that due to my inability to understand [piston](https://github.com/PistonDevelopers/piston) I added an extra command to update the screen, that command being ```DRW```. Also the flags are not exactly implemented correctly.

## How to run

After you download the source code compile it and then run the following command

```bash
./vp8 input_file.extension
```

## Ways to contribute

Since I am not experienced in rust yet I would greatly appreciate your help in this project by helping me in the project. What I would find amazing is for people to make the code smaller or even faster and if someone is up to the task they may even add a whole new feature.

But one thing is for certain, all help is amazing.
