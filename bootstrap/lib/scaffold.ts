import type { Puzzle } from "@/lib/puzzle.ts";
import { dax } from "@/deps.ts";
import { getFolderName, getProjectName } from "@/lib/names.ts";

const { $ } = dax;

function createDirectoryIfNotExists(path: string) {
    try {
        Deno.mkdirSync(path);
        return true;
    } catch (_err) {
        return false;
    }
}

function writeIfNotExists(path: string, content: string) {
    try {
        Deno.writeTextFileSync(path, content, { createNew: true });
        return true;
    } catch (_err) {
        return false;
    }
}

type ScaffoldOptions = {
    fetch?: typeof fetch;
};

export async function scaffoldRustProject(puzzle: Puzzle, options?: ScaffoldOptions) {
    const fetch = options?.fetch || globalThis.fetch;

    const folderName = getFolderName(puzzle);
    
    function writeDescriptionSync() {
        const description = puzzle.parts.map((part) => part.description).join("\n\n");
        Deno.writeTextFileSync(`${folderName}/README.md`, description);
    }

    const isNew = createDirectoryIfNotExists(folderName);
    writeDescriptionSync();
    
    if (isNew) {
        const projectName = getProjectName(puzzle);
        await $`cargo init --bin --vcs none --name ${projectName} ${folderName}`.spawn()
        await $`cargo generate-lockfile`.cwd(folderName).spawn();

        Deno.removeSync(`${folderName}/src/main.rs`);
    } else {
        console.log("Directory already exists. Modifying README.md and adding src/bin/ files...");
        console.log("Overwriting input.txt...");
    }

    Deno.mkdirSync(`${folderName}/examples/`, { recursive: true });
    
    Deno.mkdirSync(`${folderName}/src/bin/`, { recursive: true });
    for (const part of puzzle.parts) {
        writeIfNotExists(`${folderName}/src/bin/${part.id}.rs`, `fn main() {\n    println!("Hello from ${part.id}");\n}`);
    }

    const inputText = await fetch(puzzle.inputUrl).then(r => r.text());
    Deno.writeTextFileSync(`${folderName}/input.txt`, inputText);
}