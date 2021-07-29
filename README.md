# Visual assembly editor

***WORK IN PROGRESS***    
This program splits assembly code into control-flow-graph like structure using blocks of code (***CodeBlock***s) and linking them with lines which represent the control flow. The ***CodeBlock***s can be dragged around, edited, deleted and added new ones. The visible region can be panned by holding Ctrl and dragging with mouse.

## Controls

| Key press         |   Action                                                                             |
|-------------------|--------------------------------------------------------------------------------------|
| Ctrl + D          | switch to drag mode                                                                  |
| Ctrl + Delete     | delete selected ***CodeBlock***                                                      |
| Ctrl + Insert     | add new ***CodeBlock*** at cursor position                                           |
| Escape            | exit from drag mode                                                                  |
| Ctrl              | pan the view by dragging with left mouse button while holding Ctrl                   |

### Drag mode

By pressing Ctrl + D you enter drag mode. You can move ***CodeBlock***s by clicking on them with left mouse button and dragging. You remain in drag mode until you press Escape.

### Panning

When you hold Ctrl you can pan the view by clicking with left mouse button and dragging. This moves the visible area and allows navigating arround and seeing other parts of the code which were not visible before.

#### Panning and dragging

You can switch to drag mode and start dragging a ***CodeBlock*** and at the same time press Ctrl and start panning. This way you can easily move ***CodeBlock***s to positions farther away where you can't see without panning.

## Usage

### Opening normal files
You can open any assembly code source files (normal text files). This program will load it and split automaticaly into ***CodeBlock***s such that every ***CodeBlock*** has maximum one label which is at the beginning (first line), also every ***CodeBlock*** has maximum one unconditional jump which is at the last line and finaly every ***CodeBlock*** has maximum one conditional jump. The ***CodeBlock***s will be unorganized and will probably overlap and overshadow each other. It is up to you to position them such that they are visible and organized.

### Saving
When you save, this program will automaticaly save all text from all the ***CodeBlock***s into a normal text file. The code in the text file should be correctly ordered according to control flow. The position of each ***CodeBlock*** will be saved by including a comment above each ***CodeBlock*** text piece. This allows you to use the saved file as if it was just a normal file. You can for example run it through an assembler and it should not cause any problems.

#### The ***CodeBlock*** position comment looks like this:

> ;#codeblock,*x-position*,*y-position*,   
> *some code*

### Opening previously saved files

Previously saved files can be loaded and position of every ***CodeBlock*** should be restored.

### Editing

You can click into a ***CodeBlock*** to put a cursor there and start typing. As you type it will automaticaly check the rules listed in section *Opening normal files* and split the ***CodeBlock***s accordingly. For example when you are editing a ***CodeBlock*** like this:

    mylabel:
    xor eax, eax    ;set eax to 0
    mov ebx, 10     ;set ebx to 10
    
And then you type another label like this:

    mylabel:
    xor eax, eax    ;set eax to 0
    mov ebx, 10     ;set ebx to 10
    mysecondlabel:
    
It will get automaticaly split into two ***CodeBlock***s:

    mylabel:
    xor eax, eax    ;set eax to 0
    mov ebx, 10     ;set ebx to 10

and

    mysecondlabel:

A line will be also drawn from the bottom of the first ***CodeBlock*** to the top of the second ***CodeBlock*** to show the control flow. Control flow lines will be also automaticaly drawn and updated for every jump.


## Main idea

Writing (or reading) assembly code can be sometimes difficult because the text is very linear but yet it contains branching. It can be confusing to look at all those labels and jumps and conditional jumps and understand what is the code doing. You can even get lost in the control flow. For this reason it can be even called spaghetti code. Sometimes I helped myself by first drawing the control flow graph structure of the algorithm I was going to write in assembly on paper. With that I could see all the branches more clearly and reason about it. The drawing gave me the idea to make this program.
