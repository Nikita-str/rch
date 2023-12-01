from rest import conn 

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

payload = f"{{\"board_url\":\"{board_url}\", \"post_header\":\"Давайте потестируем маняме борду!!!\", \"post_text\":\"что-нибудь типо такого ~~nyan~~ UwU :3 и даже такого ^w^ хехе\", \"post_imgs\":[]}}"
conn.request("POST", "/api/board/thr_new", payload.encode('utf-8'), headers)
resp(conn)
###


payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"ааа??\\nКотики я не пони+мяу что происходит?!\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"няшки\\nстесняшки\\nкотики\\nне выкупают...\\nжесть блин, кавайно так...\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

post_text = """> hehe\\n > > haha\\n>~< heh\\n>~< > hah\\nwahahah"""

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"{post_text}\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

###

post_text = """>>30\\n>>39\\n>>32>>31\\n>>>~<<\\n>жесть блин, кавайно так...\\ntotaly tuuue!!"""
payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"{post_text}\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

post_text = "cute-kawaii\\ncute-kawaii\\ncute-kawaii\\ncute-kawaii\\ncute-kawaii\\ncute-kawaii\\n>>30\\nуспешно тестируем :3!"
payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"{post_text}\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"UwU ~~nyan~~\\n^w^ :3 <-- это же котик!\\nкавайно вот!\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###

payload = f"{{\"board_url\":\"{board_url}\", \"op_post_n\": {op_post}, \"post_text\":\"~~няв мяв няк~~\\nя все сказал\\n~~nyaaa~~\", \"post_imgs\":[]}}"
conn.request("POST", "/api/thread/post_new", payload.encode('utf-8'), headers)
resp(conn)

###
