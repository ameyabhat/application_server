import click
from dotenv import load_dotenv
import requests
from pprint import pprint
from collections import defaultdict


@click.command()
@click.option('--prod', default=False, help="should we hit prod")
def run():
    pprint("hello")


local = "http://localhost:8080"
app = "https://generate-tech-app.xyz"


def get_token(path, nuid):
    return requests.get(f"{path}/forgot_token/{nuid}").json()["token"]


def get_challenge(path, token):
    return requests.get(f"{path}/challenge/{token}").json()["challenge_string"]


def find_kmers(k, challenge):
    soln = defaultdict(int)
    for i in range(len(challenge) - (k-1)):
        soln[challenge[i:i+k]] += 1

    return soln


def submit_soln(path, challenge):
    soln = find_kmers(3, challenge)
    return requests.post(f"{path}/submit/{token}", json=soln)


if __name__ == "__main__":
    load_dotenv()

    path = app
    token = get_token(path, "001453760")

    challenge = get_challenge(path, token)

    pprint(token)
    pprint(challenge)

    r = submit_soln(path, challenge)

    if r.status_code == 200:
        pprint(r.json())
    else:
        pprint(r.text)

