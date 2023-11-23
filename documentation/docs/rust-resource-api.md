# Rust Resource API

During the development of your app, you will need to define new Rust resources, which essentially serve as API endpoints.

Each `.proto` file located in `./messages` and its subfolders is treated as a Rust resource. It's the basic concept of this framework. This file-based Rust resource declaration provides great readability and clarity to the API system that the app is using.

Each Rust resource will be assigned a unique `ID` on code generation, which is inserted into the `resource` field of `RustRequest` and `RustSignal` to distinguish which Rust resource that the message is trying to talk about.

This framework follows the RESTful API pattern, allowing for the definition of 9 possible message combinations within each `.proto` file like below. However, it's totally acceptable to create other message types as well.

```proto
message CreateRequest { ... }
message CreateResponse { ... }
message ReadRequest { ... }
message ReadResponse { ... }
message UpdateRequest { ... }
message UpdateResponse { ... }
message DeleteRequest { ... }
message DeleteResponse { ... }
message StateSignal { ... }
```

For more details about generating message code, refer to the [message code section](writing-code.md#message-code-generation).

> We highly recommend NOT version-controlling the generated message code. This framework writes folders containing the generated code to `.gitignore` when applying the template to prevent unnecessarily bloated Git history. Usually, only version-controlling `.proto` files is enough.
