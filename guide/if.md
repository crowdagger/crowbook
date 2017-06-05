Interactive fiction
=======================

Version `0.12.0` added experimental support for writing interactive fiction. 

## Basics ##

If you want to have a non-linear story, you can simply use Markdown
links just as you would for any other link:

```markdown
* [Open the treasure chest](open_chest.md)
* [It might be trapped, stay away from it](stay_away.md)
```

All Crowbook renderers should render this correctly, allowing the
reader to "choose her adventure". Note, however, that you still need
to include all these Markdown files in you book configuration files. 

## The interactive fiction renderer ##

While the above allows you to generate correct EPUB and PDF files, it
will still display all the content if the reader chooses to read your
book linearly. While this may not be a problem, you might want to only
display the part of the book that the reader is actually exploring. 

In order to do so, you can use the interactive fiction html renderer:

```yaml
output.html.if: my_book.html
```

This output is similar to the standalone HTML output, except the
option to display only a chapter at a time is always true, and there
is no way to display the table of contents. 

## Using Javascript in your interactive fiction

While the above allows the reader to choose his own path, its
interactivity is quite limited. With the interactive fiction renderer,
it is possible to include Javascript code in your Markdown files,
using a code block element: 

```markdown
You open the chest, and you find a shiny sword. Yay!

    user_has_sword = true;
```

This Javascript code can return a string value, which will be displayed
inside the document according to the reader's previous choices:

```markdown
You encounter a goblin, armed with a knife!

    if (user_has_sword) {
	    return "You kill him with your sword, congratulations!";
	} else {
	    return "You don't have any weapon, you die :(";
	}
```

> Note that *only* the interactive fiction renderer supports this way
> of embedding Javascript code. If you try to render a document
> containing such code blocks to EPUB, PDF, or the "normal" HTML
> renderer, they will be displayed as regular code blocks. 


## Embedding Makdown in your Javascript code embedded in your Markdown

If you want to include Markdown formatting in the Javascript code (to
display a passage or another without having to write HTML code), you
can use the `@"..."@` syntax:

```markdown
    @"You face a troll!"@
    if (user_has_sword) {
	    @"* [Attack him with your sword](fight_troll.md)"@
	} else {
        @"* [Better run away](run_away.md)"@
	}
```

Note that in this case you don't need to return a value, this is done
behind your back. Similarly, `@"..."@` blocks don't require
semicolons.

If you need to access the value of a Javascript variable inside this
Markdown code, you can use the `{{...}}` syntax:

```markdown
    var name = prompt("Enter your name", "world");
	@"Hello, {{name}}"@
```

## Interactive fiction options 

As other renderers, there are options specific to the interactive
fiction.

**html.if.new_game** allows you to specify the path to a Javascript  that will
be run at the beginning of the game. Since this code is not embedded
in a function and is at the root (and the beginning) of the document,
it is a good place to declare all the functions and the global
variables you might need for your interactive fiction mechanics.
e.g.:

```yaml
html.if.new_game: some_file.js
```

**html.if.new_turn** and **html.if.end_turn** allow you to specify some Javascript code that
will be executed at the beginning and the end of each segment. Unlike
`html.if.new_game`, the (usually shorter) code is specified inline,
and can return a string value that will be displayed at the beginning
and the end of each segment. This is exactly like including code
blocks at the beginning or the end of each of your Markdown file. E.g.:

```yaml
html.if.new_turn: "nb_turns += 1;"
html.end_turn: "return 'Turn: ' + nb_turns;"
```

**html.if.script** allows you to specify the name of a Javascript file
to override the default script.
