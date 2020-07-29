# Protótipo para a validação da senha
# Esse código não é utilizado na aplicação

numberList = "0123456789"

inp = input("password: ")

valid = "Válido"

if len(inp) < 8:
    valid = "Inválido: menor que 8 caracteres"
elif len(inp) > 64:
    valid = "Inválido: maior que 64 caracteres"

numbers = 0

for i in inp:
    if i in numberList:
        numbers += 1

if numbers < 1:
    valid = "Inválido: necessário pelo menos 1 número"

print(valid)
