---
# This is a YAML block. It should not appear in the final result
title: A test
---

---
# This is another YAML block. 
foo: "this should not be parsed"
...


foo
---

Sub-chapter
-----------

# Chapter #

Section 
-------

# Another chapter #

## Another section ##

### Subsection ###

#### Subsubsection ####

##### Sub-3-section #####

###### Sub-4 section ######

####### Normally does not exist #######

A normal paragraph.

Another one, with *emphasis* and _more emphasis_.

One with **strong** and __more strong__.

Combination of *empasis and __bold__*

a dash-yes

two dashes -- cool

three dashes --- neat


## Unordered list ##

* a
* b
    * b.foo
* c

## Ordered list ##

1. one
2. two
3. tree


## Links ##

[inline link](http://foo.bar)

[inline- link with title](https://foo.bar "foobar")

[reference link][3]

[3]: http://foo.bar


## Code ##

Ìnline `<br>*foo*\text`[^code]

[^code]: inline code should escape markdown, latex or HTML characters
inside of it.



```
code block
```

### `code in title` ###

## Table ##

| A | B | C |
|---|---|---|
| 1 | 2 | 3 |

## Blockquote ##

> A blockquote wih *emphasis*, `some code`, and
> on multiple lines

## horizontal rule ##

---

***

___


## Footnotes ##

A normal footnote[^foo] 

[^foo]: hello


[^bar]: world!


This footnote is defined before used, is that a problem?[^bar]

Again a reference to first footnote[^foo]

## Auto clean ? ##

Should autoclean here ! `But maybe not here ?` *Here, definitely !*

```
definetly *not* here !
```


## Escaping ##

### html < \latex ? ###

Just testing, not trolling, I could have said `html > \latex{}` ore
even

```
html & \latex{} ~can be ^friends\\
```




