from rest import conn 

board_url = "rp"
op_post = 8

headers = {
    'user-agent': "vscode-restclient",
    'content-type': "application/json;charset=UTF-8"
}

def resp(conn):
    res = conn.getresponse()
    data = res.read()
    print(data.decode("utf-8"))


payload = f"{{\"board_url\":\"{board_url}\", \"post_header\":\"Кидаем кубики ([D100])\", \"post_text\":\"[d4][dice6][d8]\\n[d10][D:12]\\n[b]hmm...[/b]\\n[D20]\\n[Dice : 100]\", \"post_imgs\":[]}}"
conn.request("POST", "/api/board/thr_new", payload.encode('utf-8'), headers)
resp(conn)

###

d4x4 = "[d4]" * 4
d4x4n = d4x4 + '\\n'
d4_field = d4x4n * 4
payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"{d4_field}[d3]\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)
