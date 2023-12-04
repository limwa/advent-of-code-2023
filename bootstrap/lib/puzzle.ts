import { dom, turndown } from "@/deps.ts";

export type Puzzle = {
    day: number;
    title: string;
    url: string;
    inputUrl: string;

    parts: PuzzlePart[];
}

export type PuzzlePart = {
    id: string;
    description: string;
};

function getPuzzleUrl(year: number, day: number) {
    return `https://adventofcode.com/${year}/day/${day}`;
}

type GetPuzzleOptions = {
    fetch?: typeof fetch;
};


const turndownService = new turndown();

function parsePuzzlePart(element: dom.Element) {
    const titleElement = element.querySelector("h2");
    if (!titleElement) throw new Error("Failed to parse puzzle part. Reason: Invalid title");

    const id = titleElement.id || "part1";
    const description =  turndownService.turndown(element.innerHTML);

    return {
        id,
        description,
    };
}

function getPuzzleTitle(document: dom.Document) {
    const titleElement = document.querySelector("article.day-desc > h2:not([id])");
    if (!titleElement) throw new Error("Failed to parse puzzle title. Reason: Invalid title");

    const matches = titleElement.textContent.match(/(?<=^--- Day \d+: ).*(?= ---$)/gm);
    if (!matches) throw new Error("Failed to parse puzzle title. Reason: Title is not in expected format");

    return matches[0];
}

function getPuzzleParts(document: dom.Document) {
    const partsElements = document.querySelectorAll("article.day-desc");

    const parts: PuzzlePart[] = [];

    partsElements.forEach((partElement) => {
        const part = parsePuzzlePart(partElement as dom.Element);
        parts.push(part)
    });

    return parts;

}

export async function getPuzzle(year: number, day: number, options?: GetPuzzleOptions): Promise<Puzzle> {

    const fetch = options?.fetch ?? globalThis.fetch;

    const url = getPuzzleUrl(year, day);

    const response = await fetch(url).then(r => r.text());
    const document = new dom.DOMParser().parseFromString(response, "text/html");
    if (!document) throw new Error(`Failed to parse HTML. Received: ${response}`);
    
    const puzzleParts = getPuzzleParts(document);
    if (puzzleParts.length === 0) throw new Error("Failed to parse puzzle parts. Reason: No parts found");

    const title = getPuzzleTitle(document);

    return {
        day,
        title,
        url,
        inputUrl: `${url}/input`,
        parts: puzzleParts,
    }
}