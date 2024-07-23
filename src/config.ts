
interface Config {
    version: string,
    sync: Sync,
    minecraft: Minecraft,
}

enum ActionAfterSync {
    Exit,
    DoNothing,
    ExecuteCommand,
    ExecuteCommandAndExit,
}

interface Sync {
    server: string,
    autoUpdate: boolean,
    autoSync: boolean,
    actionAfterSync: ActionAfterSync,
    command: string,
}

interface Minecraft {
    version: string,
    isolate: boolean,
    syncConfig: boolean,
}

export type { Config }
export { ActionAfterSync }