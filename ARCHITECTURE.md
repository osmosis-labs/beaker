# Architechture

Protostar SDK is designed to be modular, composable and extensible so that:

- any chain can customize protostar to suit their needs
- extentions can be built in a permissionless manner which decentralize the contribution and maintainance

Thus, Protostar SDK is being separated into 3 main parts:

1. **Core** – It's the glue that ties everything together. Very lean and does not have any idea about CosmWasm or anything apart from just making sure everything could work together.
2. **Modules** – The actual logic for their repective job, eg. CosmWasm build and deployment, interactive console, workspace management. It's follows the open/closed principle (open for extension, closed for modification), so for example, instead of cloning the whole CosmWasm related module to modify the logic from supporting `wasmd` to `osmosisd`, you can just change tell the module what chain it needs to support and add more chain specific logic if needed without modifying the module.
3. **Interface** – core and modules together provides functions as a dev tool but makes no assumption about how it should be interacted with. While it is started with a CLI, we can build any other kind of interface around it, eg. Web UI, VSCode extension.
