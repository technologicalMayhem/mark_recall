# Mark and Recall
## Installation
Place the application in a folder that is in your ``PATH``.

Then add this to the profile of your shell.

```bash
# Mark and recall
alias "mark=mark_recall mark"

function recall() {
  cd $(mark_recall recall $1)
}
```

## Usage
You can use ``mark`` to bookmark a directory and ``recall`` to restore it.
You can also use a name like this ``mark cat-pictures`` and then return to it using ``recall cat-pictures``.

If no name is given, like in the first example, then the bookmark will just be called ``_``. All the bookmarks are stored in ``~/.config/marks.list``.
