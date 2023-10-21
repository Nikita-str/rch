import http.client

conn = http.client.HTTPConnection("127.0.0.1:5173")

headers = {
    'user-agent': "vscode-restclient",
    'content-type': "application/json;charset=UTF-8"
}

payload = "{\"board_url\":\"b\", \"post_header\":\"Выясняем чо происходит\", \"post_text\":\"предлагаю выяснить что всё-же происходит...\"}"


conn.request("POST", "/api/board/thr_new", payload.encode('utf-8'), headers)

res = conn.getresponse()
data = res.read()
print(data.decode("utf-8"))

###

payload = "{\"board_url\":\"b\", \"post_text\":\"А чо кстати заголовок то сам добавиться к посту, а? не знаете? ща узнаем\"}"
conn.request("POST", "/api/board/thr_new", payload.encode('utf-8'), headers)

res = conn.getresponse()
data = res.read()
print(data.decode("utf-8"))

###

payload = "{\"board_url\":\"b\", \"op_post_n\": 244, \"post_text\":\"первый\"}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)

res = conn.getresponse()
data = res.read()

print(data.decode("utf-8"))

###

payload = "{\"board_url\":\"b\", \"op_post_n\": 244, \"post_text\":\"второй\"}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)

res = conn.getresponse()
data = res.read()

print(data.decode("utf-8"))

###

payload = "{\"board_url\":\"b\", \"op_post_n\": 245, \"post_text\":\"хех, там в другом треде расчет дурочков короче идет, ахаха!\"}"

conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
res = conn.getresponse()
data = res.read()

print(data.decode("utf-8"))

###

payload = "{\"board_url\":\"b\", \"op_post_n\": 244, \"post_text\":\"совсем уже обленились!!! третий! да! мне не лень!\"}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)

res = conn.getresponse()
data = res.read()

print(data.decode("utf-8"))

###

for i in range(0, 50): 
    payload = f"{{\"board_url\":\"b\", \"op_post_n\": 244, \"post_text\":\"начинаем дудос: {i}!\"}}"
    conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)

    res = conn.getresponse()
    data = res.read()

    print(data.decode("utf-8"))