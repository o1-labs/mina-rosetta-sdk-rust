#!/usr/bin/env python3
"""Capture the responses of a live Mina rosetta server into tests/fixtures/.

It reads the network, a block with a payment, balances and the mempool, and
runs the whole construction flow for a payment of 1 MINA, which it signs with
mina-ocaml-signer and SUBMITS. Use a local network, such as one minimina
starts with `role = "rosetta"`.

usage:
  MINA_PROFILE=devnet MINA_PRIVKEY_PASS=... python3 -I scripts/capture_fixtures.py \
      <rosetta url> tests/fixtures <mina-ocaml-signer> <sender key file> <receiver public key>
"""
import json
import os
import subprocess
import sys
import urllib.request
import urllib.error

url, out, signer, key_file, receiver = sys.argv[1:6]
os.makedirs(out, exist_ok=True)


def post(endpoint, body, name=None):
    request = urllib.request.Request(
        url + endpoint, data=json.dumps(body).encode(), headers={"content-type": "application/json"}
    )
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            answer = json.load(response)
    except urllib.error.HTTPError as error:
        answer = json.load(error)
        print(endpoint, "HTTP", error.code, json.dumps(answer)[:300])
    if name:
        with open(os.path.join(out, name + ".json"), "w") as f:
            json.dump(answer, f, indent=2)
            f.write("\n")
    return answer


def run(*args):
    return subprocess.run([signer, *args], check=True, capture_output=True, text=True).stdout


net = post("/network/list", {}, "network_list")["network_identifiers"][0]
n = {"network_identifier": net}
status = post("/network/status", n, "network_status")
post("/network/options", n, "network_options")
tip = status["current_block_identifier"]

# the newest block with a user command, and the genesis block
block = None
for index in range(tip["index"], 0, -1):
    candidate = post("/block", {**n, "block_identifier": {"index": index}})
    kinds = {o["type"] for t in candidate["block"]["transactions"] for o in t["operations"]}
    if "payment_source_dec" in kinds:
        block = candidate
        break
if block:
    with open(os.path.join(out, "block_with_payment.json"), "w") as f:
        json.dump(block, f, indent=2)
        f.write("\n")
post("/block", {**n, "block_identifier": {"index": 1}}, "block_genesis")

private = run("hex-of-private-key-file", "--private-key-path", key_file).split()[-1]
lines = run("derive-public-key", "--private-key", private).split()
public_hex = lines[lines.index("Key:", lines.index("Public")) + 1]
public_key = {"hex_bytes": public_hex, "curve_type": "pallas"}
derived = post("/construction/derive", {**n, "public_key": public_key}, "construction_derive")
sender = derived["account_identifier"]["address"]

post("/account/balance", {**n, "account_identifier": {"address": sender}}, "account_balance")
post(
    "/account/balance",
    {**n, "account_identifier": {"address": sender}, "block_identifier": {"index": 1}},
    "account_balance_historical",
)
post("/account/balance", {**n, "account_identifier": {"address": "B62qnot"}}, "error_bad_address")

token = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"
mina = {"symbol": "MINA", "decimals": 9}
acct = lambda a: {"address": a, "metadata": {"token_id": token}}
operations = [
    {"operation_identifier": {"index": 0}, "type": "fee_payment", "account": acct(sender),
     "amount": {"value": "-10000000", "currency": mina}},
    {"operation_identifier": {"index": 1}, "type": "payment_source_dec", "account": acct(sender),
     "amount": {"value": "-1000000000", "currency": mina}},
    {"operation_identifier": {"index": 2}, "related_operations": [{"index": 1}],
     "type": "payment_receiver_inc", "account": acct(receiver),
     "amount": {"value": "1000000000", "currency": mina}},
]
memo = {"memo": "rosetta sdk"}
pre = post("/construction/preprocess", {**n, "operations": operations, "metadata": memo},
           "construction_preprocess")
meta = post("/construction/metadata", {**n, "options": pre["options"], "public_keys": [public_key]},
            "construction_metadata")
payloads = post("/construction/payloads", {**n, "operations": operations, "metadata": meta["metadata"],
                                           "public_keys": [public_key]}, "construction_payloads")
post("/construction/parse", {**n, "signed": False, "transaction": payloads["unsigned_transaction"]},
     "construction_parse_unsigned")
signature_hex = run("sign", "--private-key", private,
                    "--unsigned-transaction", payloads["unsigned_transaction"]).split()[-1]
signature = {"signing_payload": payloads["payloads"][0], "public_key": public_key,
             "signature_type": "schnorr_poseidon", "hex_bytes": signature_hex}
combined = post("/construction/combine", {**n, "unsigned_transaction": payloads["unsigned_transaction"],
                                          "signatures": [signature]}, "construction_combine")
signed = combined["signed_transaction"]
post("/construction/parse", {**n, "signed": True, "transaction": signed}, "construction_parse_signed")
post("/construction/hash", {**n, "signed_transaction": signed}, "construction_hash")
submitted = post("/construction/submit", {**n, "signed_transaction": signed}, "construction_submit")
pool = post("/mempool", n, "mempool")
if pool["transaction_identifiers"]:
    post("/mempool/transaction", {**n, "transaction_identifier": pool["transaction_identifiers"][0]},
         "mempool_transaction")
post("/search/transactions", {**n, "address": sender, "limit": 2}, "search_transactions")
print("submitted", submitted)
