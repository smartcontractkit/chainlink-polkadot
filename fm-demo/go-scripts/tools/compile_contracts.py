import json
import subprocess
import os
from os import path
import shutil

# A proof of concept / convenient script to quickly compile contracts and their go bindings

solc_versions = ["v0.4", "v0.6", "v0.7"]
rootdir = "./artifacts/contracts/ethereum/"
targetdir = "./contracts/ethereum"

used_contract_names = ["FluxAggregator", "VRF", "OffchainAggregator", "LinkToken"]

print("Locally installing hardhat...")
subprocess.run('npm install --save-dev hardhat', shell=True, check=True)

print("Modifying hardhat settings...")
with open("hardhat.config.js", "w") as hardhat_config:
    hardhat_config.write("""module.exports = {
solidity: {
    compilers: [
    {
        version: "0.8.0",
        settings: {
            optimizer: {
                enabled: true,
                runs: 1000
            }
        }
    },
    {
        version: "0.7.1",
        settings: {
            optimizer: {
                enabled: true,
                runs: 1000
            }
        }
    },
    {
        version: "0.7.0",
        settings: {
            optimizer: {
                enabled: true,
                runs: 1000
            }
        }
    },
    {
        version: "0.6.6",
        settings: {
            optimizer: {
                enabled: true,
                runs: 1000
            }
        }
    },
    {
        version: "0.4.11",
        settings: {
            optimizer: {
                enabled: true,
                runs: 1000
            }
        }
    },
    {
        version: "0.4.24",
        settings: {
            optimizer: {
                enabled: true,
                runs: 1000
            }
        }
    }
    ]
}
}""")

print("Compiling contracts...")
subprocess.run('npx hardhat compile', shell=True, check=True)

print("Creating contract go bindings...")
for version in solc_versions:
    for subdir, dirs, files in os.walk(rootdir + version):
        for f in files:
            if ".dbg." not in f:
                print(f)
                compile_contract = open(subdir + "/" + f, "r")
                data = json.load(compile_contract)
                contract_name = data["contractName"]

                abi_name = targetdir + "/" + version + "/abi/" + contract_name + ".abi"
                abi_file = open(abi_name, "w")
                abi_file.write(json.dumps(data["abi"], indent=2))

                bin_name = targetdir + "/" + version + "/bin/" + contract_name + ".bin"
                bin_file = open(bin_name, "w")
                bin_file.write(str(data["bytecode"]))
                abi_file.close()
                bin_file.close()

                if contract_name in used_contract_names:
                    subprocess.run("abigen --bin=" + bin_name + " --abi=" + abi_name + " --pkg=" + contract_name + " --out=" + 
                    targetdir + "/" + contract_name + ".go", shell=True, check=True)
            
print("Cleaning up Hardhat...")
subprocess.run('npm uninstall --save-dev hardhat', shell=True)
if path.exists("hardhat.config.js"):
    os.remove("hardhat.config.js")
if path.exists("package-lock.json"):
    os.remove("package-lock.json")
if path.exists("package.json"):
    os.remove("package.json")
if path.exists("node_modules/"):
    shutil.rmtree("node_modules/")
if path.exists("artifacts/"):
    shutil.rmtree("artifacts/") 
if path.exists("cache/"):
    shutil.rmtree("cache/")

print("Done!")