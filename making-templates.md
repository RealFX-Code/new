# making tamplates !!

file-name format: `category/name`
OR: `lang/type`
so `rust/cargo`, `node/js`, `js/vite` all goes

Example template:
```json
{
    "name":"template-format",
    "category":"examples",
    "command":{
        "args": [
            {
                "placeholder":"Argument 1",
                "replace":"{foo}"
            },
            {
                "placeholder":"Argument 2",
                "replace":"{bar}"
            }
        ],
        "exec":[
            "echo {foo} {bar}"
        ]
    }
}
```
