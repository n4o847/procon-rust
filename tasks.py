import sys
import subprocess
import shutil
import os
import json
import onlinejudge

def set_url(problem_id, url):
    if not os.path.isfile("./data/problem_to_url.json"):
        with open("./data/problem_to_url.json", mode="w") as f:
            d = {}
            d[problem_id] = url
            json.dump(d, f)
    else:
        with open("./data/problem_to_url.json", mode="r+") as f:
            d = json.load(f)
            d[problem_id] = url
            f.seek(0)
            json.dump(d, f)

def get_url(problem_id):
    with open("./data/problem_to_url.json", mode="r") as f:
        d = json.load(f)
        return d[problem_id]

def download(url):
    problem = onlinejudge.dispatch.problem_from_url(url)
    problem_id = problem.problem_id
    subprocess.run(["oj", "d", url, "-d", f"./tests/{problem_id}"])
    set_url(problem_id, url)
    if not os.path.isfile(f"./examples/{problem_id}.rs"):
        shutil.copy("./src/main.rs", f"./examples/{problem_id}.rs")

def run(problem_id):
    subprocess.run(["cargo", "run", "--example", problem_id], check=True)

def test(problem_id):
    subprocess.run(["cargo", "build", "--example", problem_id], check=True)
    subprocess.run(["oj", "t", "-c", f"./target/debug/examples/{problem_id}", "-d", f"./tests/{problem_id}"])

def submit(problem_id):
    url = get_url(problem_id)
    subprocess.run(["oj", "s", url, f"./examples/{problem_id}.rs"])

def main():
    sys.tracebacklimit = 0
    _, command, *args = sys.argv
    if command == "download":
        download(*args)
    elif command == "run":
        run(*args)
    elif command == "test":
        test(*args)
    elif command == "submit":
        submit(*args)

if __name__ == "__main__":
    main()
