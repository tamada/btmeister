# btmeister

This is the library to detect the build tools/task runners in the projects.

## Build Tool Definitions

The the build tools are defined in the JSON file which contains the binaries.
The schema of the definition JSON file is as follows.

```json
{
    "type": "array",
    "minItems": 0,
    "items": {
        "type": "object",
        "required": ["name", "build-files", "url"],
        "additionalProperties": false,
        "properties": {
            "name": {
                "type": "string"
            },
            "build-files": {
                "type": "array",
                "minItems": 1,
                "items": {
                    "type": "string"
                }
            },
            "url": {
                "type": "string"
            }
        }
    }
}
```

The default definition list is shown in `--list-defs` option of BtMeister.
BtMeister also can accept `--append-defs` option to specify the additional definitions.
