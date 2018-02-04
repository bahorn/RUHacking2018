import pysodium
import msgpack
import json
import socket

key = [48,252,138,122,143,15,173,85,109,255,161,5,219,192,51,82,250,252,7,230,213,230,48,165,249,178,17,47,187,150,144,0]

realKey = []
for x in key:
    realKey += [chr(x)]
key = "".join(realKey)
message = {
    "message_type":'AnnouncePeer',
    "data":{
        "peers": [
            {
                "public_key": key.encode('hex'), "host":"127.0.0.1", "port":3001
            }
        ]
    }
}
print json.dumps(message)
blah = pysodium.crypto_box_seal(json.dumps(message), key)
print blah.encode('hex')
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM) # UDP
sock.sendto(blah, ('127.0.0.1', 3000))
