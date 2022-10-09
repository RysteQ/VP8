# VP8

VP8 is a virtual 8 bit computer that runs 6502 assembly created in rust.

**Table of contents**
- **[Features](#features)**
- **[Memory map](#memory-map)**
- **[How to run](#how-to-run)**
- **[Current TODO list](#todo)**
- **[Ways to contribute](#ways-to-contribute)**

## Features

This project has two very important features. 

The first main feature this project has is a fully working screen that is 128 tall by 128 wide with 16 colours although you can modify it to go up to 256 colours if you so desire.

The second feature is that the program is not saved in the 64 kilobytes of virtual RAM, the program memory is seperate from the RAM so your code can be as big as you want.

## Memory map

The memory map is simple, the first 256 bytes (\$0000 - \$00FF) is the zero page, the rest 16 kilobytes after that (\$01FF - \$40FF) is reserved for screen memory and the rest of memory is reserved for whatever you want to use it for.

![Memory map](/misc/memory%20map.png)

🟥 **(\$0000 - \$00FF)** Zero page <br>
🟨 **(\$01FF - \$40FF)** Screen memory <br>
🟩 **(\$4100 - \$FFFF)** Free memory <br>

* The stack is not mentioned yet since I will do some minor changes to it later

## How to run

After you download the source code compile it and then run the following command

```bash
./vp8 input_file.extension
```

* *I will expand this in the future as more features get added in as a way to change the max fps the emulator will run at, a hexdump functionality (the chances of that are really slim) and a debug mode so it will run a single instruction (or multiple) each time you press a button.*

## TODO

My current TODO list is the following

1) Get the GUI to work properly
2) Add full flag functionality and PHP / PLP instructions
3) Make the Readme.md look better with images / GIFs and stuff like that
4) Add an examples folder with some very simple programs
5) Test the program for any hidden bugs that I may very well have missed

It goes without saying that my other goals that I do not have in my TODO list because I find it quite obvious is to increase perfomance but most importantly increase code readability even if it hurts perfomance a little bit.

## Ways to contribute

Since I am not experienced in rust yet I would greatly appreciate your help in this project by helping me in the project. What I would find amazing is for people to make the code smaller or even faster and if someone is up to the task they may even add a whole new feature.

But one thing is for certain, all help is amazing.
