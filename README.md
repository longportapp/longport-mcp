# Longport MCP

`Longport-MCP` is a MCP server for Longport OpenAPI.

## Usage
To run the project, use the following command:

### Start a [stdio server](https://spec.modelcontextprotocol.io/specification/2024-11-05/basic/transports/#stdio)

```bash
longport stdio
```

### Start a [SSE server](https://spec.modelcontextprotocol.io/specification/2024-11-05/basic/transports/#http-with-sse)

```bash
longport sse
```

Default bind address is `127.0.0.1:8000`, you can change it by using the `--bind` flag:

```bash
longport stdio --bind 127.0.0.1:3000
```
