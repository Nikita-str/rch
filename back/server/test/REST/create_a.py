import http.client

conn = http.client.HTTPConnection("127.0.0.1:5173")

board_url = "a"
op_post = 30

headers = {
    'user-agent': "vscode-restclient",
    'content-type': "application/json;charset=UTF-8"
}

def resp(conn):
    res = conn.getresponse()
    data = res.read()
    print(data.decode("utf-8"))

payload = f"{{\"board_url\":\"{board_url}\", \"post_header\":\"Давайте потестируем маняме борду!!!\", \"post_text\":\"что-нибудь типо такого ~~nyan~~ UwU :3 и даже такого ^w^ хехе\"}}"
conn.request("POST", "/api/board/thr_new", payload.encode('utf-8'), headers)
resp(conn)
###


payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"ааа??\\nКотики я не пони+мяу что происходит?!\"}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"няшки\\nстесняшки\\nкотики\\nне выкупают...\\nжесть блин, кавайно так...\"}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"cute-kawaii\\ncute-kawaii\\ncute-kawaii\\ncute-kawaii\\ncute-kawaii\\ncute-kawaii\"}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\">~<< UwU ~~nyan~~\\n^w^ :3 <-- это же котик!\\nкавайно вот!\"}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"~~няв мяв няк~~\\nя все сказал\\n~~nyaaa~~\"}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###