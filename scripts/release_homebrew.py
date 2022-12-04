#!/usr/bin/env python3
import os
import sys
import hashlib
import subprocess

GH_TOKEN = os.environ["HOMEBREW_GH_TOKEN"]
repo = f"https://{GH_TOKEN}@github.com/UncommonIndustries/homebrew-spake-cli.git"


ARM_ARTIFACT_PATH = "dist/mac-arm/spake-cli-mac-arm64.tar.gz"
AMD64_ARTIFACT_PATH = "dist/mac-x86_64/spake-cli-mac-x86_64.tar.gz"


def clone_repo():
    commands = [["git", "clone", repo],
                ["git", "-C", "homebrew-spake-cli", "config",
                 "user.name", "UncommonIndustries"],
                ["git", "-C", "homebrew-spake-cli", "config",
                 "user.email", "feedback@uncommon.industries"]]
    for command in commands:
        subprocess.run(command, check=True)


def get_checksums():

    checksums = {"arm": get_hash(ARM_ARTIFACT_PATH),
                 "amd64": get_hash(AMD64_ARTIFACT_PATH)}

    return checksums


def get_hash(artifact_path):
    if not os.path.exists(artifact_path):
        raise FileNotFoundError(f"Artifact not found at {artifact_path}")
    with open(artifact_path, "rb") as f:
        bytes = f.read()
        readable_hash = hashlib.sha256(bytes).hexdigest()
        return readable_hash


def create_formula(armSha, amd64Sha, armUrl=None, amd64Url=None, version=None):
    from brew_formula_template import formula_template
    return formula_template.format(sha_arm64=armSha, sha_x86_64=amd64Sha, url_arm64=armUrl, url_x86_64=amd64Url, version=version)


def build_URLS(git_tag):
    # build the urls for the two files
    return {"arm": f"https://github.com/UncommonIndustries/spake-cli/releases/download/{git_tag}/spake-cli_{git_tag}_macos-arm64.tar.gz",
            "amd64": f"https://github.com/UncommonIndustries/spake-cli/releases/download/{git_tag}/spake-cli_{git_tag}_macos-x86_64.tar.gz"}


def commit_formula():
    commands = [["git", "-C", "homebrew-spake-cli", "add", "spake.rb"],
                ["git", "-C", "homebrew-spake-cli", "commit",
                    "-m", "Update formula for new release."]]
    for command in commands:
        subprocess.run(command, check=True)


def push_repo():
    command = ["git", "-C", "homebrew-spake-cli", "push", "--dry-run", repo]
    subprocess.run(command, check=True)


def release_homebrew(git_tag):
    # git clone repo
    clone_repo()
    # sha the two files,
    checksums = get_checksums()
    urls = build_URLS(git_tag)

    # template them into the formula
    formula = create_formula(
        checksums["arm"], checksums["amd64"], armUrl=urls['arm'], amd64Url=urls['amd64'], version=git_tag)

    # write template to formula file in repo with interpolated sha and link paths.
    with open("homebrew-spake-cli/spake.rb", "w") as f:
        f.write(formula)
    # git add, commit, and push
    commit_formula()


if __name__ == "__main__":
    git_tag = sys.argv[1]
    release_homebrew(git_tag)
