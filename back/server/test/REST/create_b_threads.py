import http.client

conn = http.client.HTTPConnection("127.0.0.1:5173")

headers = {
    'user-agent': "vscode-restclient",
    'content-type': "application/json;charset=UTF-8"
}
1
###

for i in range(0, 25):
    ix = str(i + 1)
    ii = ix * (i + 1)
    payload = f"{{\"board_url\":\"b\", \"post_header\":\"Thread#{ix}\", \"post_text\":\"{ix}:{ii}\", \"post_imgs\":[]}}"
    conn.request("POST", "/api/board/thr_new", payload.encode('utf-8'), headers)

    res = conn.getresponse()
    data = res.read()
    print(data.decode("utf-8"))