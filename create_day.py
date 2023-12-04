import requests
import bs4
import html2text
import subprocess
import re
import os

YEAR = 2023
PUZZLE_TITLE_REGEX = re.compile(r'--- Day \d+: (.+) ---')
SANITIZATION_REGEX = re.compile(r'[^a-z0-9-]')

def main():
    session_cookie = input('Enter session cookie: ')
    day = input('Enter day number: ')
    print()
    
    # Get puzzle description

    url = f"https://adventofcode.com/{YEAR}/day/{day}"
    puzzle = requests.get(url, timeout=10)
    puzzle_soup = bs4.BeautifulSoup(puzzle.text, 'html.parser')
    
    puzzle_description = puzzle_soup.find('article', class_='day-desc')
    if puzzle_description is None:
        raise ValueError('Could not find puzzle description')
    
    # Obtain puzzle information
    
    puzzle_header = puzzle_description.find('h2') # type: ignore
    
    puzzle_title_match = PUZZLE_TITLE_REGEX.match(puzzle_header.text) # type: ignore
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