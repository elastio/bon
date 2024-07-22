import toml from "smol-toml";
import fs from "node:fs/promises";
import path from "node:path";

export default {
    watch: ['../../Cargo.toml'],
    async load([cargoTomlPath]: [string]) {
        const cargoTomlContent = await fs.readFile(cargoTomlPath, "utf-8");
        const cargoToml = toml.parse(cargoTomlContent) as any;
        return cargoToml.workspace.package.version;
    }
}
