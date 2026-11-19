# Add an event handler

An event handler is a function in your form's `.rs` file that runs when
something happens to a component: a click, a key, a change of value.

## The quick way: double-click

Double-click a component on the form. CrawRAD switches to **Code** view, adds
a handler for the component's main event (for a Button, `OnClick`) and puts the
caret inside it. Type the code and save.

## Any event: the Events tab

1. Select the component.
2. In the **Object Inspector**, open the **Events** tab (the
   **Edit events…** command does the same).
3. Find the event, for example `OnKeyDown`, type a name for the handler in its
   value and press <kbd>Enter</kbd>.

CrawRAD adds an empty function with that name and the event's signature to your
file. The signature of each event is listed in
[Components and properties](../reference/components.md).

To point several components at the same handler, type the same name in each.
To unlink an event, clear its value.

## What CrawRAD writes

```rust
/// `button1.on_click`
fn button1_click(&mut self) {
    // your code
}
```

The `` /// `button1.on_click` `` line is how CrawRAD knows which event the
function answers. Keep it above the function.

## Empty handlers go away

A handler the IDE created and you left empty is removed, together with its
event link, the next time the form is generated. Once you write anything in it,
it is yours and CrawRAD never touches it. See
[Forms: the designer file and your file](../concepts/forms-and-files.md).
