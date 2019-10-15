# cdd - A simple directory bookmark program

This program allows you to create directory bookmarks that you can use to quickly navigate around your file system instead of having to type/tab-complete through long paths constantly.

## Installation

First, build the .exe file with Cargo. Second, place 'cbd.bat' in your PATH and rename it if you would prefer to call the command something else.
'cbd.bat' makes calls to the .exe file, so if you don't plan to have them in the same directory, make sure to edit the .bat with the proper path.

## Usage

**Creating or changing a bookmark**
```shell
> cbd.bat create bookmarkName path/To/Directory
```
Where bookmarkName is whatever name you'd like to give it, e.g. *win*, and path/To/Directory is the path, e.g. *C:\Windows*

**Removing or deleting a bookmark**
```shell
> cbd.bat remove bookmarkName
```

**Display usage information**
```shell
> cbd.bat help
```

**Change directory**
```shell
> cbd.bat bookmarkName
```
Note that you can also pass normal paths as well (e.g. C:\Users) if you have a directory you want to change to that's not bookmarked.

### Disclaimer

Currently this only works on Windows because the key functionality relies on a .bat file. In the future I plan to write a Bash script so that this can be used in Linux as well.
