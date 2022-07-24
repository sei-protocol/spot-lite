from http.server import BaseHTTPRequestHandler, HTTPServer
import json

hostName = "0.0.0.0"
serverPort = 8088
accountNames = ["admin", "alice", "bob"]

def get_mnemonic(name):
    with open(f'/{name}.mnemonic', 'r') as f:
        return f.readline()

def get_address(name):
    with open(f'/{name}.addr', 'r') as f:
        return f.readline()

class MyServer(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps({name: {"mnemonic": get_mnemonic(name), "address": get_address(name)} for name in accountNames}).encode())

if __name__ == "__main__":
    print("Starting")   
    webServer = HTTPServer((hostName, serverPort), MyServer)
    print("Server started http://%s:%s" % (hostName, serverPort))

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")
