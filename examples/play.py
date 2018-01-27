import sys
from librespot import Session, SpotifyId
import threading
import time

if len(sys.argv) != 4:
  print("Usage: %s USERNAME PASSWORD TRACK" % sys.argv[0])
  sys.exit(1)

totalLen = 0
class Sink:
  def write(self, buf):
    global totalLen
    #print("%s" % repr(buf), end='', flush=True)
    print('.', end='', flush=True)
    totalLen += len(buf)

username = sys.argv[1]
password = sys.argv[2]
trackid = SpotifyId(sys.argv[3])

print("Connecting ...")
session = Session.connect(username, password, Sink()).result()
player = session.player()

print("Playing ...")
handle = player.load(trackid)
handle.add_done_callback(lambda x: print('callback! %s' % x))

time.sleep(5)
print("Stop")
player.stop()
print(handle.result())

print("Done", totalLen)
