# xflux

Action/trigger workflow framework. Define async actions that run logic, wire them to triggers that fire on events, and compose them into declarative workflows with schema-validated inputs and templated URLs.

## Workflow Definition (YAML)

Workflows compose actions declaratively. URLs are `xtera` templates, inputs are validated against `xsch` schemas:

```yaml
name: get-user-by-id
version: "0.0.1"
display_name: "Get User By ID"
input:
  type: object
  fields:
    id:
      type: string
      required: true
actions:
    - id: get_user
      name: http::get
      version: 0.0.0
      input:
        uri: "https://my.api.com/v1/users/{{ $id }}"
        headers:
          Authorization: "Bearer {{ $token }}"
      output: "$.data"

    - name: log::info
      input: "user => {{ $get_user | json }}"
```

## Features

| Feature | Description |
|---------|-------------|
| `serde` | `Serialize`/`Deserialize` for action specs, context, and events |
