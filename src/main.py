import os
import subprocess as sp
import time
from datetime import datetime
from pathlib import Path


def main() -> None:
    init()

    while True:
        # monday: 0, sunday: 6
        if str(datetime.now().weekday()) not in os.getenv("RECP_SKIP_DAYS"):
            repeat()
            print(f"pushed at {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}")
        else:
            print(f"skipped {datetime.now().strftime("%A").lower()} at {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}")

        print(f"sleeping for {os.getenv("RECP_SLEEP_SECS")} secs at {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}")
        time.sleep(int(os.getenv("RECP_SLEEP_SECS")))


def init() -> None:
    repo_path = f"/tmp/{os.getenv("RECP_REPO_NAME")}"

    Path(repo_path).mkdir(parents=True, exist_ok=True)
    os.chdir(repo_path)

    sp.run("git init", shell=True)

    sp.run(f"git config --local user.name '{os.getenv("RECP_USER_NAME")}'", shell=True)
    sp.run(f"git config --local user.email '{os.getenv("RECP_USER_EMAIL")}'", shell=True)

    sp.run(f"git remote add origin '{os.getenv("RECP_REPO_AUTH_URL")}'", shell=True)
    sp.run(f"git checkout -b '{os.getenv("RECP_BRANCH")}'", shell=True)


def repeat() -> None:
    sp.run(f"git pull origin '{os.getenv("RECP_BRANCH")}'", shell=True)
    sp.run("git commit --allow-empty -m 'echo'", shell=True)
    sp.run(f"git push origin '{os.getenv("RECP_BRANCH")}'", shell=True)


if __name__ == "__main__":
    main()
