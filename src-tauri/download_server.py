import platform
import os, shutil
import zipfile
import subprocess
import requests

def get_rustc_host() -> str:
    # 运行 rustc -Vv 命令并捕获输出
    result = subprocess.run(['rustc', '-Vv'], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    if result.returncode != 0:
        raise f"Error running rustc: {result.stderr}"

    # 解析输出，查找包含 "host" 的行
    for line in result.stdout.splitlines():
        if 'host' in line:
            # 提取 host 值
            host = line.split(' ')[1]
            return host

    raise "Host information not found in rustc output"

system = platform.system()
match system:
    case "Windows":
        file_name = 'FurLang_server-windows-latest.zip'
        suffix = '.exe'
    case "Linux":
        file_name = 'FurLang_server-ubuntu-latest.zip'
        suffix = '.bin'
    case "Darwin":
        file_name = 'FurLang_server-macos-latest.zip'
        suffix = '.bin'
    case _:
        raise Exception("Unsupported system")

latest_tag = requests.get("https://api.github.com/repos/ovo-Tim/FurLang_server/releases/latest").json()['tag_name']
url = f"https://github.com/ovo-Tim/FurLang_server/releases/download/{latest_tag}/{file_name}"
# os.system(f"curl -LjO {url}")

# with zipfile.ZipFile(file_name, 'r') as zip_ref:
#     zip_ref.extractall('.')

# shutil.move('./build/main.dist', '.')
# os.removedirs('./build')
# os.remove(file_name)
# os.rename('./main.dist', './server')
exe_name = f'main-{get_rustc_host()}{'' if suffix == '.bin' else suffix}'
os.rename(f'./server/main{suffix}', f'./server/{exe_name}')

if system != 'Windows':
    os.system(f'chmod +x ./server/{exe_name}')