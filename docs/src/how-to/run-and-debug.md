# Run and debug your app

Shortcuts are given for the default keymap, with the Classic keymap in
brackets. All of them are in the **Run** menu.

## Run

Press <kbd>F5</kbd> (<kbd>F9</kbd>). CrawRAD generates the form code, builds the
project and starts it under the debugger. The first build takes a while; later
ones only rebuild what changed. A web project opens in your browser instead of
a window (see [Publishing your app for the web](../tutorials/publishing-your-app-for-the-web.md)).

To run without the debugger, use **Run Without Debugging**,
<kbd>Ctrl</kbd>+<kbd>F5</kbd> (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>F9</kbd>).
Breakpoints are ignored.

If the build fails, the errors are listed with their file and line.

## Stop on a line

Put the caret on a line of your code and press <kbd>F9</kbd> (<kbd>F5</kbd>) to
add a breakpoint; press it again to remove it. Run the app. When it reaches the
line it stops, and the **Run and Debug** panel shows the call stack and the
local variables.

## Step through the code

| To | Shortcut |
|---|---|
| Run to the next line, over any call | <kbd>F10</kbd> (<kbd>F8</kbd>) |
| Go into the call on this line | <kbd>F11</kbd> (<kbd>F7</kbd>) |
| Run until the current function returns | <kbd>Shift</kbd>+<kbd>F11</kbd> (<kbd>Shift</kbd>+<kbd>F8</kbd>) |
| Carry on to the next breakpoint | <kbd>F5</kbd> (<kbd>F9</kbd>) |
| Stop where it is, now | <kbd>F6</kbd> |
| End the program | <kbd>Shift</kbd>+<kbd>F5</kbd> (<kbd>Ctrl</kbd>+<kbd>F2</kbd>) |

## Watch a value

Select an expression in the editor, or put the caret on a word, and use
**Add Watch** (<kbd>Ctrl</kbd>+<kbd>F5</kbd> in the Classic keymap). The value
is shown in the **Run and Debug** panel each time the program stops.

All the commands are listed in [Menus and commands](../reference/commands.md).
