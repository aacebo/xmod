# XFlux

a simple action/trigger based workflow framework.

## Examples

> a simple example of a routine definition that
> calls two actions.

```yaml
version: "0.0.1"  # semver
name: my-workflow # unique
display_name: "My Workflow"
actions:
    - id: get_user
      type: http::get
      input:
        uri: "https://my.api.com/v1/users/me"
        headers:
          Authorization: "Bearer {{ $token }}" # ctx.get("token")
      output: "$.data" # this of `$` as `this`

    - id: log_user
      type: log::info
      input: "user => {{ json($get_user) }}"
```
