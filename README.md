# Mark and Recall
## Installation
Place the application in a folder that is in your ``PATH``.

For convenience, add this to the profile of your shell.

```bash
# Mark and recall
alias "mark=mr mark"

function recall() {
  cd $(mr recall $1)
}
```

## Usage
You can use ``mark``, (or ``mr mark``, if you don't use the alias) to bookmark a directory and ``recall`` (or ``mr recall``) to restore it.
You can also use a name like this ``mark cat-pictures`` and then return to it using ``recall cat-pictures``.

If no name is given, like in the first example, then the bookmark will just be called ``default``. All the bookmarks are stored in ``~/.config/marks.list``.

There is also ``mr list`` to list all currently set marks and ``mr clear [name]`` to clear a mark. Use ``mr clear --all`` to clear all marks.
