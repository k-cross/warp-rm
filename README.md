# Warp Utilities

A series of high level utilities that perform functions you might generally use scripts for.
It is meant to make certain types of operations easy to perform.

## Warp Remove

A file utility that removes files with the same name but different extensions.
As an example:

```
> ls ./
f1.jpg f1.raw f2.cr2 f3.txt
> wrm -t raw cr2 -s jpg jpeg png
> ls ./
f1.jpg f1.raw f3.txt
```

The `f2.cr2` file ended up getting removed.
This will remove any `.raw` or `.cr2` file that does not have a corresponding `.jpg`, `.jpeg`, or `.png` file of the same name.

For my use case, it's useful for cleaning up undesirable photos after manually reviewing photos.
It can be for any filetype though, I have run into this problem when documenting results in experiments and keeping all the different files that document specimen or experiment under the same name is common.
