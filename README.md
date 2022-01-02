# File Sort

The program helps to move and sort files based on various options.

The options will all be in configuration files.

## Configuration

```JSON
{
  "configurations": [
    {
      "search": [
        "J.K. Rowling",
        "Harry Potter"
      ],
      "types": [
        "epub",
        "pdf"
      ],
      "directory": "/absolute/path/to/directory",
      "rename": "", // optional
      "append": true, // only applies when rename is applied
      "regex": {
        "search": "",
        "output": ""
      }
    }
  ]
}
```

## Flow

 * check if config available in current dir
   * If none, print error
   * If exists
     * Load config
     * Load files
     * Loop through config
       * check entry against files
       * continue if no match
       * if match
         * ask for confirmation if flag is set before moving and renaming
         * move and rename file
         * remove file from files list

## Roadmap

 * Have global config
 * Have GUI to edit configs
 * Generate config based on files in dir
 * Have several renaming options
 * Plugins for renaming
 * YAML config alternative
 * Multithreading? Disk read/write?

