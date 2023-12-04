import requests
import bs4
import html2text
import subprocess
import re
import os

YEAR = 2023

class PuzzleInformation:
    
    TITLE_SANITIZATION_REGEX = re.compile(r'[^a-z0-9-]')
    
    def __init__(self, day: int, part_name: str, title: str, description: str, url: str) -> None:
        self.day = day
        self.part_name = part_name
        self.title = title
        self.description = description
        self.url = url
        
    def get_puzzle_folder_name(self) -> str:
        sanitized_title = self.title.replace(' ', '-')
        sanitized_title = sanitized_title.lower()
        sanitized_title = PuzzleInformation.TITLE_SANITIZATION_REGEX.sub('', sanitized_title)
        
        return f"{self.day:02}-{self.title}"

    def get_binary_name(self) -> str:
        return self.part_name
    
    def get_input_url(self) -> str:
        return f"{self.url}/input"
    

class PuzzlePartInformation:
    
    def __init__(self, puzzle: PuzzleInformation, part_id: str, description: str) -> None:
        self.puzzle = puzzle
        self.part_id = part_id
        self.description = description
        
    def get_binary_name(self) -> str:
        return self.part_id
    

PUZZLE_TITLE_REGEX = re.compile(r'--- Day \d+: (.+) ---')

def get_puzzles_for_day(day: int, session_cookie: str) -> list[PuzzleInformation]:
    result = []
    
    url = f"https://adventofcode.com/{YEAR}/day/{day}"
    puzzles = requests.get(url, cookies={'session': session_cookie}, timeout=10)

    puzzles_soup = bs4.BeautifulSoup(puzzles.text, 'html.parser')
    
    puzzle_description_elements = puzzles_soup.find_all('article', class_='day-desc')
    for puzzle_description_element in puzzle_description_elements:
        
        puzzle_header = puzzle_description_element.find('h2')
        
        puzzle_title_match = PUZZLE_TITLE_REGEX.match(puzzle_header.text)
        if puzzle_title_match is None:
            raise ValueError('Could not parse puzzle title')
        
        puzzle_title = puzzle_title_match.group(1)        
        part_name = puzzle_header.attrs['id'] or "part1"
        
        description_html = puzzle_description_element.decode_contents()
        description_markdown = html2text.html2text(description_html, baseurl=url)
        
        puzzle_information = PuzzleInformation(day, part_name, puzzle_title, description_markdown, url)
        result.append(puzzle_information)
        
    return result
        
    
def create_rust_project(name: str) -> None:
    subprocess.run(["cargo", "new", "--bin", "--vcs", "none", "--name", name], check=True)
    subprocess.run(["cargo", "generate-lockfile"], cwd=name, check=True)

def main():
    session_cookie = input('Enter session cookie: ')
    day = input('Enter day number: ')
    print()
    
    

    # Get puzzle description

    url = f"https://adventofcode.com/{YEAR}/day/{day}"
    
    puzzle_soup = bs4.BeautifulSoup(puzzle.text, 'html.parser')
    
    puzzle_description = puzzle_soup.find('article', class_='day-desc')
    if puzzle_description is None:
        raise ValueError('Could not find puzzle description')
    
    # Obtain puzzle information
    
    puzzle_header = puzzle_description.find('h2') # type: ignore
    

    if puzzle_title_match is None:
        raise ValueError('Could not parse puzzle title')
    
    puzzle_title = puzzle_title_match.group(1)
    puzzle_title = puzzle_title.replace(' ', '-')
    puzzle_title = puzzle_title.lower()
    puzzle_title = SANITIZATION_REGEX.sub('', puzzle_title)
    
    puzzle_title_day = day.zfill(2)
    
    puzzle_folder_name = f"{puzzle_title_day}-{puzzle_title}"
    if os.path.exists(puzzle_folder_name):
        raise ValueError(f"Folder {puzzle_folder_name} already exists")
    
    print(f"Creating {puzzle_folder_name}...")

    # Scaffold Rust project

    print("Scaffolding Rust project...")
    print()
    
    subprocess.run(["cargo", "new", "--locked", "--bin", "--vcs", "none", "--name", puzzle_title, puzzle_folder_name], check=True)
    subprocess.run(["cargo", "generate-lockfile"], cwd=puzzle_folder_name, check=True)
    
    print()
    
    # Write puzzle description to README.md
    
    print("Writing puzzle description to README.md...")
    html = puzzle_description.decode_contents() # type: ignore
    markdown = html2text.html2text(html, baseurl=url)
    
    with open(f"{puzzle_folder_name}/README.md", 'w', encoding="utf-8") as f:
        f.write(markdown)
    
    # Write puzzle input to input.txt
    
    print("Writing puzzle input to input.txt...")
    print()
    
    subprocess.run(["curl", "-b", f"session={session_cookie}", f"{url}/input", "-o", f"{puzzle_folder_name}/input.txt"], check=True)

    print()
    
if __name__ == '__main__':
    main()