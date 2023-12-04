import { Puzzle } from "@/lib/puzzle.ts";

export function getProjectName(puzzle: Puzzle) {
    return puzzle.title.toLowerCase()
        .replaceAll(" ", "-")
        .replaceAll(/[^a-z0-9-]/g, "");
}

export function getFolderName(puzzle: Puzzle) {
    const day = puzzle.day.toString().padStart(2, "0");
    return `${day}-${getProjectName(puzzle)}`;
}
