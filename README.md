# Mark and Recall

Place the application in a folder that is in your ``PATH``.

Then add this to the profile of your shell.

```bash
# Mark and recall
alias "mark=mark_recall mark"

function recall() {
  cd $(mark_recall recall $1)
}
```
