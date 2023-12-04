import { cliffy } from "./deps.ts"
import { createFetchClient } from "@/lib/fetch.ts";
import { getPuzzle } from "@/lib/puzzle.ts";
import { scaffoldRustProject } from "@/lib/scaffold.ts";

if (import.meta.main) {
  const sessionCookie = await cliffy.Input.prompt({
    message: "Enter your session cookie",
  });

  const day = await cliffy.Input.prompt({
    message: "Enter the day",
  });

  const year = await cliffy.Input.prompt({
    message: "Enter the year",
  });

  const fetch = createFetchClient(sessionCookie);
  const puzzle = await getPuzzle(parseInt(year), parseInt(day), { fetch });
  
  await scaffoldRustProject(puzzle, { fetch });
}