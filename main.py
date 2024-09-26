import json
import subprocess

def compile_contract(contract_path):
    """Компиляция смарт-контракта."""
    result = subprocess.run(["anchor", "build"], cwd=contract_path, capture_output=True, text=True)
    if result.returncode != 0:
        print("Ошибка компиляции:", result.stderr)
    else:
        print("Контракт успешно скомпилирован:", result.stdout)

def deploy_contract():
    """Деплой контракта на devnet."""
    deploy_process = subprocess.run(["solana", "program", "deploy", "target/deploy/my_contract.so", "--url", "https://api.devnet.solana.com"])
    if deploy_process.returncode == 0:
        print("Контракт успешно задеплоен на devnet!")
    else:
        print("Ошибка при деплое контракта.")

def load_template(template_path):
    """Загрузка шаблона контракта."""
    with open(template_path, "r") as template_file:
        return template_file.read()

def main():
    # Пример JSON данных, полученных от фронтенда
    json_data = '''{
        "contract_type": "update_value",
        "initial_value": 100,
        "program_id": "BPFLoader2111111111111111111111111111111111111"
    }'''
    
    # Конвертируем JSON в Python объект
    data = json.loads(json_data)

    # Определяем путь к шаблону в зависимости от типа контракта
    if data["contract_type"] == "update_value":
        template_path = "templates/update_value.rs"
    elif data["contract_type"] == "transfer_tokens":
        template_path = "templates/transfer.rs"
    else:
        print("Неизвестный тип контракта.")
        return

    # Загружаем шаблон контракта
    contract_template = load_template(template_path)

    # Подставляем данные в шаблон
    contract_code = contract_template.format(
        program_id=data["program_id"]
    )

    # Записываем контракт в нужную папку, чтобы Anchor мог его найти
    with open("contracts/src/lib.rs", "w") as f:
        f.write(contract_code)

    # Компиляция контракта
    compile_contract("path/to/your/anchor/project")

if __name__ == "__main__":
    main()
