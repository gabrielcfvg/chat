# Protótipo para a validação do nome de usuário
# Esse código não é utilizado na aplicação

inp = input("nickname: ")
valid = "Válido"

if len(inp) < 5:
    valid = "Inválido: menor que 5 caracteres"
elif len(inp) > 32:
    valid = "Inválido: maior que 10 caracteres"

print(valid)